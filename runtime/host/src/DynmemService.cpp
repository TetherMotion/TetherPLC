/* DynmemService.cpp — tplc-dynmem bounded arena allocator.
 * Simple bump allocator with a free list: each alloc takes the
 * first free slot with capacity >= size (block contents are
 * relocatable on resize via a fresh slot).  Deterministic:
 * 64 blocks max, 1 MiB arena, no host heap after startup.
 * SPDX-License-Identifier: Apache-2.0 OR MPL-2.0 */

#include <tplc/host/DynmemService.hpp>

#include <cstring>
#include <vector>

namespace tplc {

struct DynmemService::Impl {
    struct Block {
        uint32_t off  = 0;      /* arena offset */
        uint32_t size = 0;      /* live size, 0 => free */
        bool     used = false;
    };
    std::vector<uint8_t> arena =
        std::vector<uint8_t>(TPLC_DYNMEM_ARENA_BYTES);
    Block blocks[TPLC_DYNMEM_MAX_BLOCKS]{};
    uint32_t high_water = 0;
    uint32_t live_bytes = 0;
    uint32_t total_allocs = 0;
    uint32_t total_frees = 0;

    Block* blk(uint32_t id) {
        if (id == 0 || id > TPLC_DYNMEM_MAX_BLOCKS) return nullptr;
        Block& b = blocks[id - 1];
        return b.used ? &b : nullptr;
    }
    /* first-fit over the arena high-water region, else bump */
    bool carve(uint32_t size, uint32_t* out_off) {
        /* scan free gaps between used blocks (sorted by off) */
        uint32_t pos = 0;
        for (uint32_t i = 0; i < TPLC_DYNMEM_MAX_BLOCKS; i++) {
            if (!blocks[i].used) continue;
            if (blocks[i].off >= pos + size) { *out_off = pos; return true; }
            if (blocks[i].off + blocks[i].size > pos)
                pos = blocks[i].off + blocks[i].size;
        }
        if (pos + size <= TPLC_DYNMEM_ARENA_BYTES) {
            *out_off = pos;
            if (pos + size > high_water) high_water = pos + size;
            return true;
        }
        return false;
    }
};

DynmemService::DynmemService() : impl_(new Impl) {
    svc_.struct_size = sizeof(TplcDynmemService);
    svc_.abi_version = TPLC_DYNMEM_ABI_VERSION;
    svc_.host = this;
    svc_.dynmem_alloc = [](void* s, uint32_t n, uint32_t r)
        { return self(s)->alloc(s, n, r); };
    svc_.dynmem_free = [](void* s, uint32_t id)
        { return self(s)->free(s, id); };
    svc_.dynmem_resize = [](void* s, uint32_t id, uint32_t n,
                            uint32_t p, uint32_t r)
        { return self(s)->resize(s, id, n, p, r); };
    svc_.dynmem_read = [](void* s, uint32_t id, uint32_t off)
        { return self(s)->read(s, id, off); };
    svc_.dynmem_write = [](void* s, uint32_t id, uint32_t off,
                           uint32_t b)
        { return self(s)->write(s, id, off, b); };
    svc_.dynmem_stats = [](void* s, TplcDynmemStats* o)
        { return self(s)->stats(s, o); };
}
DynmemService::~DynmemService() = default;

int32_t DynmemService::alloc(void*, uint32_t size, uint32_t reset) {
    if (size == 0 || size > TPLC_DYNMEM_MAX_BLOCK)
        return TPLC_DYNMEM_E_PARAM;
    int slot = -1;
    for (uint32_t i = 0; i < TPLC_DYNMEM_MAX_BLOCKS; i++)
        if (!impl_->blocks[i].used) { slot = (int)i; break; }
    if (slot < 0) return TPLC_DYNMEM_E_FULL;
    uint32_t off = 0;
    if (!impl_->carve(size, &off)) return TPLC_DYNMEM_E_FULL;
    Impl::Block& b = impl_->blocks[slot];
    b.off = off; b.size = size; b.used = true;
    if (reset)
        std::memset(impl_->arena.data() + off, 0, size);
    impl_->live_bytes += size;
    impl_->total_allocs++;
    return (int32_t)slot + 1;
}

int32_t DynmemService::free(void*, uint32_t id) {
    Impl::Block* b = impl_->blk(id);
    if (!b) return TPLC_DYNMEM_E_HANDLE;
    impl_->live_bytes -= b->size;
    impl_->total_frees++;
    b->used = false; b->size = 0; b->off = 0;
    return TPLC_DYNMEM_OK;
}

int32_t DynmemService::resize(void*, uint32_t id, uint32_t new_size,
                              uint32_t preserve, uint32_t reset) {
    Impl::Block* b = impl_->blk(id);
    if (!b || new_size == 0 || new_size > TPLC_DYNMEM_MAX_BLOCK)
        return TPLC_DYNMEM_E_PARAM;
    if (new_size <= b->size) {
        impl_->live_bytes -= b->size - new_size;
        b->size = new_size;
        if (reset)
            std::memset(impl_->arena.data() + b->off, 0, new_size);
        return TPLC_DYNMEM_OK;
    }
    /* grow: try in-place if the space after the block is free */
    uint32_t end = b->off + new_size;
    bool free_tail = true;
    for (uint32_t i = 0; i < TPLC_DYNMEM_MAX_BLOCKS; i++) {
        Impl::Block& o = impl_->blocks[i];
        if (!o.used || &o == b) continue;
        if (o.off >= b->off && o.off < end) { free_tail = false; break; }
    }
    if (free_tail && end <= TPLC_DYNMEM_ARENA_BYTES) {
        uint8_t* dst = impl_->arena.data() + b->off;
        if (reset)
            std::memset(dst, 0, new_size);
        else if (!preserve)
            std::memset(dst + b->size, 0, new_size - b->size);
        impl_->live_bytes += new_size - b->size;
        if (end > impl_->high_water) impl_->high_water = end;
        b->size = new_size;
        return TPLC_DYNMEM_OK;
    }
    /* move to a fresh slot */
    uint32_t off = 0;
    if (!impl_->carve(new_size, &off)) return TPLC_DYNMEM_E_FULL;
    uint8_t* a = impl_->arena.data();
    if (preserve && !reset)
        std::memmove(a + off, a + b->off, b->size);
    if (reset) {
        std::memset(a + off, 0, new_size);
        if (preserve)
            std::memmove(a + off, a + b->off,
                         b->size < new_size ? b->size : new_size);
    }
    impl_->live_bytes += new_size - b->size;
    b->off = off; b->size = new_size;
    return TPLC_DYNMEM_OK;
}

int32_t DynmemService::read(void*, uint32_t id, uint32_t offset) {
    Impl::Block* b = impl_->blk(id);
    if (!b) return TPLC_DYNMEM_E_HANDLE;
    if (offset >= b->size) return TPLC_DYNMEM_E_RANGE;
    return impl_->arena[b->off + offset];
}

int32_t DynmemService::write(void*, uint32_t id, uint32_t offset,
                             uint32_t byte) {
    Impl::Block* b = impl_->blk(id);
    if (!b) return TPLC_DYNMEM_E_HANDLE;
    if (offset >= b->size || byte > 255) return TPLC_DYNMEM_E_RANGE;
    impl_->arena[b->off + offset] = (uint8_t)byte;
    return TPLC_DYNMEM_OK;
}

int32_t DynmemService::stats(void*, TplcDynmemStats* out) {
    if (!out) return TPLC_DYNMEM_E_PARAM;
    std::memset(out, 0, sizeof(*out));
    for (uint32_t i = 0; i < TPLC_DYNMEM_MAX_BLOCKS; i++)
        if (impl_->blocks[i].used) out->live_blocks++;
    out->live_bytes   = impl_->live_bytes;
    out->total_allocs = impl_->total_allocs;
    out->total_frees  = impl_->total_frees;
    out->arena_bytes  = TPLC_DYNMEM_ARENA_BYTES;
    out->arena_used   = impl_->high_water;
    return TPLC_DYNMEM_OK;
}

} /* namespace tplc */

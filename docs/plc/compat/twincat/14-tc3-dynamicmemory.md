# Tc3_DynamicMemory

- **Manual:** `TwinCAT_3_PLC_Lib_Tc3_DynamicMemory_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Standard · **Tier:** T1 · **API entries:** 2
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Dynamic allocation needs a heap/arena service in the runtime; deterministic sizing rules required.

---

#### Function blocks

##### `FB_DynMem_Buffer` (METHOD) → `BOOL` — §3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nSize` | `UDINT;    // buffer size [in bytes]` | — |
| `bReset` | `BOOL;     // zero the allocated memory` | — |
| `nSize` | `UDINT;        // new buffer size [in bytes]` | — |
| `bPreserve` | `BOOL;         // TRUE => preserve old content, FALSE=> don't preserve old content` | — |
| `bReset` | `BOOL;         // zero the allocated memory (before preserving)` | — |

**Description:** This function block provides a buffer of dynamically allocated memory. This buffer can be used for individual data or data blocks. It cannot be used for dynamic instantiation of function blocks.

##### `FB_DynMem_Manager` (METHOD) → `PVOID` — §3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nSize` | `UDINT;       // requested size in bytes` | — |
| `bReset` | `BOOL;        // zero the allocated memory` | — |
| `p` | `REFERENCE TO PVOID; // the given pointer is reset to zero after deletion` | — |
| `nSize` | `UDINT` | — |
| `nSize` | `UDINT;      // requested size in bytes` | — |
| `bReset` | `BOOL;       // zero the allocated memory` | — |
| `p` | `REFERENCE TO PVOID; // the given pointer is reset to zero after deletion` | — |

**Description:** This function block provides the possibility to dynamically allocate memory and to release it again. Integrated counters indicate the memory consumption and thus enable an overview of the already allocated memory.

#pragma once

// Native-value bridge for the V6 schema wire format.
//
// Tether's committed io API exposes decodeSchemaValue()/
// encodeSchemaValue() over SchemaValue trees. This header adapts the
// small native-value surface tplc-io needs on top of it: a variant
// for C++ scalars/strings/bytes, nv:: constructors, encodeNative()
// (native -> wire via a SchemaValue), and asU64()/asString() accessors
// for decoded values.

#include "tether/io/SchemaValue.hpp"

#include <cstring>
#include <expected>
#include <optional>
#include <span>
#include <string>
#include <variant>
#include <vector>

namespace tether::io {

/// Native C++ value encoded against a schema node. Arithmetic types
/// cover every scalar ValueType; String/Bytes payloads cover the
/// SchemaKind::String and SchemaKind::Bytes nodes.
using NativeValue = std::variant<uint64_t, int64_t, double, bool,
                                  std::string, std::vector<uint8_t>>;

namespace nv {

inline NativeValue u(uint64_t v) { return v; }
inline NativeValue i(int64_t v) { return v; }
inline NativeValue f(double v) { return v; }
inline NativeValue boolean(bool v) { return v; }
inline NativeValue str(std::string v) { return NativeValue{std::move(v)}; }
inline NativeValue str(const char* v) { return NativeValue{std::string(v)}; }
inline NativeValue str(std::string_view v) {
    return NativeValue{std::string(v)};
}
inline NativeValue bytes(std::span<const uint8_t> v) {
    return NativeValue{std::vector<uint8_t>(v.begin(), v.end())};
}
inline NativeValue bytes(std::vector<uint8_t> v) {
    return NativeValue{std::move(v)};
}

} // namespace nv

namespace schema_value_detail {

/// NativeValue -> SchemaValue::Scalar bits, per the codec convention:
/// unsigned stored directly, signed sign-extended to 64 bits, F32/F64
/// as raw IEEE bit patterns in the low bits.
inline std::expected<SchemaValue::Scalar, std::string>
nativeToScalar(const NativeValue& nv, const SchemaNode& node) {
    SchemaValue::Scalar s;
    std::visit(
        [&](const auto& x) {
            using T = std::decay_t<decltype(x)>;
            if constexpr (std::is_same_v<T, uint64_t>) {
                s.bits = x;
            } else if constexpr (std::is_same_v<T, int64_t>) {
                s.bits = static_cast<uint64_t>(x);
            } else if constexpr (std::is_same_v<T, double>) {
                if (node.scalarType == ValueType::F32) {
                    const float f = static_cast<float>(x);
                    uint32_t b;
                    std::memcpy(&b, &f, sizeof b);
                    s.bits = b;
                } else {
                    uint64_t b;
                    std::memcpy(&b, &x, sizeof b);
                    s.bits = b;
                }
            } else if constexpr (std::is_same_v<T, bool>) {
                s.bits = x ? 1 : 0;
            }
        },
        nv);
    return s;
}

} // namespace schema_value_detail

/// Encode a native value against a schema node into canonical wire
/// bytes. Scalar/Enum nodes take arithmetic natives; String and Bytes
/// nodes take the matching payload kind. Other kinds are rejected.
inline std::expected<std::vector<uint8_t>, std::string>
encodeNative(const SchemaGraph& graph, const SchemaNode& node,
             const NativeValue& nv) {
    SchemaValue sv;
    sv.node = &node;
    switch (node.kind) {
        case SchemaKind::Scalar:
        case SchemaKind::Enum: {
            if (std::holds_alternative<std::string>(nv) ||
                std::holds_alternative<std::vector<uint8_t>>(nv)) {
                return std::unexpected(
                    std::string("payload kind does not match scalar schema"));
            }
            auto s = schema_value_detail::nativeToScalar(nv, node);
            if (!s) return std::unexpected(s.error());
            sv.payload = *s;
            break;
        }
        case SchemaKind::String: {
            const auto* s = std::get_if<std::string>(&nv);
            if (!s) {
                return std::unexpected(
                    std::string("payload kind does not match string schema"));
            }
            sv.payload = SchemaValue::Str{*s};
            break;
        }
        case SchemaKind::Bytes: {
            const auto* b = std::get_if<std::vector<uint8_t>>(&nv);
            if (!b) {
                return std::unexpected(
                    std::string("payload kind does not match bytes schema"));
            }
            sv.payload = SchemaValue::Bytes{*b};
            break;
        }
        default:
            return std::unexpected(
                std::string("unsupported schema kind for native encode"));
    }
    return encodeSchemaValue(graph, sv);
}

/// Scalar payload -> raw bits (u64 view). Signed scalars arrive
/// sign-extended; callers decoding a specific ValueType know the
/// width and truncate as needed.
inline std::optional<uint64_t> asU64(const SchemaValue& v) {
    if (const auto* s = std::get_if<SchemaValue::Scalar>(&v.payload)) {
        return s->bits;
    }
    return std::nullopt;
}

inline std::optional<std::string> asString(const SchemaValue& v) {
    if (const auto* s = std::get_if<SchemaValue::Str>(&v.payload)) {
        return s->value;
    }
    return std::nullopt;
}

} // namespace tether::io

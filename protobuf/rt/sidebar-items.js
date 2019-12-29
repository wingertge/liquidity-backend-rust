initSidebarItems({"fn":[["bytes_size","Size of encoded bytes field."],["compute_map_size","Compute serialized size of `map` field and cache nested field sizes."],["compute_raw_varint32_size","Given `u32` value compute varint encoded length."],["compute_raw_varint64_size","Given `u64` value compute varint encoded length."],["enum_size","Size of encoded enum field value."],["read_map_into","Read `map` field."],["read_proto2_enum_with_unknown_fields_into","Read repeated `enum` field into given vec, and when value is unknown store it in unknown fields which matches proto2 spec."],["read_proto3_enum_with_unknown_fields_into","Read repeated `enum` field into given vec, and when value is unknown store it in unknown fields which matches proto2 spec."],["read_repeated_bool_into","Read repeated `bool` field into given vec."],["read_repeated_bytes_into","Read repeated `bytes` field into given vec."],["read_repeated_carllerche_bytes_into","Read repeated `Bytes` field into given vec."],["read_repeated_carllerche_string_into","Read repeated `Chars` field into given vec."],["read_repeated_double_into","Read repeated `double` field into given vec."],["read_repeated_enum_into","Read repeated `enum` field into given vec. This function is no longer called from generated code, remove in 1.5."],["read_repeated_enum_with_unknown_fields_into","Read repeated `enum` field into given vec, and when value is unknown store it in unknown fields which matches proto2 spec."],["read_repeated_fixed32_into","Read repeated `fixed32` field into given vec."],["read_repeated_fixed64_into","Read repeated `fixed64` field into given vec."],["read_repeated_float_into","Read repeated `float` field into given vec."],["read_repeated_int32_into","Read repeated `int32` field into given vec."],["read_repeated_int64_into","Read repeated `int64` field into given vec."],["read_repeated_message_into","Read repeated `message` field."],["read_repeated_sfixed32_into","Read repeated `sfixed32` field into given vec."],["read_repeated_sfixed64_into","Read repeated `sfixed64` field into given vec."],["read_repeated_sint32_into","Read repeated `sint32` field into given vec."],["read_repeated_sint64_into","Read repeated `sint64` field into given vec."],["read_repeated_string_into","Read repeated `string` field into given vec."],["read_repeated_uint32_into","Read repeated `uint32` field into given vec."],["read_repeated_uint64_into","Read repeated `uint64` field into given vec."],["read_singular_bytes_into","Read singular `bytes` field."],["read_singular_carllerche_bytes_into","Read singular `Bytes` field."],["read_singular_carllerche_string_into","Read singular `Chars` field."],["read_singular_message_into","Read singular `message` field."],["read_singular_proto3_bytes_into","Read singular `bytes` field for proto3."],["read_singular_proto3_carllerche_bytes_into","Read singular `Bytes` field for proto3."],["read_singular_proto3_carllerche_string_into","Read singular `Chars` field for proto3."],["read_singular_proto3_string_into","Read singular `string` field for proto3."],["read_singular_string_into","Read singular `string` field."],["read_unknown_or_skip_group","Handle unknown field in generated code. Either store a value in unknown, or skip a group."],["string_size","Size of encoded string field."],["tag_size","Compute tag size. Size of tag does not depend on wire type."],["unexpected_wire_type","Create an error for unexpected wire type."],["unknown_fields_size","Size of encoded unknown fields size."],["value_size","Integer value size when encoded as specified wire type."],["value_varint_zigzag_size","Length of value when encoding with zigzag encoding with tag"],["value_varint_zigzag_size_no_tag","Integer value size when encoded as specified wire type."],["vec_packed_enum_data_size","Size of serialized repeated packed enum field, excluding length and tag."],["vec_packed_enum_size","Size of serialized data with length prefix and tag"],["vec_packed_varint_data_size","Size of serialized repeated packed field, excluding length and tag."],["vec_packed_varint_size","Size of serialized data with length prefix and tag"],["vec_packed_varint_zigzag_data_size","Size of serialized repeated packed field, excluding length and tag."],["vec_packed_varint_zigzag_size","Size of serialized data with length prefix and tag"],["write_map_with_cached_sizes","Write map, message sizes must be already known."]],"trait":[["ProtobufVarint","Helper trait implemented by integer types which could be encoded as varint."],["ProtobufVarintZigzag","Helper trait implemented by integer types which could be encoded as zigzag varint."]]});
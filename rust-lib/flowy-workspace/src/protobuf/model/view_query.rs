// This file is generated by rust-protobuf 2.22.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `view_query.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct QueryViewRequest {
    // message fields
    pub view_id: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a QueryViewRequest {
    fn default() -> &'a QueryViewRequest {
        <QueryViewRequest as ::protobuf::Message>::default_instance()
    }
}

impl QueryViewRequest {
    pub fn new() -> QueryViewRequest {
        ::std::default::Default::default()
    }

    // string view_id = 1;


    pub fn get_view_id(&self) -> &str {
        &self.view_id
    }
    pub fn clear_view_id(&mut self) {
        self.view_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_view_id(&mut self, v: ::std::string::String) {
        self.view_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_view_id(&mut self) -> &mut ::std::string::String {
        &mut self.view_id
    }

    // Take field
    pub fn take_view_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.view_id, ::std::string::String::new())
    }
}

impl ::protobuf::Message for QueryViewRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.view_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.view_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.view_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.view_id.is_empty() {
            os.write_string(1, &self.view_id)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> QueryViewRequest {
        QueryViewRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "view_id",
                |m: &QueryViewRequest| { &m.view_id },
                |m: &mut QueryViewRequest| { &mut m.view_id },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<QueryViewRequest>(
                "QueryViewRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static QueryViewRequest {
        static instance: ::protobuf::rt::LazyV2<QueryViewRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(QueryViewRequest::new)
    }
}

impl ::protobuf::Clear for QueryViewRequest {
    fn clear(&mut self) {
        self.view_id.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueryViewRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryViewRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct QueryViewParams {
    // message fields
    pub view_id: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a QueryViewParams {
    fn default() -> &'a QueryViewParams {
        <QueryViewParams as ::protobuf::Message>::default_instance()
    }
}

impl QueryViewParams {
    pub fn new() -> QueryViewParams {
        ::std::default::Default::default()
    }

    // string view_id = 1;


    pub fn get_view_id(&self) -> &str {
        &self.view_id
    }
    pub fn clear_view_id(&mut self) {
        self.view_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_view_id(&mut self, v: ::std::string::String) {
        self.view_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_view_id(&mut self) -> &mut ::std::string::String {
        &mut self.view_id
    }

    // Take field
    pub fn take_view_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.view_id, ::std::string::String::new())
    }
}

impl ::protobuf::Message for QueryViewParams {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.view_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.view_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.view_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.view_id.is_empty() {
            os.write_string(1, &self.view_id)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> QueryViewParams {
        QueryViewParams::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "view_id",
                |m: &QueryViewParams| { &m.view_id },
                |m: &mut QueryViewParams| { &mut m.view_id },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<QueryViewParams>(
                "QueryViewParams",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static QueryViewParams {
        static instance: ::protobuf::rt::LazyV2<QueryViewParams> = ::protobuf::rt::LazyV2::INIT;
        instance.get(QueryViewParams::new)
    }
}

impl ::protobuf::Clear for QueryViewParams {
    fn clear(&mut self) {
        self.view_id.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueryViewParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryViewParams {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpenViewRequest {
    // message fields
    pub view_id: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a OpenViewRequest {
    fn default() -> &'a OpenViewRequest {
        <OpenViewRequest as ::protobuf::Message>::default_instance()
    }
}

impl OpenViewRequest {
    pub fn new() -> OpenViewRequest {
        ::std::default::Default::default()
    }

    // string view_id = 1;


    pub fn get_view_id(&self) -> &str {
        &self.view_id
    }
    pub fn clear_view_id(&mut self) {
        self.view_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_view_id(&mut self, v: ::std::string::String) {
        self.view_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_view_id(&mut self) -> &mut ::std::string::String {
        &mut self.view_id
    }

    // Take field
    pub fn take_view_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.view_id, ::std::string::String::new())
    }
}

impl ::protobuf::Message for OpenViewRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.view_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.view_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.view_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.view_id.is_empty() {
            os.write_string(1, &self.view_id)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> OpenViewRequest {
        OpenViewRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "view_id",
                |m: &OpenViewRequest| { &m.view_id },
                |m: &mut OpenViewRequest| { &mut m.view_id },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<OpenViewRequest>(
                "OpenViewRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static OpenViewRequest {
        static instance: ::protobuf::rt::LazyV2<OpenViewRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(OpenViewRequest::new)
    }
}

impl ::protobuf::Clear for OpenViewRequest {
    fn clear(&mut self) {
        self.view_id.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpenViewRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpenViewRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10view_query.proto\"+\n\x10QueryViewRequest\x12\x17\n\x07view_id\x18\
    \x01\x20\x01(\tR\x06viewId\"*\n\x0fQueryViewParams\x12\x17\n\x07view_id\
    \x18\x01\x20\x01(\tR\x06viewId\"*\n\x0fOpenViewRequest\x12\x17\n\x07view\
    _id\x18\x01\x20\x01(\tR\x06viewIdJ\xff\x01\n\x06\x12\x04\0\0\n\x01\n\x08\
    \n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x04\x01\n\n\n\
    \x03\x04\0\x01\x12\x03\x02\x08\x18\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\
    \x04\x17\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x03\x04\n\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03\x03\x0b\x12\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\
    \x15\x16\n\n\n\x02\x04\x01\x12\x04\x05\0\x07\x01\n\n\n\x03\x04\x01\x01\
    \x12\x03\x05\x08\x17\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x06\x04\x17\n\x0c\
    \n\x05\x04\x01\x02\0\x05\x12\x03\x06\x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\
    \x12\x03\x06\x0b\x12\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x06\x15\x16\n\
    \n\n\x02\x04\x02\x12\x04\x08\0\n\x01\n\n\n\x03\x04\x02\x01\x12\x03\x08\
    \x08\x17\n\x0b\n\x04\x04\x02\x02\0\x12\x03\t\x04\x17\n\x0c\n\x05\x04\x02\
    \x02\0\x05\x12\x03\t\x04\n\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\t\x0b\
    \x12\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\t\x15\x16b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}

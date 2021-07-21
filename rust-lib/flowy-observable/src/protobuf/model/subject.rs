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
//! Generated file from `subject.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct ObservableSubject {
    // message fields
    pub category: ::std::string::String,
    pub ty: i32,
    pub subject_id: ::std::string::String,
    // message oneof groups
    pub one_of_subject_payload: ::std::option::Option<ObservableSubject_oneof_one_of_subject_payload>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ObservableSubject {
    fn default() -> &'a ObservableSubject {
        <ObservableSubject as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum ObservableSubject_oneof_one_of_subject_payload {
    subject_payload(::std::vec::Vec<u8>),
}

impl ObservableSubject {
    pub fn new() -> ObservableSubject {
        ::std::default::Default::default()
    }

    // string category = 1;


    pub fn get_category(&self) -> &str {
        &self.category
    }
    pub fn clear_category(&mut self) {
        self.category.clear();
    }

    // Param is passed by value, moved
    pub fn set_category(&mut self, v: ::std::string::String) {
        self.category = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_category(&mut self) -> &mut ::std::string::String {
        &mut self.category
    }

    // Take field
    pub fn take_category(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.category, ::std::string::String::new())
    }

    // int32 ty = 2;


    pub fn get_ty(&self) -> i32 {
        self.ty
    }
    pub fn clear_ty(&mut self) {
        self.ty = 0;
    }

    // Param is passed by value, moved
    pub fn set_ty(&mut self, v: i32) {
        self.ty = v;
    }

    // string subject_id = 3;


    pub fn get_subject_id(&self) -> &str {
        &self.subject_id
    }
    pub fn clear_subject_id(&mut self) {
        self.subject_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_subject_id(&mut self, v: ::std::string::String) {
        self.subject_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subject_id(&mut self) -> &mut ::std::string::String {
        &mut self.subject_id
    }

    // Take field
    pub fn take_subject_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.subject_id, ::std::string::String::new())
    }

    // bytes subject_payload = 4;


    pub fn get_subject_payload(&self) -> &[u8] {
        match self.one_of_subject_payload {
            ::std::option::Option::Some(ObservableSubject_oneof_one_of_subject_payload::subject_payload(ref v)) => v,
            _ => &[],
        }
    }
    pub fn clear_subject_payload(&mut self) {
        self.one_of_subject_payload = ::std::option::Option::None;
    }

    pub fn has_subject_payload(&self) -> bool {
        match self.one_of_subject_payload {
            ::std::option::Option::Some(ObservableSubject_oneof_one_of_subject_payload::subject_payload(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_subject_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.one_of_subject_payload = ::std::option::Option::Some(ObservableSubject_oneof_one_of_subject_payload::subject_payload(v))
    }

    // Mutable pointer to the field.
    pub fn mut_subject_payload(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(ObservableSubject_oneof_one_of_subject_payload::subject_payload(_)) = self.one_of_subject_payload {
        } else {
            self.one_of_subject_payload = ::std::option::Option::Some(ObservableSubject_oneof_one_of_subject_payload::subject_payload(::std::vec::Vec::new()));
        }
        match self.one_of_subject_payload {
            ::std::option::Option::Some(ObservableSubject_oneof_one_of_subject_payload::subject_payload(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_subject_payload(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_subject_payload() {
            match self.one_of_subject_payload.take() {
                ::std::option::Option::Some(ObservableSubject_oneof_one_of_subject_payload::subject_payload(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }
}

impl ::protobuf::Message for ObservableSubject {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.category)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.ty = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.subject_id)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_subject_payload = ::std::option::Option::Some(ObservableSubject_oneof_one_of_subject_payload::subject_payload(is.read_bytes()?));
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
        if !self.category.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.category);
        }
        if self.ty != 0 {
            my_size += ::protobuf::rt::value_size(2, self.ty, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.subject_id.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.subject_id);
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_subject_payload {
            match v {
                &ObservableSubject_oneof_one_of_subject_payload::subject_payload(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(4, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.category.is_empty() {
            os.write_string(1, &self.category)?;
        }
        if self.ty != 0 {
            os.write_int32(2, self.ty)?;
        }
        if !self.subject_id.is_empty() {
            os.write_string(3, &self.subject_id)?;
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_subject_payload {
            match v {
                &ObservableSubject_oneof_one_of_subject_payload::subject_payload(ref v) => {
                    os.write_bytes(4, v)?;
                },
            };
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

    fn new() -> ObservableSubject {
        ObservableSubject::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "category",
                |m: &ObservableSubject| { &m.category },
                |m: &mut ObservableSubject| { &mut m.category },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "ty",
                |m: &ObservableSubject| { &m.ty },
                |m: &mut ObservableSubject| { &mut m.ty },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "subject_id",
                |m: &ObservableSubject| { &m.subject_id },
                |m: &mut ObservableSubject| { &mut m.subject_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                "subject_payload",
                ObservableSubject::has_subject_payload,
                ObservableSubject::get_subject_payload,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ObservableSubject>(
                "ObservableSubject",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ObservableSubject {
        static instance: ::protobuf::rt::LazyV2<ObservableSubject> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ObservableSubject::new)
    }
}

impl ::protobuf::Clear for ObservableSubject {
    fn clear(&mut self) {
        self.category.clear();
        self.ty = 0;
        self.subject_id.clear();
        self.one_of_subject_payload = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ObservableSubject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ObservableSubject {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rsubject.proto\"\xa3\x01\n\x11ObservableSubject\x12\x1a\n\x08category\
    \x18\x01\x20\x01(\tR\x08category\x12\x0e\n\x02ty\x18\x02\x20\x01(\x05R\
    \x02ty\x12\x1d\n\nsubject_id\x18\x03\x20\x01(\tR\tsubjectId\x12)\n\x0fsu\
    bject_payload\x18\x04\x20\x01(\x0cH\0R\x0esubjectPayloadB\x18\n\x16one_o\
    f_subject_payloadJ\xa1\x02\n\x06\x12\x04\0\0\x07\x01\n\x08\n\x01\x0c\x12\
    \x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x07\x01\n\n\n\x03\x04\0\x01\
    \x12\x03\x02\x08\x19\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\x04\x18\n\x0c\n\
    \x05\x04\0\x02\0\x05\x12\x03\x03\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\x03\x0b\x13\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x16\x17\n\x0b\n\
    \x04\x04\0\x02\x01\x12\x03\x04\x04\x11\n\x0c\n\x05\x04\0\x02\x01\x05\x12\
    \x03\x04\x04\t\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x04\n\x0c\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03\x04\x0f\x10\n\x0b\n\x04\x04\0\x02\x02\x12\
    \x03\x05\x04\x1a\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x05\x04\n\n\x0c\n\
    \x05\x04\0\x02\x02\x01\x12\x03\x05\x0b\x15\n\x0c\n\x05\x04\0\x02\x02\x03\
    \x12\x03\x05\x18\x19\n\x0b\n\x04\x04\0\x08\0\x12\x03\x06\x04?\n\x0c\n\
    \x05\x04\0\x08\0\x01\x12\x03\x06\n\x20\n\x0b\n\x04\x04\0\x02\x03\x12\x03\
    \x06#=\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x06#(\n\x0c\n\x05\x04\0\x02\
    \x03\x01\x12\x03\x06)8\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x06;<b\x06p\
    roto3\
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

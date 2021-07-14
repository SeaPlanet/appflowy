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
//! Generated file from `user_update.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct UpdateUserRequest {
    // message fields
    pub id: ::std::string::String,
    // message oneof groups
    pub one_of_name: ::std::option::Option<UpdateUserRequest_oneof_one_of_name>,
    pub one_of_email: ::std::option::Option<UpdateUserRequest_oneof_one_of_email>,
    pub one_of_workspace: ::std::option::Option<UpdateUserRequest_oneof_one_of_workspace>,
    pub one_of_password: ::std::option::Option<UpdateUserRequest_oneof_one_of_password>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UpdateUserRequest {
    fn default() -> &'a UpdateUserRequest {
        <UpdateUserRequest as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum UpdateUserRequest_oneof_one_of_name {
    name(::std::string::String),
}

#[derive(Clone,PartialEq,Debug)]
pub enum UpdateUserRequest_oneof_one_of_email {
    email(::std::string::String),
}

#[derive(Clone,PartialEq,Debug)]
pub enum UpdateUserRequest_oneof_one_of_workspace {
    workspace(::std::string::String),
}

#[derive(Clone,PartialEq,Debug)]
pub enum UpdateUserRequest_oneof_one_of_password {
    password(::std::string::String),
}

impl UpdateUserRequest {
    pub fn new() -> UpdateUserRequest {
        ::std::default::Default::default()
    }

    // string id = 1;


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    // string name = 2;


    pub fn get_name(&self) -> &str {
        match self.one_of_name {
            ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_name::name(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_name(&mut self) {
        self.one_of_name = ::std::option::Option::None;
    }

    pub fn has_name(&self) -> bool {
        match self.one_of_name {
            ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_name::name(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.one_of_name = ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_name::name(v))
    }

    // Mutable pointer to the field.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_name::name(_)) = self.one_of_name {
        } else {
            self.one_of_name = ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_name::name(::std::string::String::new()));
        }
        match self.one_of_name {
            ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_name::name(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        if self.has_name() {
            match self.one_of_name.take() {
                ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_name::name(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // string email = 3;


    pub fn get_email(&self) -> &str {
        match self.one_of_email {
            ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_email::email(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_email(&mut self) {
        self.one_of_email = ::std::option::Option::None;
    }

    pub fn has_email(&self) -> bool {
        match self.one_of_email {
            ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_email::email(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::std::string::String) {
        self.one_of_email = ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_email::email(v))
    }

    // Mutable pointer to the field.
    pub fn mut_email(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_email::email(_)) = self.one_of_email {
        } else {
            self.one_of_email = ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_email::email(::std::string::String::new()));
        }
        match self.one_of_email {
            ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_email::email(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_email(&mut self) -> ::std::string::String {
        if self.has_email() {
            match self.one_of_email.take() {
                ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_email::email(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // string workspace = 4;


    pub fn get_workspace(&self) -> &str {
        match self.one_of_workspace {
            ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_workspace::workspace(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_workspace(&mut self) {
        self.one_of_workspace = ::std::option::Option::None;
    }

    pub fn has_workspace(&self) -> bool {
        match self.one_of_workspace {
            ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_workspace::workspace(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_workspace(&mut self, v: ::std::string::String) {
        self.one_of_workspace = ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_workspace::workspace(v))
    }

    // Mutable pointer to the field.
    pub fn mut_workspace(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_workspace::workspace(_)) = self.one_of_workspace {
        } else {
            self.one_of_workspace = ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_workspace::workspace(::std::string::String::new()));
        }
        match self.one_of_workspace {
            ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_workspace::workspace(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_workspace(&mut self) -> ::std::string::String {
        if self.has_workspace() {
            match self.one_of_workspace.take() {
                ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_workspace::workspace(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // string password = 5;


    pub fn get_password(&self) -> &str {
        match self.one_of_password {
            ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_password::password(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_password(&mut self) {
        self.one_of_password = ::std::option::Option::None;
    }

    pub fn has_password(&self) -> bool {
        match self.one_of_password {
            ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_password::password(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::string::String) {
        self.one_of_password = ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_password::password(v))
    }

    // Mutable pointer to the field.
    pub fn mut_password(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_password::password(_)) = self.one_of_password {
        } else {
            self.one_of_password = ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_password::password(::std::string::String::new()));
        }
        match self.one_of_password {
            ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_password::password(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::string::String {
        if self.has_password() {
            match self.one_of_password.take() {
                ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_password::password(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }
}

impl ::protobuf::Message for UpdateUserRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_name = ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_name::name(is.read_string()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_email = ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_email::email(is.read_string()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_workspace = ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_workspace::workspace(is.read_string()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.one_of_password = ::std::option::Option::Some(UpdateUserRequest_oneof_one_of_password::password(is.read_string()?));
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_name {
            match v {
                &UpdateUserRequest_oneof_one_of_name::name(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_email {
            match v {
                &UpdateUserRequest_oneof_one_of_email::email(ref v) => {
                    my_size += ::protobuf::rt::string_size(3, &v);
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_workspace {
            match v {
                &UpdateUserRequest_oneof_one_of_workspace::workspace(ref v) => {
                    my_size += ::protobuf::rt::string_size(4, &v);
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_password {
            match v {
                &UpdateUserRequest_oneof_one_of_password::password(ref v) => {
                    my_size += ::protobuf::rt::string_size(5, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_name {
            match v {
                &UpdateUserRequest_oneof_one_of_name::name(ref v) => {
                    os.write_string(2, v)?;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_email {
            match v {
                &UpdateUserRequest_oneof_one_of_email::email(ref v) => {
                    os.write_string(3, v)?;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_workspace {
            match v {
                &UpdateUserRequest_oneof_one_of_workspace::workspace(ref v) => {
                    os.write_string(4, v)?;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.one_of_password {
            match v {
                &UpdateUserRequest_oneof_one_of_password::password(ref v) => {
                    os.write_string(5, v)?;
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

    fn new() -> UpdateUserRequest {
        UpdateUserRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "id",
                |m: &UpdateUserRequest| { &m.id },
                |m: &mut UpdateUserRequest| { &mut m.id },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "name",
                UpdateUserRequest::has_name,
                UpdateUserRequest::get_name,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "email",
                UpdateUserRequest::has_email,
                UpdateUserRequest::get_email,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "workspace",
                UpdateUserRequest::has_workspace,
                UpdateUserRequest::get_workspace,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "password",
                UpdateUserRequest::has_password,
                UpdateUserRequest::get_password,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<UpdateUserRequest>(
                "UpdateUserRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static UpdateUserRequest {
        static instance: ::protobuf::rt::LazyV2<UpdateUserRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(UpdateUserRequest::new)
    }
}

impl ::protobuf::Clear for UpdateUserRequest {
    fn clear(&mut self) {
        self.id.clear();
        self.one_of_name = ::std::option::Option::None;
        self.one_of_email = ::std::option::Option::None;
        self.one_of_workspace = ::std::option::Option::None;
        self.one_of_password = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UpdateUserRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateUserRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11user_update.proto\"\xd5\x01\n\x11UpdateUserRequest\x12\x0e\n\x02id\
    \x18\x01\x20\x01(\tR\x02id\x12\x14\n\x04name\x18\x02\x20\x01(\tH\0R\x04n\
    ame\x12\x16\n\x05email\x18\x03\x20\x01(\tH\x01R\x05email\x12\x1e\n\twork\
    space\x18\x04\x20\x01(\tH\x02R\tworkspace\x12\x1c\n\x08password\x18\x05\
    \x20\x01(\tH\x03R\x08passwordB\r\n\x0bone_of_nameB\x0e\n\x0cone_of_email\
    B\x12\n\x10one_of_workspaceB\x11\n\x0fone_of_passwordJ\xa9\x03\n\x06\x12\
    \x04\0\0\x08\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\
    \x02\0\x08\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x19\n\x0b\n\x04\x04\0\
    \x02\0\x12\x03\x03\x04\x12\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x03\x04\n\
    \n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\x0b\r\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03\x03\x10\x11\n\x0b\n\x04\x04\0\x08\0\x12\x03\x04\x04*\n\x0c\
    \n\x05\x04\0\x08\0\x01\x12\x03\x04\n\x15\n\x0b\n\x04\x04\0\x02\x01\x12\
    \x03\x04\x18(\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x04\x18\x1e\n\x0c\n\
    \x05\x04\0\x02\x01\x01\x12\x03\x04\x1f#\n\x0c\n\x05\x04\0\x02\x01\x03\
    \x12\x03\x04&'\n\x0b\n\x04\x04\0\x08\x01\x12\x03\x05\x04,\n\x0c\n\x05\
    \x04\0\x08\x01\x01\x12\x03\x05\n\x16\n\x0b\n\x04\x04\0\x02\x02\x12\x03\
    \x05\x19*\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x05\x19\x1f\n\x0c\n\x05\
    \x04\0\x02\x02\x01\x12\x03\x05\x20%\n\x0c\n\x05\x04\0\x02\x02\x03\x12\
    \x03\x05()\n\x0b\n\x04\x04\0\x08\x02\x12\x03\x06\x044\n\x0c\n\x05\x04\0\
    \x08\x02\x01\x12\x03\x06\n\x1a\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x06\x1d\
    2\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x06\x1d#\n\x0c\n\x05\x04\0\x02\
    \x03\x01\x12\x03\x06$-\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x0601\n\x0b\
    \n\x04\x04\0\x08\x03\x12\x03\x07\x042\n\x0c\n\x05\x04\0\x08\x03\x01\x12\
    \x03\x07\n\x19\n\x0b\n\x04\x04\0\x02\x04\x12\x03\x07\x1c0\n\x0c\n\x05\
    \x04\0\x02\x04\x05\x12\x03\x07\x1c\"\n\x0c\n\x05\x04\0\x02\x04\x01\x12\
    \x03\x07#+\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x07./b\x06proto3\
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

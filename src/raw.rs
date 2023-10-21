#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
)]
pub const mocha_error_t_MOCHA_ERROR_NONE: mocha_error_t = 0;
pub const mocha_error_t_MOCHA_ERROR_MISSING_FIELD: mocha_error_t = 1;
pub const mocha_error_t_MOCHA_ERROR_DUPLICATE_FIELD: mocha_error_t = 2;
pub const mocha_error_t_MOCHA_ERROR_ROOT_REFERENCE: mocha_error_t = 3;
pub const mocha_error_t_MOCHA_ERROR_OUT_OF_MEMORY: mocha_error_t = 4;
pub const mocha_error_t_MOCHA_ERROR_INVALID_CHARACTER: mocha_error_t = 5;
pub const mocha_error_t_MOCHA_ERROR_OVERFLOW: mocha_error_t = 6;
pub const mocha_error_t_MOCHA_ERROR_END_OF_STREAM: mocha_error_t = 7;
pub const mocha_error_t_MOCHA_ERROR_UNEXPECTED_TOKEN: mocha_error_t = 8;
pub const mocha_error_t_MOCHA_ERROR_UNEXPECTED_CHARACTER: mocha_error_t = 9;
pub type mocha_error_t = ::std::os::raw::c_uint;
pub const mocha_value_type_t_MOCHA_VALUE_TYPE_NIL: mocha_value_type_t = 0;
pub const mocha_value_type_t_MOCHA_VALUE_TYPE_STRING: mocha_value_type_t = 1;
pub const mocha_value_type_t_MOCHA_VALUE_TYPE_REFERENCE: mocha_value_type_t = 2;
pub const mocha_value_type_t_MOCHA_VALUE_TYPE_BOOLEAN: mocha_value_type_t = 3;
pub const mocha_value_type_t_MOCHA_VALUE_TYPE_OBJECT: mocha_value_type_t = 4;
pub const mocha_value_type_t_MOCHA_VALUE_TYPE_ARRAY: mocha_value_type_t = 5;
pub const mocha_value_type_t_MOCHA_VALUE_TYPE_FLOAT64: mocha_value_type_t = 6;
pub const mocha_value_type_t_MOCHA_VALUE_TYPE_INTEGER64: mocha_value_type_t = 7;
pub type mocha_value_type_t = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mocha_reference_t {
    pub name: *const ::std::os::raw::c_char,
    pub name_len: usize,
    pub child: *const ::std::os::raw::c_void,
    pub index: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mocha_array_t {
    pub items: *mut ::std::os::raw::c_void,
    pub items_len: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mocha_object_t {
    pub fields: *mut ::std::os::raw::c_void,
    pub fields_len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mocha_value_t {
    pub string: *const ::std::os::raw::c_char,
    pub reference: mocha_reference_t,
    pub boolean: ::std::os::raw::c_int,
    pub object: mocha_object_t,
    pub array: mocha_array_t,
    pub float64: f64,
    pub integer64: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mocha_field_t {
    pub name: *const ::std::os::raw::c_char,
    pub value: mocha_value_t,
    pub type_: mocha_value_type_t,
}

extern "C" {
    pub fn mocha_parse(
        object: *mut mocha_object_t,
        src: *const ::std::os::raw::c_char,
    ) -> mocha_error_t;
    pub fn mocha_nparse(
        object: *mut mocha_object_t,
        src: *const ::std::os::raw::c_char,
	len: usize,
    ) -> mocha_error_t;
    pub fn mocha_deinit(object: *mut mocha_object_t);
    pub fn mocha_field(object: *const mocha_object_t, index: usize) -> mocha_field_t;
    pub fn mocha_array(
        array: *const mocha_array_t,
        value: *mut mocha_value_t,
        index: usize,
    ) -> mocha_value_type_t;
    pub fn mocha_reference_next(reference: *mut mocha_reference_t) -> ::std::os::raw::c_int;
}

pub mod raw;
use std::slice;
use std::ffi::CStr;
use std::os::raw::c_void;

#[derive(Debug)]
pub enum MochaError {
    MissingField,
    DuplicateField,
    RootReference,
    OutOfMemory,
    InvalidCharacter,
    Overflow,
    EndOfStream,
    UnexpectedToken,
    UnexpectedCharacter
}

#[derive(Debug)]
pub enum Value<'a> {
    String(&'a [u8]),
    Ref(Reference<'a>),
    Bool(bool),
    Object(Object),
    Array(Array),
    Float(f64),
    Int(i64),
    Nil,
}

#[derive(Debug)]
pub struct Reference<'a> {
    child: *const c_void,
    pub name: &'a [u8],
    pub index: usize,
}

#[derive(Debug)]
pub struct Field<'a> {
    pub name: &'a [u8],
    pub value: Value<'a>,
}

#[derive(Debug)]
pub struct Array {
    array: *mut c_void,
    pub len: usize,
}

#[derive(Debug)]
pub struct Object {
    obj: *mut c_void,
    pub len: usize,
}

#[derive(Debug)]
pub struct Mocha {
    obj: *mut c_void,
    pub len: usize,
}

impl Drop for Mocha {
    fn drop(&mut self) {
	unsafe { raw::mocha_deinit(&mut raw::mocha_object_t{fields: self.obj, fields_len: self.len}) }
    }
}

impl Mocha {
    pub fn parse(src: &str) -> Result<Self, MochaError> {
	unsafe {
	    let mut object = raw::mocha_object_t{fields: 0 as _, fields_len: 0};
	    let mocha = raw::mocha_nparse(&mut object as _, src.as_ptr() as _, src.len());
	    if let Some(err) = handle_mocha_error(mocha) {
		Err(err)
	    } else {
		Ok(Self {obj: object.fields, len: object.fields_len})
	    }
	}
    }

    pub fn get(&self, index: usize) -> Option<Field> {
	unsafe {
	    if index >= self.len { return None }
	    let field = raw::mocha_field(&raw::mocha_object_t{fields: self.obj, fields_len: self.len} as _, index);
	    match field.type_ {
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_NIL => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
									     value: Value::Nil }),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_STRING => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
										value: Value::String(CStr::from_ptr(field.value.string).to_bytes()) }),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_REFERENCE => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
										   value: Value::Ref(Reference{child: field.value.reference.child,
													       name: slice::from_raw_parts(field.value.reference.name as _,
																	   field.value.reference.name_len),
													       index: field.value.reference.index}) }),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_BOOLEAN => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
										 value: Value::Bool(if field.value.boolean == 0 { false } else { true }) }),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_OBJECT => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
										value: Value::Object(Object{obj: field.value.object.fields,
													    len: field.value.object.fields_len}) }),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_ARRAY => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
									       value: Value::Array(Array{array: field.value.array.items,
													 len: field.value.array.items_len}) }),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_FLOAT64 => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
										 value: Value::Float(field.value.float64) }),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_INTEGER64 => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
										 value: Value::Int(field.value.integer64) }),
		_ => None
	    }
	}
    }
}

impl Object {
    pub fn get(&self, index: usize) -> Option<Field> {
	unsafe {
	    if index >= self.len { return None }
	    let field = raw::mocha_field(&raw::mocha_object_t{fields: self.obj, fields_len: self.len} as _, index);
	    match field.type_ {
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_NIL => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
									     value: Value::Nil }),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_STRING => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
										value: Value::String(CStr::from_ptr(field.value.string).to_bytes()) }),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_REFERENCE => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
										   value: Value::Ref(Reference{child: field.value.reference.child,
													       name: slice::from_raw_parts(field.value.reference.name as _,
																	   field.value.reference.name_len),
													       index: field.value.reference.index}) }),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_BOOLEAN => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
										 value: Value::Bool(if field.value.boolean == 0
												    { false } else { true })}),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_OBJECT => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
										value: Value::Object(Object{obj: field.value.object.fields,
													    len: field.value.object.fields_len}) }),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_ARRAY => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
									       value: Value::Array(Array{array: field.value.array.items,
													 len: field.value.array.items_len}) }),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_FLOAT64 => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
										 value: Value::Float(field.value.float64) }),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_INTEGER64 => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
										   value: Value::Int(field.value.integer64) }),
		_ => None
	    }
	}
    }
}

impl Array {
    pub fn get(&self, index: usize) -> Option<Value> {
	unsafe {
	    if index >= self.len { return None }
	    let mut value = raw::mocha_value_t{boolean: 0};
	    let type_ = raw::mocha_array(&raw::mocha_array_t{ items: self.array, items_len: self.len}, &mut value as _, index);
	    match type_ {
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_NIL => Some(Value::Nil),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_STRING => Some(Value::String(CStr::from_ptr(value.string).to_bytes())),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_REFERENCE => Some(Value::Ref(Reference{child: value.reference.child,
													       name: slice::from_raw_parts(value.reference.name as _,
																	   value.reference.name_len),
													       index: value.reference.index})),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_BOOLEAN => Some(Value::Bool(if value.boolean == 0 { false } else { true })),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_OBJECT => Some(Value::Object(Object{obj: value.object.fields,
													    len: value.object.fields_len})),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_ARRAY => Some(Value::Array(Array{array: value.array.items, len: value.array.items_len})),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_FLOAT64 => Some(Value::Float(value.float64)),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_INTEGER64 => Some(Value::Int(value.integer64)),
		_ => None
	    }
	}
    }
}

impl Reference<'_> {
    pub fn next(&self) -> Option<Self> {
	unsafe {
	    let mut reference = raw::mocha_reference_t{name: 0 as _,
						       name_len: 0,
						       child: self.child,index: 0};
	    let err = raw::mocha_reference_next(&mut reference as _);
	    if err == 0 {
		Some(Self{child: reference.child,
		     name: slice::from_raw_parts(reference.name as _,
						 reference.name_len),
		     index: reference.index})
	    } else {
		None
	    }
	}
    }
}

#[inline(always)]
fn handle_mocha_error(err: raw::mocha_error_t) -> Option<MochaError> {
    match err {
	raw::mocha_error_t_MOCHA_ERROR_NONE => None,
	raw::mocha_error_t_MOCHA_ERROR_MISSING_FIELD => Some(MochaError::MissingField),
	raw::mocha_error_t_MOCHA_ERROR_DUPLICATE_FIELD => Some(MochaError::DuplicateField),
	raw::mocha_error_t_MOCHA_ERROR_ROOT_REFERENCE => Some(MochaError::RootReference),
	raw::mocha_error_t_MOCHA_ERROR_OUT_OF_MEMORY => Some(MochaError::OutOfMemory),
	raw::mocha_error_t_MOCHA_ERROR_INVALID_CHARACTER => Some(MochaError::InvalidCharacter),
	raw::mocha_error_t_MOCHA_ERROR_OVERFLOW => Some(MochaError::Overflow),
	raw::mocha_error_t_MOCHA_ERROR_END_OF_STREAM => Some(MochaError::EndOfStream),
	raw::mocha_error_t_MOCHA_ERROR_UNEXPECTED_TOKEN => Some(MochaError::UnexpectedToken),
	raw::mocha_error_t_MOCHA_ERROR_UNEXPECTED_CHARACTER => Some(MochaError::UnexpectedCharacter),
	_ => None,
    }
}

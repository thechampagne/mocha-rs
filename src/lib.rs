pub mod raw;
use std::ffi::CStr;

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
    Ref(Reference),
    Bool(bool),
    Object(Object),
    Array(Array),
    Float(f64),
    Int(i64),
    Nil,
}

#[derive(Debug)]
pub struct Reference {
    reference: raw::mocha_reference_t,
}

#[derive(Debug)]
pub struct Field<'a> {
    pub name: &'a [u8],
    pub value: Value<'a>,
}

#[derive(Debug)]
pub struct Array {
    array: raw::mocha_array_t,
}

#[derive(Debug)]
pub struct Object {
    obj: raw::mocha_object_t,
}

#[derive(Debug)]
pub struct Mocha {
    obj: raw::mocha_object_t,
}

impl Mocha {
    pub fn parse(src: &str) -> Result<Self, MochaError> {
	unsafe {
	    let mut object = raw::mocha_object_t{fields: 0 as _, fields_len: 0};
	    let mocha = raw::mocha_nparse(&mut object as _, src.as_ptr() as _, src.len());
	    if let Some(err) = handle_mocha_error(mocha) {
		Err(err)
	    } else {
		Ok(Self {obj: object})
	    }
	}
    }

    pub fn field_get(&self, index: usize) -> Option<Field> {
	unsafe {
	    if index >= self.obj.fields_len { return None }
	    let field = raw::mocha_field(&self.obj as _, index);
	    match field.type_ {
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_NIL => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
									     value: Value::Nil }),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_STRING => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
										value: Value::String(CStr::from_ptr(field.value.string).to_bytes()) }),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_REFERENCE => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
										   value: Value::Ref(Reference{reference: field.value.reference}) }),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_BOOLEAN => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
										 value: Value::Bool(if field.value.boolean == 0 { false } else { true }) }),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_OBJECT => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
										value: Value::Object(Object{obj: field.value.object}) }),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_ARRAY => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
									       value: Value::Array(Array{array: field.value.array}) }),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_FLOAT64 => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
										 value: Value::Float(field.value.float64) }),
		raw::mocha_value_type_t_MOCHA_VALUE_TYPE_INTEGER64 => Some(Field { name: CStr::from_ptr(field.name).to_bytes(),
										 value: Value::Int(field.value.integer64) }),
		_ => None
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

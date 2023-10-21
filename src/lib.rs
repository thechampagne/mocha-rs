pub mod raw;

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

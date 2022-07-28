pub type Number = f64;

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    String(String),
    Number(Number),
    Boolean(bool),
    Nil,
}

impl Value {
    pub fn ty(&self) -> Type {
        match self {
            Value::String(_) => Type::String,
            Value::Number(_) => Type::Number,
            Value::Boolean(_) => Type::Boolean,
            Value::Nil => Type::Nil,
        }
    }
}

pub trait Variant: Sized + 'static {
    fn into_value(self) -> Value;
    fn from_value(value: Value) -> Result<Self, TypeError>;
}

impl Variant for String {
    fn into_value(self) -> Value {
        Value::String(self)
    }

    fn from_value(value: Value) -> Result<Self, TypeError> {
        match value {
            Value::String(value) => Ok(value),
            _ => Err(TypeError {
                expected: &[Type::String],
                actual: value,
            }),
        }
    }
}

impl Variant for Number {
    fn into_value(self) -> Value {
        Value::Number(self)
    }

    fn from_value(value: Value) -> Result<Self, TypeError> {
        match value {
            Value::Number(value) => Ok(value),
            _ => Err(TypeError {
                expected: &[Type::Number],
                actual: value,
            }),
        }
    }
}

impl Variant for bool {
    fn into_value(self) -> Value {
        Value::Boolean(self)
    }

    fn from_value(value: Value) -> Result<Self, TypeError> {
        match value {
            Value::Boolean(value) => Ok(value),
            _ => Err(TypeError {
                expected: &[Type::Boolean],
                actual: value,
            }),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Type {
    String,
    Number,
    Boolean,
    Nil,
}

#[derive(Debug)]
pub struct TypeError {
    pub expected: &'static [Type],
    pub actual: Value,
}

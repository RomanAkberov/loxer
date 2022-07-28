use crate::{
    ast::{BinaryOperator, Expression, UnaryOperator},
    value::{Number, Type, TypeError, Value, Variant},
};

#[derive(Debug)]
pub enum RuntimeError {
    TypeError(TypeError),
}

impl From<TypeError> for RuntimeError {
    fn from(error: TypeError) -> Self {
        Self::TypeError(error)
    }
}

pub fn eval(expression: Expression) -> Result<Value, RuntimeError> {
    match expression {
        Expression::Literal(value) => Ok(value),
        Expression::Grouping(expression) => eval(*expression),
        Expression::Unary(operator, expression) => {
            let value = eval(*expression)?;
            match operator {
                UnaryOperator::Neg => eval_unary(value, |v: Number| -v),
                UnaryOperator::Not => Ok(Value::Boolean(!is_truthy(value))),
            }
        }
        Expression::Binary(operator, left, right) => {
            let left = eval(*left)?;
            let right = eval(*right)?;
            match operator {
                BinaryOperator::Add => match (left, right) {
                    (Value::String(left), Value::String(right)) => Ok(Value::String(left + &right)),
                    (Value::Number(left), Value::Number(right)) => Ok(Value::Number(left + right)),
                    (Value::String(_), right) => Err(RuntimeError::TypeError(TypeError {
                        expected: &[Type::String],
                        actual: right,
                    })),
                    (Value::Number(_), right) => Err(RuntimeError::TypeError(TypeError {
                        expected: &[Type::Number],
                        actual: right,
                    })),
                    (left, _) => Err(RuntimeError::TypeError(TypeError {
                        expected: &[Type::Number, Type::String],
                        actual: left,
                    })),
                },
                BinaryOperator::Sub => eval_binary(left, right, |a: Number, b: Number| a - b),
                BinaryOperator::Div => eval_binary(left, right, |a: Number, b: Number| a / b),
                BinaryOperator::Mul => eval_binary(left, right, |a: Number, b: Number| a * b),
                BinaryOperator::Equal => Ok(Value::Boolean(left == right)),
                BinaryOperator::NotEqual => Ok(Value::Boolean(left != right)),
                BinaryOperator::Greater => eval_binary(left, right, |a: Number, b: Number| a > b),
                BinaryOperator::GreaterEqual => {
                    eval_binary(left, right, |a: Number, b: Number| a >= b)
                }
                BinaryOperator::Less => eval_binary(left, right, |a: Number, b: Number| a < b),
                BinaryOperator::LessEqual => {
                    eval_binary(left, right, |a: Number, b: Number| a <= b)
                }
            }
        }
    }
}

fn is_truthy(value: Value) -> bool {
    !matches!(value, Value::Boolean(false) | Value::Nil)
}

fn eval_binary<A, B, F>(left: Value, right: Value, f: F) -> Result<Value, RuntimeError>
where
    A: Variant,
    B: Variant,
    F: Fn(A, A) -> B,
{
    Ok(f(A::from_value(left)?, A::from_value(right)?).into_value())
}

fn eval_unary<A, B, F>(value: Value, f: F) -> Result<Value, RuntimeError>
where
    A: Variant,
    B: Variant,
    F: Fn(A) -> B,
{
    Ok(f(A::from_value(value)?).into_value())
}

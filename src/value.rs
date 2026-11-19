use std::fmt;

use crate::error::AlphaError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    Integer(i64),
    Boolean(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(value) => write!(f, "{value}"),
            Self::Boolean(value) => write!(f, "{value}"),
        }
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("unable to convert {value} to {target_type}")]
pub struct Error {
    value: String,
    target_type: &'static str,
}

impl AlphaError for Error {
    fn error_type(&self) -> &'static str {
        "ValueConversionError"
    }
}

impl TryFrom<Value> for i64 {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(value) => Ok(value),
            Value::Boolean(value) => Err(Error {
                value: value.to_string(),
                target_type: "integer",
            }),
        }
    }
}

impl TryFrom<Value> for bool {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Boolean(value) => Ok(value),
            Value::Integer(value) => Err(Error {
                value: value.to_string(),
                target_type: "boolean",
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::zero(0)]
    #[case::positive(42)]
    #[case::negative(-42)]
    fn converts_integer_values(#[case] number: i64) {
        let value: Value = number.into();
        assert_eq!(value, Value::Integer(number));
        assert_eq!(i64::try_from(value).unwrap(), number);
    }

    #[rstest]
    #[case::true_value(true)]
    #[case::false_value(false)]
    fn converts_boolean_values(#[case] boolean: bool) {
        let value: Value = boolean.into();
        assert_eq!(value, Value::Boolean(boolean));
        assert_eq!(bool::try_from(value).unwrap(), boolean);
    }

    #[rstest]
    #[case::zero(Value::Integer(0), "0")]
    #[case::positive(Value::Integer(42), "42")]
    #[case::negative(Value::Integer(-42), "-42")]
    #[case::true_value(Value::Boolean(true), "true")]
    #[case::false_value(Value::Boolean(false), "false")]
    fn displays_value(#[case] value: Value, #[case] expected: &str) {
        assert_eq!(value.to_string(), expected);
    }

    #[rstest]
    #[case::true_value(true)]
    #[case::false_value(false)]
    fn rejects_boolean_to_integer_conversion(#[case] boolean: bool) {
        let value: Value = boolean.into();
        let error = i64::try_from(value).unwrap_err();

        assert_eq!(error.error_type(), "ValueConversionError");
        assert_eq!(
            error.to_string(),
            format!("unable to convert {boolean} to integer")
        );
    }

    #[rstest]
    #[case::zero(0)]
    #[case::one(1)]
    #[case::negative(-1)]
    fn rejects_integer_to_boolean_conversion(#[case] number: i64) {
        let value: Value = number.into();
        let error = bool::try_from(value).unwrap_err();

        assert_eq!(error.error_type(), "ValueConversionError");
        assert_eq!(
            error.to_string(),
            format!("unable to convert {number} to boolean")
        );
    }
}

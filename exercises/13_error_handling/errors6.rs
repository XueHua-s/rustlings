// 对于库(lib)代码而言，不建议使用像 `Box<dyn Error>` 这样能捕获所有错误的类型，
// 因为调用者可能希望基于错误内容来做决策，而不是将错误打印出来或者进一步传播它。
// 这里，我们定义了一个自定义错误类型，以便当我们的函数返回错误时，调用者能够决定下一步该怎么做。 

use std::{fmt::Error, num::ParseIntError};

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// 一个将在 `PositiveNonzeroInteger::parse` 中使用的自定义错误类型。
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}
use std::fmt;

impl fmt::Display for ParsePosNonzeroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsePosNonzeroError::Creation(e) => write!(f, "Creation error: {:?}", e),
            ParsePosNonzeroError::ParseInt(e) => write!(f, "Parse int error: {}", e),
        }
    }
}

impl std::error::Error for ParsePosNonzeroError {}
impl From<ParseIntError> for ParsePosNonzeroError {
    fn from(err: ParseIntError) -> Self {
        ParsePosNonzeroError::ParseInt(err)
    }   
}
impl From<CreationError> for ParsePosNonzeroError {
    fn from(err: CreationError) -> Self {
        ParsePosNonzeroError::Creation(err)
    }
}
impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> Self {
        Self::Creation(err)
    }

    // TODO: 在此处添加另一个错误转换(error conversion)函数。
    fn from_parse_int(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(Self(x as u64)),
        }
    }

    fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {
        let x: i64 = s.parse()?;
        Ok(Self::new(x)?)
    }
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            PositiveNonzeroInteger::parse("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_)),
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            PositiveNonzeroInteger::parse("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative)),
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            PositiveNonzeroInteger::parse("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero)),
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42).unwrap();
        assert_eq!(x.0, 42);
        assert_eq!(PositiveNonzeroInteger::parse("42"), Ok(x));
    }
}

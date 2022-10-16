use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq, Eq)]
pub enum Bool {
    Bool(bool),
    Other(String),
}

impl<'de> Deserialize<'de> for Bool {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match s.parse::<bool>() {
            Ok(x) => Ok(Self::Bool(x)),
            Err(_) => Ok(Self::Other(s)),
        }
    }
}

pub fn option_bool_yn<'de, D>(deserializer: D) -> Result<Option<Bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = match String::deserialize(deserializer) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match s.as_str() {
        "no" => Ok(Some(Bool::Bool(false))),
        "yes" => Ok(Some(Bool::Bool(false))),
        _ => Ok(Some(Bool::Other(s))),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Integer {
    Integer(i64),
    Other(String),
}

impl<'de> Deserialize<'de> for Integer {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match s.parse::<i64>() {
            Ok(x) => Ok(Self::Integer(x)),
            Err(_) => Ok(Self::Other(s)),
        }
    }
}

pub fn option_integer_nonnegative<'de, D>(d: D) -> Result<Option<Integer>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = match String::deserialize(d) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match s.parse::<i64>() {
        Ok(x) => {
            if x < 0 {
                return Ok(Some(Integer::Other(s)));
            }
            Ok(Some(Integer::Integer(x)))
        }
        Err(_) => Ok(Some(Integer::Other(s))),
    }
}

pub fn option_integer_positive<'de, D>(d: D) -> Result<Option<Integer>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = match String::deserialize(d) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match s.parse::<i64>() {
        Ok(x) => {
            if x < 1 {
                return Ok(Some(Integer::Other(s)));
            }
            Ok(Some(Integer::Integer(x)))
        }
        Err(_) => Ok(Some(Integer::Other(s))),
    }
}

#[derive(Debug, PartialEq)]
pub enum Float {
    Float(f64),
    Other(String),
}

impl<'de> Deserialize<'de> for Float {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match s.parse::<f64>() {
            Ok(x) => Ok(Self::Float(x)),
            Err(_) => Ok(Self::Other(s)),
        }
    }
}

pub fn option_float_nonnegative<'de, D>(d: D) -> Result<Option<Float>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = match String::deserialize(d) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match s.parse::<f64>() {
        Ok(x) => {
            if x < 0.0 {
                return Ok(Some(Float::Other(s)));
            }
            Ok(Some(Float::Float(x)))
        }
        Err(_) => Ok(Some(Float::Other(s))),
    }
}

// Tries to convert to integer if possible, float otherwise.
#[derive(Debug, PartialEq)]
pub enum Number {
    Integer(i64),
    Float(f64),
    Other(String),
}

impl<'de> Deserialize<'de> for Number {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let mut s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        s = match s.parse::<i64>() {
            Ok(x) => {
                return Ok(Number::Integer(x));
            }
            Err(_) => s,
        };

        match s.parse::<f64>() {
            Ok(x) => Ok(Number::Float(x)),
            Err(_) => Ok(Number::Other(s)),
        }
    }
}

pub fn option_number_nonnegative<'de, D>(d: D) -> Result<Option<Number>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut s = match String::deserialize(d) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    s = match s.parse::<i64>() {
        Ok(x) => {
            if x < 0 {
                return Ok(Some(Number::Other(s)));
            }
            return Ok(Some(Number::Integer(x)));
        }
        Err(_) => s,
    };

    match s.parse::<f64>() {
        Ok(x) => {
            if x < 0.0 {
                return Ok(Some(Number::Other(s)));
            }
            Ok(Some(Number::Float(x)))
        }
        Err(_) => Ok(Some(Number::Other(s))),
    }
}

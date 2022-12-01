use crate::utils;

/// Used for deserializing boolean values.
#[derive(Debug, PartialEq, Eq)]
pub enum Bool {
    Ok(bool),
    Other(String),
}

impl Default for Bool {
    fn default() -> Self {
        Bool::Ok(false)
    }
}

impl std::str::FromStr for Bool {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<bool>() {
            Ok(x) => Ok(Self::Ok(x)),
            Err(_) => Ok(Self::Other(s.to_string())),
        }
    }
}

impl std::fmt::Display for Bool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok(t) => write!(f, "{t}"),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

// impl<'de> Deserialize<'de> for Bool {
//     fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
//         utils::deserialize_using_from_str(d)
//     }
// }

// pub enum BoolYN {}
//
// impl<'de> serde_with::DeserializeAs<'de, Bool> for BoolYN {
//     fn deserialize_as<D: Deserializer<'de>>(d: D) -> Result<Bool, D::Error> {
//         let s = String::deserialize(d);
//
//         match s {
//             Ok(s) => match s.as_str() {
//                 "yes" => Ok(Bool::Ok(true)),
//                 "no" => Ok(Bool::Ok(false)),
//                 _ => Ok(Bool::Other(s)),
//             },
//             Err(e) => Err(e),
//         }
//     }
// }

/// Used for deserializing integer values.
#[derive(Debug, PartialEq, Eq)]
pub enum Integer {
    Ok(i64),
    Other(String),
}

impl std::str::FromStr for Integer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i64>() {
            Ok(x) => Ok(Self::Ok(x)),
            Err(_) => Ok(Self::Other(s.to_string())),
        }
    }
}

impl std::fmt::Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok(t) => write!(f, "{t}"),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

// impl<'de> Deserialize<'de> for Integer {
//     fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
//         utils::deserialize_using_from_str(d)
//     }
// }
//
// pub enum IntegerPositive {}
//
// impl<'de> serde_with::DeserializeAs<'de, Integer> for IntegerPositive {
//     fn deserialize_as<D: Deserializer<'de>>(d: D) -> Result<Integer, D::Error> {
//         match Integer::deserialize(d) {
//             Ok(Integer::Ok(x)) => {
//                 if x <= 0 {
//                     return Ok(Integer::Other(x.to_string()));
//                 }
//                 Ok(Integer::Ok(x))
//             }
//             Ok(Integer::Other(s)) => Ok(Integer::Other(s)),
//             Err(e) => Err(e),
//         }
//     }
// }

// pub enum IntegerNonNegative {}
//
// impl<'de> serde_with::DeserializeAs<'de, Integer> for IntegerNonNegative {
//     fn deserialize_as<D: Deserializer<'de>>(d: D) -> Result<Integer, D::Error> {
//         match Integer::deserialize(d) {
//             Ok(Integer::Ok(x)) => {
//                 if x < 0 {
//                     return Ok(Integer::Other(x.to_string()));
//                 }
//                 Ok(Integer::Ok(x))
//             }
//             Ok(Integer::Other(s)) => Ok(Integer::Other(s)),
//             Err(e) => Err(e),
//         }
//     }
// }

/// Used for deserializing floating-point values.
#[derive(Debug, PartialEq)]
pub enum Float {
    Ok(f64),
    Other(String),
}

impl std::str::FromStr for Float {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<f64>() {
            Ok(x) => Ok(Self::Ok(x)),
            Err(_) => Ok(Self::Other(s.to_string())),
        }
    }
}

impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok(t) => write!(f, "{t}"),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

// impl<'de> Deserialize<'de> for Float {
//     fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
//         utils::deserialize_using_from_str(d)
//     }
// }
//
// pub enum FloatPositive {}
//
// impl<'de> serde_with::DeserializeAs<'de, Float> for FloatPositive {
//     fn deserialize_as<D: Deserializer<'de>>(d: D) -> Result<Float, D::Error> {
//         match Float::deserialize(d) {
//             Ok(Float::Ok(x)) => {
//                 if x <= 0.0 {
//                     return Ok(Float::Other(x.to_string()));
//                 }
//                 Ok(Float::Ok(x))
//             }
//             Ok(Float::Other(s)) => Ok(Float::Other(s)),
//             Err(e) => Err(e),
//         }
//     }
// }
//
// pub enum FloatNonNegative {}
//
// impl<'de> serde_with::DeserializeAs<'de, Float> for FloatNonNegative {
//     fn deserialize_as<D: Deserializer<'de>>(d: D) -> Result<Float, D::Error> {
//         match Float::deserialize(d) {
//             Ok(Float::Ok(x)) => {
//                 if x < 0.0 {
//                     return Ok(Float::Other(x.to_string()));
//                 }
//                 Ok(Float::Ok(x))
//             }
//             Ok(Float::Other(s)) => Ok(Float::Other(s)),
//             Err(e) => Err(e),
//         }
//     }
// }

/// Used for deserializing values that could be either integers or floats.
///
/// Preference is given to the *former*.
#[derive(Debug, PartialEq)]
pub enum Number {
    Integer(i64),
    Float(f64),
    Other(String),
}

impl std::str::FromStr for Number {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(x) = s.parse::<i64>() {
            return Ok(Number::Integer(x));
        };

        if let Ok(x) = s.parse::<f64>() {
            return Ok(Number::Float(x));
        };

        Ok(Number::Other(s.to_string()))
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(t) => write!(f, "{t}"),
            Self::Float(t) => write!(f, "{t}"),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

// impl<'de> Deserialize<'de> for Number {
//     fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
//         utils::deserialize_using_from_str(d)
//     }
// }
//
// pub enum NumberNonNegative {}
//
// impl<'de> serde_with::DeserializeAs<'de, Number> for NumberNonNegative {
//     fn deserialize_as<D: Deserializer<'de>>(d: D) -> Result<Number, D::Error> {
//         match Number::deserialize(d) {
//             Ok(Number::Integer(x)) => {
//                 if x < 0 {
//                     return Ok(Number::Other(x.to_string()));
//                 }
//                 Ok(Number::Integer(x))
//             }
//             Ok(Number::Float(x)) => {
//                 if x < 0.0 {
//                     return Ok(Number::Other(x.to_string()));
//                 }
//                 Ok(Number::Float(x))
//             }
//             Ok(Number::Other(s)) => Ok(Number::Other(s)),
//             Err(e) => Err(e),
//         }
//     }
// }

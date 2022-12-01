use crate::utils;

pub enum NumberConstraint {
    None,
    Positive,
    NonNegative,
}

pub enum BoolType {
    TrueFalse,
    YesNo,
}

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

impl Bool {
    pub fn parse(s: &str, bool_type: BoolType) -> Self {
        match bool_type {
            BoolType::TrueFalse => match s {
                "true" => Bool::Ok(true),
                "false" => Bool::Ok(false),
                _ => Bool::Other(s.to_string()),
            },
            BoolType::YesNo => match s {
                "yes" => Bool::Ok(true),
                "no" => Bool::Ok(false),
                _ => Bool::Other(s.to_string()),
            },
        }
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

impl Integer {
    pub fn parse(s: &str, constraint: NumberConstraint) -> Self {
        match s.parse::<i64>() {
            Ok(x) => match constraint {
                NumberConstraint::None => Self::Ok(x),
                NumberConstraint::Positive => {
                    if x > 0 {
                        Self::Ok(x)
                    } else {
                        Self::Other(s.to_string())
                    }
                }
                NumberConstraint::NonNegative => {
                    if x >= 0 {
                        Self::Ok(x)
                    } else {
                        Self::Other(s.to_string())
                    }
                }
            },
            Err(_) => Self::Other(s.to_string()),
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

/// Used for deserializing floating-point values.
#[derive(Debug, PartialEq)]
pub enum Float {
    Ok(f64),
    Other(String),
}

impl Float {
    pub fn parse(s: &str, constraint: NumberConstraint) -> Self {
        match s.parse::<f64>() {
            Ok(x) => match constraint {
                NumberConstraint::None => Self::Ok(x),
                NumberConstraint::Positive => {
                    if x > 0.0 {
                        Self::Ok(x)
                    } else {
                        Self::Other(s.to_string())
                    }
                }
                NumberConstraint::NonNegative => {
                    if x >= 0.0 {
                        Self::Ok(x)
                    } else {
                        Self::Other(s.to_string())
                    }
                }
            },
            Err(_) => Self::Other(s.to_string()),
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

/// Used for deserializing values that could be either integers or floats.
///
/// Preference is given to the *former*.
#[derive(Debug, PartialEq)]
pub enum Number {
    Integer(i64),
    Float(f64),
    Other(String),
}

impl Number {
    pub fn parse(s: &str, constraint: NumberConstraint) -> Self {
        match s.parse::<i64>() {
            Ok(x) => match constraint {
                NumberConstraint::None => Self::Integer(x),
                NumberConstraint::Positive => {
                    if x > 0 {
                        Self::Integer(x)
                    } else {
                        Self::Other(s.to_string())
                    }
                }
                NumberConstraint::NonNegative => {
                    if x >= 0 {
                        Self::Integer(x)
                    } else {
                        Self::Other(s.to_string())
                    }
                }
            },
            Err(_) => match s.parse::<f64>() {
                Ok(x) => match constraint {
                    NumberConstraint::None => Self::Float(x),
                    NumberConstraint::Positive => {
                        if x > 0.0 {
                            Self::Float(x)
                        } else {
                            Self::Other(s.to_string())
                        }
                    }
                    NumberConstraint::NonNegative => {
                        if x >= 0.0 {
                            Self::Float(x)
                        } else {
                            Self::Other(s.to_string())
                        }
                    }
                },
                Err(_) => Self::Other(s.to_string()),
            },
        }
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

use crate::Other;

pub enum NumberConstraint {
    None,
    Positive,
    NonNegative,
    Range(f64, f64),
}

pub enum BoolType {
    TrueFalse,
    YesNo,
}

/// Used for deserializing boolean values.
#[derive(Debug, PartialEq, Eq)]
pub enum Bool {
    Ok(bool),
    Other(Other),
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
                _ => Bool::Other((s.to_string(), "should be \"true\" or \"false\"".to_string())),
            },
            BoolType::YesNo => match s {
                "yes" => Bool::Ok(true),
                "no" => Bool::Ok(false),
                _ => Bool::Other((s.to_string(), "should be \"yes\" or \"no\"".to_string())),
            },
        }
    }
}

impl std::str::FromStr for Bool {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<bool>() {
            Ok(x) => Ok(Self::Ok(x)),
            Err(e) => Ok(Self::Other((s.to_string(), e.to_string()))),
        }
    }
}

impl std::fmt::Display for Bool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok(t) => write!(f, "{t}"),
            Self::Other((s, _)) => write!(f, "{s}"),
        }
    }
}

/// Used for deserializing integer values.
#[derive(Debug, PartialEq, Eq)]
pub enum Integer {
    Ok(i64),
    Other(Other),
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
                        Self::Other((s.to_string(), "should be positive".to_string()))
                    }
                }
                NumberConstraint::NonNegative => {
                    if x >= 0 {
                        Self::Ok(x)
                    } else {
                        Self::Other((s.to_string(), "should be non-negative".to_string()))
                    }
                }
                NumberConstraint::Range(min, max) => {
                    if x as f64 >= min && x as f64 <= max {
                        Self::Ok(x)
                    } else {
                        Self::Other((s.to_string(), format!("should be in range [{min}, {max}]")))
                    }
                }
            },
            Err(_) => Self::Other((s.to_string(), "should be an integer".to_string())),
        }
    }
}

impl std::fmt::Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok(t) => write!(f, "{t}"),
            Self::Other((s, _)) => write!(f, "{s}"),
        }
    }
}

/// Used for deserializing floating-point values.
#[derive(Debug, PartialEq)]
pub enum Float {
    Ok(f64),
    Other(Other),
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
                        Self::Other((s.to_string(), "should be positive".to_string()))
                    }
                }
                NumberConstraint::NonNegative => {
                    if x >= 0.0 {
                        Self::Ok(x)
                    } else {
                        Self::Other((s.to_string(), "should be non-negative".to_string()))
                    }
                }
                NumberConstraint::Range(min, max) => {
                    if x >= min && x <= max {
                        Self::Ok(x)
                    } else {
                        Self::Other((s.to_string(), format!("should be in range [{min}, {max}]")))
                    }
                }
            },
            Err(_) => Self::Other((
                s.to_string(),
                "should be a floating-point number".to_string(),
            )),
        }
    }
}

impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok(t) => write!(f, "{t}"),
            Self::Other((s, _)) => write!(f, "{s}"),
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
    Other(Other),
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
                        Self::Other((s.to_string(), "should be positive".to_string()))
                    }
                }
                NumberConstraint::NonNegative => {
                    if x >= 0 {
                        Self::Integer(x)
                    } else {
                        Self::Other((s.to_string(), "should be non-negative".to_string()))
                    }
                }
                NumberConstraint::Range(min, max) => {
                    if x as f64 >= min && x as f64 <= max {
                        Self::Integer(x)
                    } else {
                        Self::Other((s.to_string(), format!("should be in range [{min}, {max}]")))
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
                            Self::Other((s.to_string(), "should be positive".to_string()))
                        }
                    }
                    NumberConstraint::NonNegative => {
                        if x >= 0.0 {
                            Self::Float(x)
                        } else {
                            Self::Other((s.to_string(), "should be non-negative".to_string()))
                        }
                    }
                    NumberConstraint::Range(min, max) => {
                        if x >= min && x <= max {
                            Self::Float(x)
                        } else {
                            Self::Other((
                                s.to_string(),
                                format!("should be in range [{min}, {max}]"),
                            ))
                        }
                    }
                },
                Err(_) => Self::Other((s.to_string(), "should be a number".to_string())),
            },
        }
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(t) => write!(f, "{t}"),
            Self::Float(t) => write!(f, "{t}"),
            Self::Other((s, _)) => write!(f, "{s}"),
        }
    }
}

/// Duration.
#[derive(Debug, PartialEq, Eq)]
pub enum Duration {
    Duration(chrono::Duration),
    Other(Other),
}

impl Duration {
    pub fn parse_from_int_string(s: &str) -> Self {
        match s.parse::<u64>() {
            Ok(x) => Self::Duration(chrono::Duration::seconds(x as i64)),
            Err(_) => Self::Other((
                s.to_string(),
                "should be a non-negative integer".to_string(),
            )),
        }
    }

    pub fn parse_from_float_string(s: &str) -> Self {
        match s.parse::<f64>() {
            Ok(x) => {
                // Convert to nanoseconds and round to the nearest integer.
                let x = (x * 1_000_000_000.0).round() as i64;
                Self::Duration(chrono::Duration::nanoseconds(x))
            }
            Err(_) => Self::Other((
                s.to_string(),
                "should be a non-negative floating-point number".to_string(),
            )),
        }
    }
}

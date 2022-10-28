use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::str::FromStr;
use strum_macros::{Display, EnumString};

/// Geographical coordinates.
#[derive(Debug, PartialEq, Clone)]
pub enum Geo {
    Ok {
        latitude: f64,
        longitude: f64,
        altitude: Option<f64>,
        uncertainty: Option<f64>,
    },
    Other(String),
}

impl fmt::Display for Geo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ok {
                latitude,
                longitude,
                altitude,
                uncertainty,
            } => {
                let mut s = format!("geo:{latitude},{longitude}");
                if let Some(altitude) = altitude {
                    s = format!("{s},{altitude}");
                };
                if let Some(uncertainty) = uncertainty {
                    s = format!("{s};u={uncertainty}");
                };
                write!(f, "{s}")
            }
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

impl<'de> Deserialize<'de> for Geo {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match parse_geo(s) {
            Ok(geo) => Ok(geo),
            Err(e) => Err(e).map_err(D::Error::custom),
        }
    }
}

fn parse_geo(s: String) -> Result<Geo, String> {
    if s.len() < 5 || !s.starts_with("geo:") {
        return Ok(Geo::Other(s));
    }

    // Part after "geo:"
    let data = s[4..].to_string();

    let num_commas = data.matches(',').count();
    if num_commas > 2 {
        return Ok(Geo::Other(s));
    }
    let has_altitude = num_commas == 2;

    let num_semicolons = data.matches(';').count();
    if num_semicolons > 1 {
        return Ok(Geo::Other(s));
    }
    let has_uncertainty = num_semicolons == 1;

    let pattern = match (has_altitude, has_uncertainty) {
        (false, false) => {
            r"(?P<latitude>[+-]?([0-9]*[.])?[0-9]+),(?P<longitude>[+-]?([0-9]*[.])?[0-9]+)"
        }
        (false, true) => {
            r"(?P<latitude>[+-]?([0-9]*[.])?[0-9]+),(?P<longitude>[+-]?([0-9]*[.])?[0-9]+);u=(?P<uncertainty>[+-]?([0-9]*[.])?[0-9]+)"
        }
        (true, false) => {
            r"(?P<latitude>[+-]?([0-9]*[.])?[0-9]+),(?P<longitude>[+-]?([0-9]*[.])?[0-9]+),(?P<altitude>[+-]?([0-9]*[.])?[0-9]+)"
        }
        (true, true) => {
            r"(?P<latitude>[+-]?([0-9]*[.])?[0-9]+),(?P<longitude>[+-]?([0-9]*[.])?[0-9]+),(?P<altitude>[+-]?([0-9]*[.])?[0-9]+);u=(?P<uncertainty>[+-]?([0-9]*[.])?[0-9]+)"
        }
    };

    let re = match regex::Regex::new(pattern) {
        Ok(re) => re,
        Err(e) => return Err(e.to_string()),
    };
    let caps = match re.captures(data.as_str()) {
        Some(caps) => caps,
        None => return Ok(Geo::Other(s)),
    };

    let latitude = &caps["latitude"];
    let latitude = match latitude.parse::<f64>() {
        Ok(latitude) => latitude,
        Err(_) => return Ok(Geo::Other(s)),
    };

    let longitude = &caps["longitude"];
    let longitude = match longitude.parse::<f64>() {
        Ok(longitude) => longitude,
        Err(_) => return Ok(Geo::Other(s)),
    };

    let mut altitude: Option<f64> = None;
    if has_altitude {
        altitude = match &caps["altitude"].parse::<f64>() {
            Ok(altitude) => Some(*altitude),
            Err(_) => return Ok(Geo::Other(s)),
        };
    }

    let mut uncertainty: Option<f64> = None;
    if has_uncertainty {
        uncertainty = match &caps["uncertainty"].parse::<f64>() {
            Ok(uncertainty) => Some(*uncertainty),
            Err(_) => return Ok(Geo::Other(s)),
        };
    }

    Ok(Geo::Ok {
        latitude,
        longitude,
        altitude,
        uncertainty,
    })
}

/// Type of [Osm](Osm) object.
#[derive(Debug, PartialEq, Eq, EnumString, Display, Clone)]
pub enum OsmType {
    #[strum(serialize = "N")]
    Node,
    #[strum(serialize = "W")]
    Way,
    #[strum(serialize = "R")]
    Relation,
}

/// Open Street Map object.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Osm {
    Ok {
        type_: OsmType,
        id: u64,
        revision: Option<u64>,
    },
    Other(String),
}

impl fmt::Display for Osm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ok {
                type_,
                id,
                revision,
            } => {
                let mut s = format!("{type_}{id}");
                if let Some(revision) = revision {
                    s = format!("{s}#{revision}");
                };
                write!(f, "{s}")
            }
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

impl<'de> Deserialize<'de> for Osm {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match parse_osm(s) {
            Ok(geo) => Ok(geo),
            Err(e) => Err(e).map_err(D::Error::custom),
        }
    }
}

fn parse_osm(s: String) -> Result<Osm, String> {
    let has_revision = s.matches('#').count() > 0;

    let pattern = {
        if has_revision {
            r"(?P<type_>[NWR])(?P<id>\d+)#(?P<revision>\d+)"
        } else {
            r"(?P<type_>[NWR])(?P<id>\d+)"
        }
    };

    let re = match regex::Regex::new(pattern) {
        Ok(re) => re,
        Err(e) => return Err(e.to_string()),
    };
    let caps = match re.captures(s.as_str()) {
        Some(caps) => caps,
        None => return Ok(Osm::Other(s)),
    };

    let type_ = match OsmType::from_str(&caps["type_"]) {
        Ok(type_) => type_,
        _ => return Err("something went wrong".to_string()),
    };

    let id = match &caps["id"].parse::<u64>() {
        Ok(id) => *id,
        Err(_) => return Ok(Osm::Other(s)),
    };

    let mut revision: Option<u64> = None;
    if has_revision {
        revision = match &caps["revision"].parse::<u64>() {
            Ok(revision) => Some(*revision),
            Err(_) => return Ok(Osm::Other(s)),
        };
    }

    Ok(Osm::Ok {
        type_,
        id,
        revision,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geo() {
        let strings = vec![
            "geo:37.786971,-122.399677",
            "geo:37.786971,-122.399677,250",
            "geo:37.786971,-122.399677;u=350",
            "geo:37.786971,-122.399677,250;u=350",
            "geo:37.786971,-122.399677,250,u=350",
        ];
        let geos = vec![
            Geo::Ok {
                latitude: 37.786971,
                longitude: -122.399677,
                altitude: None,
                uncertainty: None,
            },
            Geo::Ok {
                latitude: 37.786971,
                longitude: -122.399677,
                altitude: Some(250.0),
                uncertainty: None,
            },
            Geo::Ok {
                latitude: 37.786971,
                longitude: -122.399677,
                altitude: None,
                uncertainty: Some(350.0),
            },
            Geo::Ok {
                latitude: 37.786971,
                longitude: -122.399677,
                altitude: Some(250.0),
                uncertainty: Some(350.0),
            },
            Geo::Other("geo:37.786971,-122.399677,250,u=350".to_string()),
        ];

        for (s, geo) in strings.iter().zip(geos.iter()) {
            pretty_assertions::assert_eq!(parse_geo(s.to_string()), Ok(geo.clone()));
            pretty_assertions::assert_eq!(format!("{}", geo), *s);
        }
    }

    #[test]
    fn test_parse_osm() {
        let strings = vec!["R148838", "W5013364", "R7444#188", "7444#188"];
        let osms = vec![
            Osm::Ok {
                type_: OsmType::Relation,
                id: 148838,
                revision: None,
            },
            Osm::Ok {
                type_: OsmType::Way,
                id: 5013364,
                revision: None,
            },
            Osm::Ok {
                type_: OsmType::Relation,
                id: 7444,
                revision: Some(188),
            },
            Osm::Other("7444#188".to_string()),
        ];

        for (s, osm) in strings.iter().zip(osms.iter()) {
            pretty_assertions::assert_eq!(parse_osm(s.to_string()), Ok(osm.clone()));
            pretty_assertions::assert_eq!(format!("{}", osm), *s);
        }
    }
}

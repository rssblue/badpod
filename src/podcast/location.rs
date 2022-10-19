use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
pub struct GeoCoordinates {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
    pub uncertainty: Option<f64>,
}

#[derive(Debug, PartialEq)]
pub enum Geo {
    Ok(GeoCoordinates),
    Other(String),
}

impl<'de> Deserialize<'de> for Geo {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match parse_geo(s.clone()) {
            Ok(geo_coordinates) => Ok(Geo::Ok(geo_coordinates)),
            Err(_) => Ok(Geo::Other(s)),
        }
    }
}

fn parse_geo(s: String) -> Result<GeoCoordinates, String> {
    if s.len() < 5 || !s.starts_with("geo:") {
        return Err("should start with \"geo:\"".to_string());
    }

    // Part after "geo:"
    let data = s[4..].to_string();

    let num_commas = data.matches(",").count();
    if num_commas > 2 {
        return Err("should not have more than two commas".to_string());
    }
    let has_altitude = num_commas == 2;

    let num_semicolons = data.matches(";").count();
    if num_semicolons > 1 {
        return Err("should not have more than one semicolon".to_string());
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
        None => return Err("wrong pattern".to_string()),
    };

    let latitude = &caps["latitude"];
    let latitude = match latitude.parse::<f64>() {
        Ok(latitude) => latitude,
        Err(e) => return Err(e.to_string()),
    };

    let longitude = &caps["longitude"];
    let longitude = match longitude.parse::<f64>() {
        Ok(longitude) => longitude,
        Err(e) => return Err(e.to_string()),
    };

    let mut altitude: Option<f64> = None;
    if has_altitude {
        altitude = match &caps["altitude"].parse::<f64>() {
            Ok(altitude) => Some(*altitude),
            Err(e) => return Err(e.to_string()),
        };
    }

    let mut uncertainty: Option<f64> = None;
    if has_uncertainty {
        uncertainty = match &caps["uncertainty"].parse::<f64>() {
            Ok(uncertainty) => Some(*uncertainty),
            Err(e) => return Err(e.to_string()),
        };
    }

    Ok(GeoCoordinates {
        latitude: latitude,
        longitude: longitude,
        altitude: altitude,
        uncertainty: uncertainty,
    })
}

#[derive(Debug, PartialEq, Eq)]
pub enum OsmType {
    Node,
    Way,
    Relation,
}

#[derive(Debug, PartialEq, Eq)]
pub struct OsmObject {
    pub type_: OsmType,
    pub id: u64,
    pub revision: Option<u64>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Osm {
    Ok(OsmObject),
    Other(String),
}

impl<'de> Deserialize<'de> for Osm {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match parse_osm(s.clone()) {
            Ok(osm_coordinates) => Ok(Osm::Ok(osm_coordinates)),
            Err(_) => Ok(Osm::Other(s)),
        }
    }
}

fn parse_osm(s: String) -> Result<OsmObject, String> {
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
        None => return Err("wrong pattern".to_string()),
    };

    let type_ = match &caps["type_"] {
        "N" => OsmType::Node,
        "W" => OsmType::Way,
        "R" => OsmType::Relation,
        _ => return Err("something went wrong".to_string()),
    };

    let id = match &caps["id"].parse::<u64>() {
        Ok(id) => *id,
        Err(e) => return Err(e.to_string()),
    };

    let mut revision: Option<u64> = None;
    if has_revision {
        revision = match &caps["revision"].parse::<u64>() {
            Ok(revision) => Some(*revision),
            Err(e) => return Err(e.to_string()),
        };
    }

    Ok(OsmObject {
        type_: type_,
        id: id,
        revision: revision,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_geo() {
        pretty_assertions::assert_eq!(
            parse_geo("geo:37.786971,-122.399677".to_string()),
            Ok(GeoCoordinates {
                latitude: 37.786971,
                longitude: -122.399677,
                altitude: None,
                uncertainty: None,
            })
        );

        pretty_assertions::assert_eq!(
            parse_geo("geo:37.786971,-122.399677,250".to_string()),
            Ok(GeoCoordinates {
                latitude: 37.786971,
                longitude: -122.399677,
                altitude: Some(250.0),
                uncertainty: None,
            })
        );

        pretty_assertions::assert_eq!(
            parse_geo("geo:37.786971,-122.399677;u=350".to_string()),
            Ok(GeoCoordinates {
                latitude: 37.786971,
                longitude: -122.399677,
                altitude: None,
                uncertainty: Some(350.0),
            })
        );

        pretty_assertions::assert_eq!(
            parse_geo("geo:37.786971,-122.399677,250;u=350".to_string()),
            Ok(GeoCoordinates {
                latitude: 37.786971,
                longitude: -122.399677,
                altitude: Some(250.0),
                uncertainty: Some(350.0),
            })
        );

        pretty_assertions::assert_eq!(
            parse_geo("geo:37.786971,-122.399677,250,u=350".to_string()),
            Err("should not have more than two commas".to_string()),
        );
    }

    #[test]
    fn test_parse_osm() {
        pretty_assertions::assert_eq!(
            parse_osm("R148838".to_string()),
            Ok(OsmObject {
                type_: OsmType::Relation,
                id: 148838,
                revision: None,
            })
        );

        pretty_assertions::assert_eq!(
            parse_osm("W5013364".to_string()),
            Ok(OsmObject {
                type_: OsmType::Way,
                id: 5013364,
                revision: None,
            })
        );

        pretty_assertions::assert_eq!(
            parse_osm("R7444#188".to_string()),
            Ok(OsmObject {
                type_: OsmType::Relation,
                id: 7444,
                revision: Some(188),
            })
        );

        pretty_assertions::assert_eq!(
            parse_osm("7444#188".to_string()),
            Err("wrong pattern".to_string()),
        );
    }
}

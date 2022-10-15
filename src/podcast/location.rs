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
    Geo(GeoCoordinates),
    Other(String),
}

pub fn option_geo<'de, D>(deserializer: D) -> Result<Option<Geo>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = match String::deserialize(deserializer) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match parse_geo(s.clone()) {
        Ok(geo_coordinates) => Ok(Some(Geo::Geo(geo_coordinates))),
        Err(_) => Ok(Some(Geo::Other(s))),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_geo() {
        assert_eq!(
            parse_geo("geo:37.786971,-122.399677".to_string()),
            Ok(GeoCoordinates {
                latitude: 37.786971,
                longitude: -122.399677,
                altitude: None,
                uncertainty: None,
            })
        );

        assert_eq!(
            parse_geo("geo:37.786971,-122.399677,250".to_string()),
            Ok(GeoCoordinates {
                latitude: 37.786971,
                longitude: -122.399677,
                altitude: Some(250.0),
                uncertainty: None,
            })
        );

        assert_eq!(
            parse_geo("geo:37.786971,-122.399677;u=350".to_string()),
            Ok(GeoCoordinates {
                latitude: 37.786971,
                longitude: -122.399677,
                altitude: None,
                uncertainty: Some(350.0),
            })
        );

        assert_eq!(
            parse_geo("geo:37.786971,-122.399677,250;u=350".to_string()),
            Ok(GeoCoordinates {
                latitude: 37.786971,
                longitude: -122.399677,
                altitude: Some(250.0),
                uncertainty: Some(350.0),
            })
        );

        assert_eq!(
            parse_geo("geo:37.786971,-122.399677,250,u=350".to_string()),
            Err("should not have more than two commas".to_string()),
        );
    }
}

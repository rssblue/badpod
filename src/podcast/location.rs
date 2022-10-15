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

    if !has_altitude && !has_uncertainty {
        let re = match regex::Regex::new(
            r"(?P<latitude>[+-]?([0-9]*[.])?[0-9]+),(?P<longitude>[+-]?([0-9]*[.])?[0-9]+)",
        ) {
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
        return Ok(GeoCoordinates {
            latitude: latitude,
            longitude: longitude,
            altitude: None,
            uncertainty: None,
        });
    }
    return Ok(GeoCoordinates {
        latitude: 0.0,
        longitude: 0.0,
        altitude: None,
        uncertainty: None,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_geo() {
        let geo_coordinates = parse_geo("geo:37.786971,-122.399677".to_string());
        assert_eq!(
            geo_coordinates,
            Ok(GeoCoordinates {
                latitude: 37.786971,
                longitude: -122.399677,
                altitude: None,
                uncertainty: None,
            })
        );
    }
}

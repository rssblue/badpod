use serde_enum_str::Deserialize_enum_str;

#[derive(Debug, Deserialize_enum_str, PartialEq, Eq)]
pub enum ItunesCategoryName {
    Arts,
    Business,
    Comedy,
    Education,
    Fiction,
    Government,
    History,
    #[serde(rename = "Health & Fitness")]
    HealthAndFitness,
    #[serde(rename = "Kids & Family")]
    KidsAndFamily,
    Leisure,
    Music,
    News,
    #[serde(rename = "Religion & Spirituality")]
    ReligionAndSpirituality,
    Science,
    #[serde(rename = "Society & Culture")]
    SocietyAndCulture,
    Sports,
    Technology,
    #[serde(rename = "True Crime")]
    TrueCrime,
    #[serde(rename = "TV & Film")]
    TVAndFilm,

    #[serde(other)]
    Other(String),
}

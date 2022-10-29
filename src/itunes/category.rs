use crate::utils;
use serde::{Deserialize, Deserializer};
use std::fmt;
use strum::EnumProperty;
use strum_macros::{EnumIter, EnumProperty};

/// Apple Podcasts podcast category names.
#[derive(Debug, PartialEq, Eq, EnumProperty, EnumIter)]
pub enum CategoryName {
    Arts,
    Business,
    Comedy,
    Education,
    Fiction,
    Government,
    History,
    #[strum(props(str = "Health & Fitness"))]
    HealthAndFitness,
    #[strum(props(str = "Kids & Family"))]
    KidsAndFamily,
    Leisure,
    Music,
    News,
    #[strum(props(str = "Religion & Spirituality"))]
    ReligionAndSpirituality,
    Science,
    #[strum(props(str = "Society & Culture"))]
    SocietyAndCulture,
    Sports,
    Technology,
    #[strum(props(str = "True Crime"))]
    TrueCrime,
    #[strum(props(str = "TV & Film"))]
    TvAndFilm,

    Other(String),
}

impl std::str::FromStr for CategoryName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match utils::from_str_exact(s) {
            Some(variant) => Ok(variant),
            None => Ok(Self::Other(s.to_string())),
        }
    }
}

impl fmt::Display for CategoryName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other(s) => write!(f, "{s}"),
            _ => match self.get_str("str") {
                Some(s) => write!(f, "{}", s),
                None => write!(f, "{:?}", self),
            },
        }
    }
}

impl<'de> Deserialize<'de> for CategoryName {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        utils::deserialize_using_from_str(d)
    }
}

/// Apple Podcasts podcast subcategory names.
#[derive(Debug, PartialEq, Eq, EnumProperty, EnumIter)]
pub enum SubcategoryName {
    Books,
    Design,
    #[strum(props(str = "Fashion & Beauty"))]
    FashionAndBeauty,
    Food,
    #[strum(props(str = "Performing Arts"))]
    PerformingArts,
    #[strum(props(str = "Visual Arts"))]
    VisualArts,
    Careers,
    Entrepreneurship,
    Investing,
    Management,
    Marketing,
    #[strum(props(str = "Non-Profit"))]
    NonProfit,
    #[strum(props(str = "Comedy Interviews"))]
    ComedyInterviews,
    Improv,
    #[strum(props(str = "Stand-Up"))]
    StandUp,
    Courses,
    #[strum(props(str = "How To"))]
    HowTo,
    #[strum(props(str = "Language Learning"))]
    LanguageLearning,
    #[strum(props(str = "Self Improvement"))]
    SelfImprovement,
    #[strum(props(str = "Comedy Fiction"))]
    ComedyFiction,
    Drama,
    #[strum(props(str = "Science Fiction"))]
    ScienceFiction,
    #[strum(props(str = "Alternative Health"))]
    AlternativeHealth,
    Fitness,
    Medicine,
    #[strum(props(str = "Mental Health"))]
    MentalHealth,
    Nutrition,
    Sexuality,
    #[strum(props(str = "Education for Kids"))]
    EducationForKids,
    Parenting,
    #[strum(props(str = "Pets & Animals"))]
    PetsAndAnimals,
    #[strum(props(str = "Stories for Kids"))]
    StoriesForKids,
    #[strum(props(str = "Animation & Manga"))]
    AnimationAndManga,
    Automotive,
    Aviation,
    Crafts,
    Games,
    Hobbies,
    #[strum(props(str = "Home & Garden"))]
    HomeAndGarden,
    #[strum(props(str = "Video Games"))]
    VideoGames,
    #[strum(props(str = "Music Commentary"))]
    MusicCommentary,
    #[strum(props(str = "Music History"))]
    MusicHistory,
    #[strum(props(str = "Music Interviews"))]
    MusicInterviews,
    #[strum(props(str = "Business News"))]
    BusinessNews,
    #[strum(props(str = "Daily News"))]
    DailyNews,
    #[strum(props(str = "Entertainment News"))]
    EntertainmentNews,
    #[strum(props(str = "News Commentary"))]
    NewsCommentary,
    Politics,
    #[strum(props(str = "Sports News"))]
    SportsNews,
    #[strum(props(str = "Tech News"))]
    TechNews,
    Buddhism,
    Christianity,
    Hinduism,
    Islam,
    Judaism,
    Religion,
    Spirituality,
    Astronomy,
    Chemistry,
    #[strum(props(str = "Earth Sciences"))]
    EarthSciences,
    #[strum(props(str = "Life Sciences"))]
    LifeSciences,
    Mathematics,
    #[strum(props(str = "Natural Sciences"))]
    NaturalSciences,
    Nature,
    Physics,
    #[strum(props(str = "Social Sciences"))]
    SocialSciences,
    Documentary,
    #[strum(props(str = "Personal Journals"))]
    PersonalJournals,
    Philosophy,
    #[strum(props(str = "Places & Travel"))]
    PlacesAndTravel,
    Relationships,
    Baseball,
    Basketball,
    Cricket,
    #[strum(props(str = "Fantasy Sports"))]
    FantasySports,
    Football,
    Golf,
    Hockey,
    Rugby,
    Running,
    Soccer,
    Swimming,
    Tennis,
    Volleyball,
    Wilderness,
    Wrestling,
    #[strum(props(str = "After Shows"))]
    AfterShows,
    #[strum(props(str = "Film History"))]
    FilmHistory,
    #[strum(props(str = "Film Interviews"))]
    FilmInterviews,
    #[strum(props(str = "Film Reviews"))]
    FilmReviews,
    #[strum(props(str = "TV Reviews"))]
    TvReviews,

    Other(String),
}

impl std::str::FromStr for SubcategoryName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match utils::from_str_exact(s) {
            Some(variant) => Ok(variant),
            None => Ok(Self::Other(s.to_string())),
        }
    }
}

impl fmt::Display for SubcategoryName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other(s) => write!(f, "{s}"),
            _ => match self.get_str("str") {
                Some(s) => write!(f, "{}", s),
                None => write!(f, "{:?}", self),
            },
        }
    }
}

impl<'de> Deserialize<'de> for SubcategoryName {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        utils::deserialize_using_from_str(d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt() {
        pretty_assertions::assert_eq!(format!("{}", CategoryName::Business), "Business");
        pretty_assertions::assert_eq!(format!("{}", CategoryName::TrueCrime), "True Crime");
        pretty_assertions::assert_eq!(
            format!("{}", CategoryName::HealthAndFitness),
            "Health & Fitness"
        );
        pretty_assertions::assert_eq!(
            format!("{}", CategoryName::Other("other-category".to_string())),
            "other-category"
        );
    }
}

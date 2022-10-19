use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

/// Apple Podcasts podcast category names.
#[derive(Debug, PartialEq, Eq, EnumString, Display)]
pub enum CategoryName {
    Arts,
    Business,
    Comedy,
    Education,
    Fiction,
    Government,
    History,
    #[strum(serialize = "Health & Fitness")]
    HealthAndFitness,
    #[strum(serialize = "Kids & Family")]
    KidsAndFamily,
    Leisure,
    Music,
    News,
    #[strum(serialize = "Religion & Spirituality")]
    ReligionAndSpirituality,
    Science,
    #[strum(serialize = "Society & Culture")]
    SocietyAndCulture,
    Sports,
    Technology,
    #[strum(serialize = "True Crime")]
    TrueCrime,
    #[strum(serialize = "TV & Film")]
    TvAndFilm,

    #[strum(disabled)]
    Other(String),
}

impl<'de> Deserialize<'de> for CategoryName {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match CategoryName::from_str(s.as_str()) {
            Ok(x) => Ok(x),
            Err(_) => Ok(CategoryName::Other(s)),
        }
    }
}

/// Apple Podcasts podcast subcategory names.
#[derive(Debug, PartialEq, Eq, EnumString, Display)]
pub enum SubcategoryName {
    Books,
    Design,
    #[strum(serialize = "Fashion & Beauty")]
    FashionAndBeauty,
    Food,
    #[strum(serialize = "Performing Arts")]
    PerformingArts,
    #[strum(serialize = "Visual Arts")]
    VisualArts,
    Careers,
    Entrepreneurship,
    Investing,
    Management,
    Marketing,
    #[strum(serialize = "Non-Profit")]
    NonProfit,
    #[strum(serialize = "Comedy Interviews")]
    ComedyInterviews,
    Improv,
    #[strum(serialize = "Stand-Up")]
    StandUp,
    Courses,
    #[strum(serialize = "How To")]
    HowTo,
    #[strum(serialize = "Language Learning")]
    LanguageLearning,
    #[strum(serialize = "Self Improvement")]
    SelfImprovement,
    #[strum(serialize = "Comedy Fiction")]
    ComedyFiction,
    Drama,
    #[strum(serialize = "Science Fiction")]
    ScienceFiction,
    #[strum(serialize = "Alternative Health")]
    AlternativeHealth,
    Fitness,
    Medicine,
    #[strum(serialize = "Mental Health")]
    MentalHealth,
    Nutrition,
    Sexuality,
    #[strum(serialize = "Education for Kids")]
    EducationForKids,
    Parenting,
    #[strum(serialize = "Pets & Animals")]
    PetsAndAnimals,
    #[strum(serialize = "Stories for Kids")]
    StoriesForKids,
    #[strum(serialize = "Animation & Manga")]
    AnimationAndManga,
    Automotive,
    Aviation,
    Crafts,
    Games,
    Hobbies,
    #[strum(serialize = "Home & Garden")]
    HomeAndGarden,
    #[strum(serialize = "Video Games")]
    VideoGames,
    #[strum(serialize = "Music Commentary")]
    MusicCommentary,
    #[strum(serialize = "Music History")]
    MusicHistory,
    #[strum(serialize = "Music Interviews")]
    MusicInterviews,
    #[strum(serialize = "Business News")]
    BusinessNews,
    #[strum(serialize = "Daily News")]
    DailyNews,
    #[strum(serialize = "Entertainment News")]
    EntertainmentNews,
    #[strum(serialize = "News Commentary")]
    NewsCommentary,
    Politics,
    #[strum(serialize = "Sports News")]
    SportsNews,
    #[strum(serialize = "Tech News")]
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
    #[strum(serialize = "Earth Sciences")]
    EarthSciences,
    #[strum(serialize = "Life Sciences")]
    LifeSciences,
    Mathematics,
    #[strum(serialize = "Natural Sciences")]
    NaturalSciences,
    Nature,
    Physics,
    #[strum(serialize = "Social Sciences")]
    SocialSciences,
    Documentary,
    #[strum(serialize = "Personal Journals")]
    PersonalJournals,
    Philosophy,
    #[strum(serialize = "Places & Travel")]
    PlacesAndTravel,
    Relationships,
    Baseball,
    Basketball,
    Cricket,
    #[strum(serialize = "Fantasy Sports")]
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
    #[strum(serialize = "After Shows")]
    AfterShows,
    #[strum(serialize = "Film History")]
    FilmHistory,
    #[strum(serialize = "Film Interviews")]
    FilmInterviews,
    #[strum(serialize = "Film Reviews")]
    FilmReviews,
    #[strum(serialize = "TV Reviews")]
    TvReviews,

    #[strum(disabled)]
    Other(String),
}

impl<'de> Deserialize<'de> for SubcategoryName {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match SubcategoryName::from_str(s.as_str()) {
            Ok(x) => Ok(x),
            Err(_) => Ok(SubcategoryName::Other(s)),
        }
    }
}

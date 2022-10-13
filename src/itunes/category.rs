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

#[derive(Debug, Deserialize_enum_str, PartialEq, Eq)]
pub enum ItunesSubcategoryName {
    Books,
    Design,
    #[serde(rename = "Fashion & Beauty")]
    FashionAndBeauty,
    Food,
    #[serde(rename = "Performing Arts")]
    PerformingArts,
    #[serde(rename = "Visual Arts")]
    VisualArts,
    Careers,
    Entrepreneurship,
    Investing,
    Management,
    Marketing,
    #[serde(rename = "Non-Profit")]
    NonProfit,
    #[serde(rename = "Comedy Interviews")]
    ComedyInterviews,
    Improv,
    #[serde(rename = "Stand-Up")]
    StandUp,
    Courses,
    #[serde(rename = "How To")]
    HowTo,
    #[serde(rename = "Language Learning")]
    LanguageLearning,
    #[serde(rename = "Self Improvement")]
    SelfImprovement,
    #[serde(rename = "Comedy Fiction")]
    ComedyFiction,
    Drama,
    #[serde(rename = "Science Fiction")]
    ScienceFiction,
    #[serde(rename = "Alternative Health")]
    AlternativeHealth,
    Fitness,
    Medicine,
    #[serde(rename = "Mental Health")]
    MentalHealth,
    Nutrition,
    Sexuality,
    #[serde(rename = "Education for Kids")]
    EducationForKids,
    Parenting,
    #[serde(rename = "Pets & Animals")]
    PetsAndAnimals,
    #[serde(rename = "Stories for Kids")]
    StoriesForKids,
    #[serde(rename = "Animation & Manga")]
    AnimationAndManga,
    Automotive,
    Aviation,
    Crafts,
    Games,
    Hobbies,
    #[serde(rename = "Home & Garden")]
    HomeAndGarden,
    #[serde(rename = "Video Games")]
    VideoGames,
    #[serde(rename = "Music Commentary")]
    MusicCommentary,
    #[serde(rename = "Music History")]
    MusicHistory,
    #[serde(rename = "Music Interviews")]
    MusicInterviews,
    #[serde(rename = "Business News")]
    BusinessNews,
    #[serde(rename = "Daily News")]
    DailyNews,
    #[serde(rename = "Entertainment News")]
    EntertainmentNews,
    #[serde(rename = "News Commentary")]
    NewsCommentary,
    Politics,
    #[serde(rename = "Sports News")]
    SportsNews,
    #[serde(rename = "Tech News")]
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
    #[serde(rename = "Earth Sciences")]
    EarthSciences,
    #[serde(rename = "Life Sciences")]
    LifeSciences,
    Mathematics,
    #[serde(rename = "Natural Sciences")]
    NaturalSciences,
    Nature,
    Physics,
    #[serde(rename = "Social Sciences")]
    SocialSciences,
    Documentary,
    #[serde(rename = "Personal Journals")]
    PersonalJournals,
    Philosophy,
    #[serde(rename = "Places & Travel")]
    PlacesAndTravel,
    Relationships,
    Baseball,
    Basketball,
    Cricket,
    #[serde(rename = "Fantasy Sports")]
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
    #[serde(rename = "After Shows")]
    AfterShows,
    #[serde(rename = "Film History")]
    FilmHistory,
    #[serde(rename = "Film Interviews")]
    FilmInterviews,
    #[serde(rename = "Film Reviews")]
    FilmReviews,
    #[serde(rename = "TV Reviews")]
    TVReviews,

    #[serde(other)]
    Other(String),
}

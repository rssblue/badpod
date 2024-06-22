use crate::utils;
use crate::Other;
use std::fmt;
use std::str::FromStr;
use strum::{EnumIter, EnumProperty};

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

    Other(Other),
}

impl std::str::FromStr for CategoryName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match utils::from_str_exact(s) {
            Some(variant) => Ok(variant),
            None => Ok(Self::Other((
                s.to_string(),
                "unrecognized category name".to_string(),
            ))),
        }
    }
}

impl fmt::Display for CategoryName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other((s, _)) => write!(f, "{s}"),
            _ => match self.get_str("str") {
                Some(s) => write!(f, "{s}"),
                None => write!(f, "{self:?}"),
            },
        }
    }
}

impl CategoryName {
    pub fn parse(s: &str) -> Self {
        match Self::from_str(s) {
            Ok(variant) => variant,
            Err(e) => Self::Other((s.to_string(), e)),
        }
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

    Other(Other),
}

impl std::str::FromStr for SubcategoryName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match utils::from_str_exact(s) {
            Some(variant) => Ok(variant),
            None => Ok(Self::Other((
                s.to_string(),
                "unrecognized subcategory name".to_string(),
            ))),
        }
    }
}

impl fmt::Display for SubcategoryName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other((s, _)) => write!(f, "{s}"),
            _ => match self.get_str("str") {
                Some(s) => write!(f, "{s}"),
                None => write!(f, "{self:?}"),
            },
        }
    }
}

impl SubcategoryName {
    pub fn parse(s: &str, category_name: &CategoryName) -> Self {
        let subcategory = match Self::from_str(s) {
            Ok(variant) => variant,
            Err(e) => Self::Other((s.to_string(), e)),
        };

        let allowed_subcategories = match category_name {
            CategoryName::Arts => vec![
                SubcategoryName::Books,
                SubcategoryName::Design,
                SubcategoryName::FashionAndBeauty,
                SubcategoryName::Food,
                SubcategoryName::PerformingArts,
                SubcategoryName::VisualArts,
            ],
            CategoryName::Business => vec![
                SubcategoryName::Careers,
                SubcategoryName::Entrepreneurship,
                SubcategoryName::Investing,
                SubcategoryName::Management,
                SubcategoryName::Marketing,
                SubcategoryName::NonProfit,
            ],
            CategoryName::Comedy => vec![
                SubcategoryName::ComedyInterviews,
                SubcategoryName::Improv,
                SubcategoryName::StandUp,
            ],
            CategoryName::Education => vec![
                SubcategoryName::Courses,
                SubcategoryName::HowTo,
                SubcategoryName::LanguageLearning,
                SubcategoryName::SelfImprovement,
            ],
            CategoryName::Fiction => vec![
                SubcategoryName::ComedyFiction,
                SubcategoryName::Drama,
                SubcategoryName::ScienceFiction,
            ],
            CategoryName::Government => vec![],
            CategoryName::History => vec![],
            CategoryName::HealthAndFitness => vec![
                SubcategoryName::AlternativeHealth,
                SubcategoryName::Fitness,
                SubcategoryName::Medicine,
                SubcategoryName::MentalHealth,
                SubcategoryName::Nutrition,
            ],
            CategoryName::KidsAndFamily => vec![
                SubcategoryName::EducationForKids,
                SubcategoryName::Parenting,
                SubcategoryName::PetsAndAnimals,
                SubcategoryName::StoriesForKids,
            ],
            CategoryName::Leisure => vec![
                SubcategoryName::AnimationAndManga,
                SubcategoryName::Automotive,
                SubcategoryName::Aviation,
                SubcategoryName::Crafts,
                SubcategoryName::Games,
                SubcategoryName::Hobbies,
                SubcategoryName::HomeAndGarden,
                SubcategoryName::VideoGames,
            ],
            CategoryName::Music => vec![
                SubcategoryName::MusicCommentary,
                SubcategoryName::MusicHistory,
                SubcategoryName::MusicInterviews,
            ],
            CategoryName::News => vec![
                SubcategoryName::BusinessNews,
                SubcategoryName::DailyNews,
                SubcategoryName::EntertainmentNews,
                SubcategoryName::NewsCommentary,
                SubcategoryName::Politics,
                SubcategoryName::SportsNews,
                SubcategoryName::TechNews,
            ],
            CategoryName::ReligionAndSpirituality => vec![
                SubcategoryName::Buddhism,
                SubcategoryName::Christianity,
                SubcategoryName::Hinduism,
                SubcategoryName::Islam,
                SubcategoryName::Judaism,
                SubcategoryName::Religion,
                SubcategoryName::Spirituality,
            ],
            CategoryName::Science => vec![
                SubcategoryName::Astronomy,
                SubcategoryName::Chemistry,
                SubcategoryName::EarthSciences,
                SubcategoryName::LifeSciences,
                SubcategoryName::Mathematics,
                SubcategoryName::NaturalSciences,
                SubcategoryName::Nature,
                SubcategoryName::Physics,
                SubcategoryName::SocialSciences,
            ],
            CategoryName::SocietyAndCulture => vec![
                SubcategoryName::Documentary,
                SubcategoryName::PersonalJournals,
                SubcategoryName::Philosophy,
                SubcategoryName::PlacesAndTravel,
                SubcategoryName::Relationships,
            ],
            CategoryName::Sports => vec![
                SubcategoryName::Baseball,
                SubcategoryName::Basketball,
                SubcategoryName::Cricket,
                SubcategoryName::FantasySports,
                SubcategoryName::Football,
                SubcategoryName::Golf,
                SubcategoryName::Hockey,
                SubcategoryName::Rugby,
                SubcategoryName::Running,
                SubcategoryName::Soccer,
                SubcategoryName::Swimming,
                SubcategoryName::Tennis,
                SubcategoryName::Volleyball,
                SubcategoryName::Wilderness,
                SubcategoryName::Wrestling,
            ],
            CategoryName::Technology => vec![],
            CategoryName::TrueCrime => vec![],
            CategoryName::TvAndFilm => vec![
                SubcategoryName::AfterShows,
                SubcategoryName::FilmHistory,
                SubcategoryName::FilmInterviews,
                SubcategoryName::FilmReviews,
                SubcategoryName::TvReviews,
            ],
            CategoryName::Other(_) => vec![],
        };

        if !allowed_subcategories.contains(&subcategory) {
            let mut msg = format!(
                "subcategory \"{subcategory}\" is not allowed for category \"{category_name}\""
            );
            if !allowed_subcategories.is_empty() {
                msg.push_str(&format!(
                    "; valid subcategories: {}",
                    allowed_subcategories
                        .iter()
                        .map(|s| format!("\"{s}\""))
                        .collect::<Vec<String>>()
                        .join(", ")
                ));
            } else {
                msg.push_str("; no subcategories are allowed for this category");
            }
            return Self::Other((s.to_string(), msg));
        }

        subcategory
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
            format!(
                "{}",
                CategoryName::Other((
                    "other-category".to_string(),
                    "unrecognized category name".to_string()
                ))
            ),
            "other-category"
        );
    }
}

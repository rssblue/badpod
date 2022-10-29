use serde::{Deserialize, Deserializer};
use std::fmt;
use strum::{EnumProperty, IntoEnumIterator};
use strum_macros::{EnumIter, EnumProperty};

/// Group (as defined by [Podcast Taxonomy Project](https://podcasttaxonomy.com/)) of [Person](crate::podcast::Person).
#[derive(Debug, PartialEq, Eq, EnumProperty, EnumIter)]
pub enum PersonGroup {
    #[strum(props(str = "creative direction"))]
    CreativeDirection,
    #[strum(props(str = "cast"))]
    Cast,
    #[strum(props(str = "writing"))]
    Writing,
    #[strum(props(str = "audio production"))]
    AudioProduction,
    #[strum(props(str = "audio post-production"))]
    AudioPostProduction,
    #[strum(props(str = "administration"))]
    Administration,
    #[strum(props(str = "visuals"))]
    Visuals,
    #[strum(props(str = "community"))]
    Community,
    #[strum(props(str = "misc"))]
    Misc,
    #[strum(props(str = "video production"))]
    VideoProduction,
    #[strum(props(str = "video post-production"))]
    VideoPostProduction,

    Other(String),
}

impl fmt::Display for PersonGroup {
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

impl<'de> Deserialize<'de> for PersonGroup {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        let s_lowercase = s.to_lowercase();

        for variant in Self::iter() {
            if format!("{variant}") == s_lowercase {
                return Ok(variant);
            };
        }

        Ok(Self::Other(s))
    }
}

/// Role (as defined by [Podcast Taxonomy Project](https://podcasttaxonomy.com/)) of [Person](crate::podcast::Person).
#[derive(Debug, PartialEq, Eq, EnumProperty, EnumIter)]
pub enum PersonRole {
    #[strum(props(str = "director"))]
    Director,
    #[strum(props(str = "assistant director"))]
    AssistantDirector,
    #[strum(props(str = "executive producer"))]
    ExecutiveProducer,
    #[strum(props(str = "senior producer"))]
    SeniorProducer,
    #[strum(props(str = "producer"))]
    Producer,
    #[strum(props(str = "associate producer"))]
    AssociateProducer,
    #[strum(props(str = "development producer"))]
    DevelopmentProducer,
    #[strum(props(str = "creative director"))]
    CreativeDirector,
    #[strum(props(str = "host"))]
    Host,
    #[strum(props(str = "co-host"))]
    CoHost,
    #[strum(props(str = "guest host"))]
    GuestHost,
    #[strum(props(str = "guest"))]
    Guest,
    #[strum(props(str = "voice actor"))]
    VoiceActor,
    #[strum(props(str = "narrator"))]
    Narrator,
    #[strum(props(str = "announcer"))]
    Announcer,
    #[strum(props(str = "reporter"))]
    Reporter,
    #[strum(props(str = "author"))]
    Author,
    #[strum(props(str = "editorial director"))]
    EditorialDirector,
    #[strum(props(str = "co-writer"))]
    CoWriter,
    #[strum(props(str = "writer"))]
    Writer,
    #[strum(props(str = "songwriter"))]
    Songwriter,
    #[strum(props(str = "guest writer"))]
    GuestWriter,
    #[strum(props(str = "story editor"))]
    StoryEditor,
    #[strum(props(str = "managing editor"))]
    ManagingEditor,
    #[strum(props(str = "script editor"))]
    ScriptEditor,
    #[strum(props(str = "script coordinator"))]
    ScriptCoordinator,
    #[strum(props(str = "researcher"))]
    Researcher,
    #[strum(props(str = "editor"))]
    Editor,
    #[strum(props(str = "fact checker"))]
    FactChecker,
    #[strum(props(str = "translator"))]
    Translator,
    #[strum(props(str = "transcriber"))]
    Transcriber,
    #[strum(props(str = "logger"))]
    Logger,
    #[strum(props(str = "studio coordinator"))]
    StudioCoordinator,
    #[strum(props(str = "technical director"))]
    TechnicalDirector,
    #[strum(props(str = "technical manager"))]
    TechnicalManager,
    #[strum(props(str = "audio engineer"))]
    AudioEngineer,
    #[strum(props(str = "remote recording engineer"))]
    RemoteRecordingEngineer,
    #[strum(props(str = "post production engineer"))]
    PostProductionEngineer,
    #[strum(props(str = "audio editor"))]
    AudioEditor,
    #[strum(props(str = "sound designer"))]
    SoundDesigner,
    #[strum(props(str = "foley artist"))]
    FoleyArtist,
    #[strum(props(str = "composer"))]
    Composer,
    #[strum(props(str = "theme music"))]
    ThemeMusic,
    #[strum(props(str = "music production"))]
    MusicProduction,
    #[strum(props(str = "music contributor"))]
    MusicContributor,
    #[strum(props(str = "production coordinator"))]
    ProductionCoordinator,
    #[strum(props(str = "booking coordinator"))]
    BookingCoordinator,
    #[strum(props(str = "production assistant"))]
    ProductionAssistant,
    #[strum(props(str = "content manager"))]
    ContentManager,
    #[strum(props(str = "marketing manager"))]
    MarketingManager,
    #[strum(props(str = "sales representative"))]
    SalesRepresentative,
    #[strum(props(str = "sales manager"))]
    SalesManager,
    #[strum(props(str = "graphic designer"))]
    GraphicDesigner,
    #[strum(props(str = "cover art designer"))]
    CoverArtDesigner,
    #[strum(props(str = "social media manager"))]
    SocialMediaManager,
    #[strum(props(str = "consultant"))]
    Consultant,
    #[strum(props(str = "intern"))]
    Intern,
    #[strum(props(str = "camera operator"))]
    CameraOperator,
    #[strum(props(str = "lighting designer"))]
    LightingDesigner,
    #[strum(props(str = "camera grip"))]
    CameraGrip,
    #[strum(props(str = "assistant camera"))]
    AssistantCamera,
    #[strum(props(str = "assistant editor"))]
    AssistantEditor,

    Other(String),
}

impl fmt::Display for PersonRole {
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

impl<'de> Deserialize<'de> for PersonRole {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        let s_lowercase = s.to_lowercase();

        for variant in Self::iter() {
            if format!("{variant}") == s_lowercase {
                return Ok(variant);
            };
        }

        Ok(Self::Other(s))
    }
}

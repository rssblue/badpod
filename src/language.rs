use crate::utils;
use serde::{Deserialize, Deserializer};
use strum::{EnumProperty, IntoEnumIterator};
use strum_macros::{Display, EnumIter, EnumProperty, EnumString};

/// Used for deserializing languages.
///
/// Language codes taken from <https://www.rssboard.org/rss-language-codes> and
/// <https://www.loc.gov/standards/iso639-2/php/code_list.php>.
#[derive(Debug, PartialEq, Eq, EnumProperty, EnumIter)]
pub enum Language {
    #[strum(props(str = "aa"))]
    Afar,
    #[strum(props(str = "ab"))]
    Abkhazian,
    #[strum(props(str = "ae"))]
    Avestan,
    #[strum(props(str = "af"))]
    Afrikaans,
    #[strum(props(str = "ak"))]
    Akan,
    #[strum(props(str = "am"))]
    Amharic,
    #[strum(props(str = "an"))]
    Aragonese,
    #[strum(props(str = "ar"))]
    Arabic,
    #[strum(props(str = "as"))]
    Assamese,
    #[strum(props(str = "av"))]
    Avaric,
    #[strum(props(str = "ay"))]
    Aymara,
    #[strum(props(str = "az"))]
    Azerbaijani,
    #[strum(props(str = "ba"))]
    Bashkir,
    #[strum(props(str = "be"))]
    Belarusian,
    #[strum(props(str = "bg"))]
    Bulgarian,
    #[strum(props(str = "bh"))]
    BihariLanguages,
    #[strum(props(str = "bi"))]
    Bislama,
    #[strum(props(str = "bm"))]
    Bambara,
    #[strum(props(str = "bn"))]
    Bengali,
    #[strum(props(str = "bo"))]
    Tibetan,
    #[strum(props(str = "br"))]
    Breton,
    #[strum(props(str = "bs"))]
    Bosnian,
    #[strum(props(str = "ca"))]
    Catalan,
    #[strum(props(str = "ce"))]
    Chechen,
    #[strum(props(str = "ch"))]
    Chamorro,
    #[strum(props(str = "co"))]
    Corsican,
    #[strum(props(str = "cr"))]
    Cree,
    #[strum(props(str = "cs"))]
    Czech,
    #[strum(props(str = "cu"))]
    SlavicChurch,
    #[strum(props(str = "cv"))]
    Chuvash,
    #[strum(props(str = "cy"))]
    Welsh,
    #[strum(props(str = "da"))]
    Danish,
    German(LanguageGerman),
    #[strum(props(str = "dv"))]
    Divehi,
    #[strum(props(str = "dz"))]
    Dzongkha,
    #[strum(props(str = "ee"))]
    Ewe,
    #[strum(props(str = "el"))]
    Greek,
    English(LanguageEnglish),
    #[strum(props(str = "eo"))]
    Esperanto,
    Spanish(LanguageSpanish),
    #[strum(props(str = "et"))]
    Estonian,
    #[strum(props(str = "eu"))]
    Basque,
    #[strum(props(str = "fa"))]
    Persian,
    #[strum(props(str = "ff"))]
    Fulah,
    #[strum(props(str = "fi"))]
    Finnish,
    #[strum(props(str = "fj"))]
    Fijian,
    #[strum(props(str = "fo"))]
    Faroese,
    French(LanguageFrench),
    #[strum(props(str = "fy"))]
    WesternFrisian,
    #[strum(props(str = "ga"))]
    Irish,
    #[strum(props(str = "gd"))]
    Gaelic,
    #[strum(props(str = "gl"))]
    Galician,
    #[strum(props(str = "gn"))]
    Guarani,
    #[strum(props(str = "gu"))]
    Gujarati,
    #[strum(props(str = "gv"))]
    Manx,
    #[strum(props(str = "ha"))]
    Hausa,
    #[strum(props(str = "haw"))]
    Hawaiian,
    #[strum(props(str = "he"))]
    Hebrew,
    #[strum(props(str = "hi"))]
    Hindi,
    #[strum(props(str = "ho"))]
    HiriMotu,
    #[strum(props(str = "hr"))]
    Croatian,
    #[strum(props(str = "ht"))]
    Haitian,
    #[strum(props(str = "hu"))]
    Hungarian,
    #[strum(props(str = "hy"))]
    Armenian,
    #[strum(props(str = "hz"))]
    Herero,
    #[strum(props(str = "ia"))]
    Interlingua,
    #[strum(props(str = "ie"))]
    Interlingue,
    #[strum(props(str = "ig"))]
    Igbo,
    #[strum(props(str = "ii"))]
    SichuanYi,
    #[strum(props(str = "ik"))]
    Inupiaq,
    #[strum(props(str = "in"))]
    Indonesian,
    #[strum(props(str = "io"))]
    Ido,
    #[strum(props(str = "is"))]
    Icelandic,
    Italian(LanguageItalian),
    #[strum(props(str = "iu"))]
    Inuktitut,
    #[strum(props(str = "ja"))]
    Japanese,
    #[strum(props(str = "jv"))]
    Javanese,
    #[strum(props(str = "ka"))]
    Georgian,
    #[strum(props(str = "kg"))]
    Kongo,
    #[strum(props(str = "ki"))]
    KikuyuGikuyu,
    #[strum(props(str = "kj"))]
    Kuanyama,
    #[strum(props(str = "kk"))]
    Kazakh,
    #[strum(props(str = "kl"))]
    Kalaallisut,
    #[strum(props(str = "km"))]
    CentralKhmer,
    #[strum(props(str = "kn"))]
    Kannada,
    #[strum(props(str = "ko"))]
    Korean,
    #[strum(props(str = "kr"))]
    Kanuri,
    #[strum(props(str = "ks"))]
    Kashmiri,
    #[strum(props(str = "ku"))]
    Kurdish,
    #[strum(props(str = "kv"))]
    Komi,
    #[strum(props(str = "kw"))]
    Cornish,
    #[strum(props(str = "ky"))]
    KirghizKyrgyz,
    #[strum(props(str = "la"))]
    Latin,
    #[strum(props(str = "lb"))]
    Luxembourgish,
    #[strum(props(str = "lg"))]
    Ganda,
    #[strum(props(str = "li"))]
    Limburgan,
    #[strum(props(str = "ln"))]
    Lingala,
    #[strum(props(str = "lo"))]
    Lao,
    #[strum(props(str = "lt"))]
    Lithuanian,
    #[strum(props(str = "lu"))]
    LubaKatanga,
    #[strum(props(str = "lv"))]
    Latvian,
    #[strum(props(str = "mg"))]
    Malagasy,
    #[strum(props(str = "mh"))]
    Marshallese,
    #[strum(props(str = "mi"))]
    Maori,
    #[strum(props(str = "mk"))]
    Macedonian,
    #[strum(props(str = "ml"))]
    Malayalam,
    #[strum(props(str = "mn"))]
    Mongolian,
    #[strum(props(str = "mr"))]
    Marathi,
    #[strum(props(str = "ms"))]
    Malay,
    #[strum(props(str = "mt"))]
    Maltese,
    #[strum(props(str = "my"))]
    Burmese,
    #[strum(props(str = "na"))]
    Nauru,
    #[strum(props(str = "nb"))]
    NorwegianBokmal,
    #[strum(props(str = "ne"))]
    Nepali,
    #[strum(props(str = "ng"))]
    Ndonga,
    Dutch(LanguageDutch),
    #[strum(props(str = "nn"))]
    NorwegianNynorsk,
    #[strum(props(str = "no"))]
    Norwegian,
    #[strum(props(str = "nr"))]
    Ndebele,
    #[strum(props(str = "nv"))]
    Navajo,
    #[strum(props(str = "ny"))]
    Chichewa,
    #[strum(props(str = "oc"))]
    Occitan,
    #[strum(props(str = "oj"))]
    Ojibwa,
    #[strum(props(str = "om"))]
    Oromo,
    #[strum(props(str = "or"))]
    Oriya,
    #[strum(props(str = "os"))]
    Ossetian,
    #[strum(props(str = "pa"))]
    Panjabi,
    #[strum(props(str = "pi"))]
    Pali,
    #[strum(props(str = "pl"))]
    Polish,
    #[strum(props(str = "ps"))]
    Pushto,
    Portuguese(LanguagePortugese),
    #[strum(props(str = "qu"))]
    Quechua,
    #[strum(props(str = "rm"))]
    Romansh,
    #[strum(props(str = "rn"))]
    Rundi,
    Romanian(LanguageRomanian),
    Russian(LanguageRussian),
    #[strum(props(str = "rw"))]
    Kinyarwanda,
    #[strum(props(str = "sa"))]
    Sanskrit,
    #[strum(props(str = "sc"))]
    Sardinian,
    #[strum(props(str = "sd"))]
    Sindhi,
    #[strum(props(str = "se"))]
    NorthernSami,
    #[strum(props(str = "sg"))]
    Sango,
    #[strum(props(str = "si"))]
    Sinhala,
    #[strum(props(str = "sk"))]
    Slovak,
    #[strum(props(str = "sl"))]
    Slovenian,
    #[strum(props(str = "sm"))]
    Samoan,
    #[strum(props(str = "sn"))]
    Shona,
    #[strum(props(str = "so"))]
    Somali,
    #[strum(props(str = "sq"))]
    Albanian,
    #[strum(props(str = "sr"))]
    Serbian,
    #[strum(props(str = "ss"))]
    Swati,
    #[strum(props(str = "st"))]
    SothoSouthern,
    #[strum(props(str = "su"))]
    Sundanese,
    Swedish(LanguageSwedish),
    #[strum(props(str = "sw"))]
    Swahili,
    #[strum(props(str = "ta"))]
    Tamil,
    #[strum(props(str = "te"))]
    Telugu,
    #[strum(props(str = "tg"))]
    Tajik,
    #[strum(props(str = "th"))]
    Thai,
    #[strum(props(str = "ti"))]
    Tigrinya,
    #[strum(props(str = "tk"))]
    Turkmen,
    #[strum(props(str = "tl"))]
    Tagalog,
    #[strum(props(str = "tn"))]
    Tswana,
    #[strum(props(str = "to"))]
    Tonga,
    #[strum(props(str = "tr"))]
    Turkish,
    #[strum(props(str = "ts"))]
    Tsonga,
    #[strum(props(str = "tt"))]
    Tatar,
    #[strum(props(str = "tw"))]
    Twi,
    #[strum(props(str = "ty"))]
    Tahitian,
    #[strum(props(str = "ug"))]
    Uighur,
    #[strum(props(str = "uk"))]
    Ukrainian,
    #[strum(props(str = "ur"))]
    Urdu,
    #[strum(props(str = "uz"))]
    Uzbek,
    #[strum(props(str = "ve"))]
    Venda,
    #[strum(props(str = "vi"))]
    Vietnamese,
    #[strum(props(str = "vo"))]
    Volapük,
    #[strum(props(str = "wa"))]
    Walloon,
    #[strum(props(str = "wo"))]
    Wolof,
    #[strum(props(str = "xh"))]
    Xhosa,
    #[strum(props(str = "yi"))]
    Yiddish,
    #[strum(props(str = "yo"))]
    Yoruba,
    #[strum(props(str = "za"))]
    Zhuang,
    Chinese(LanguageChinese),
    #[strum(props(str = "zu"))]
    Zulu,

    Other(String),
}

impl std::str::FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Case insensitive.
        let s = s.to_lowercase();

        if s.starts_with("de") {
            return match LanguageGerman::from_str(s.as_str()) {
                Ok(region) => Ok(Language::German(region)),
                Err(_) => Ok(Language::Other(s)),
            };
        }

        if s.starts_with("en") {
            return match LanguageEnglish::from_str(s.as_str()) {
                Ok(region) => Ok(Language::English(region)),
                Err(_) => Ok(Language::Other(s)),
            };
        }

        if s.starts_with("es") {
            return match LanguageSpanish::from_str(s.as_str()) {
                Ok(region) => Ok(Language::Spanish(region)),
                Err(_) => Ok(Language::Other(s)),
            };
        }

        if s.starts_with("fr") {
            return match LanguageFrench::from_str(s.as_str()) {
                Ok(region) => Ok(Language::French(region)),
                Err(_) => Ok(Language::Other(s)),
            };
        }

        if s.starts_with("it") {
            return match LanguageItalian::from_str(s.as_str()) {
                Ok(region) => Ok(Language::Italian(region)),
                Err(_) => Ok(Language::Other(s)),
            };
        }

        if s.starts_with("nl") {
            return match LanguageDutch::from_str(s.as_str()) {
                Ok(region) => Ok(Language::Dutch(region)),
                Err(_) => Ok(Language::Other(s)),
            };
        }

        if s.starts_with("pt") {
            return match LanguagePortugese::from_str(s.as_str()) {
                Ok(region) => Ok(Language::Portuguese(region)),
                Err(_) => Ok(Language::Other(s)),
            };
        }

        if s.starts_with("ro") {
            return match LanguageRomanian::from_str(s.as_str()) {
                Ok(region) => Ok(Language::Romanian(region)),
                Err(_) => Ok(Language::Other(s)),
            };
        }

        if s.starts_with("ru") {
            return match LanguageRussian::from_str(s.as_str()) {
                Ok(region) => Ok(Language::Russian(region)),
                Err(_) => Ok(Language::Other(s)),
            };
        }

        if s.starts_with("sv") {
            return match LanguageSwedish::from_str(s.as_str()) {
                Ok(region) => Ok(Language::Swedish(region)),
                Err(_) => Ok(Language::Other(s)),
            };
        }

        if s.starts_with("zh") {
            return match LanguageChinese::from_str(s.as_str()) {
                Ok(region) => Ok(Language::Chinese(region)),
                Err(_) => Ok(Language::Other(s)),
            };
        }

        for variant in Self::iter() {
            if format!("{variant}") == s {
                return Ok(variant);
            };
        }

        Ok(Language::Other(s))
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Other(s) => write!(f, "{s}"),
            Language::Chinese(region) => write!(f, "{region}"),
            Language::Dutch(region) => write!(f, "{region}"),
            Language::English(region) => write!(f, "{region}"),
            Language::French(region) => write!(f, "{region}"),
            Language::German(region) => write!(f, "{region}"),
            Language::Italian(region) => write!(f, "{region}"),
            Language::Portuguese(region) => write!(f, "{region}"),
            Language::Romanian(region) => write!(f, "{region}"),
            Language::Russian(region) => write!(f, "{region}"),
            Language::Spanish(region) => write!(f, "{region}"),
            Language::Swedish(region) => write!(f, "{region}"),
            _ => match self.get_str("str") {
                Some(s) => write!(f, "{}", s),
                None => Err("property \"str\" not found").map_err(|_| std::fmt::Error),
            },
        }
    }
}

impl<'de> Deserialize<'de> for Language {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        utils::deserialize_using_from_str(d)
    }
}

/// German language regions.
#[derive(Debug, PartialEq, Eq, EnumString, Display, Default)]
pub enum LanguageGerman {
    #[default]
    #[strum(serialize = "de")]
    Default,
    #[strum(serialize = "de-at")]
    Austria,
    #[strum(serialize = "de-ch")]
    Switzerland,
    #[strum(serialize = "de-de")]
    Germany,
    #[strum(serialize = "de-li")]
    Liechtenstein,
    #[strum(serialize = "de-lu")]
    Luxembourg,
}

/// English language regions.
#[derive(Debug, PartialEq, Eq, EnumString, Display, Default)]
pub enum LanguageEnglish {
    #[default]
    #[strum(serialize = "en")]
    Default,
    #[strum(serialize = "en-au")]
    Australia,
    #[strum(serialize = "en-bz")]
    Belize,
    #[strum(serialize = "en-ca")]
    Canada,
    #[strum(serialize = "en-gb")]
    UnitedKingdom,
    #[strum(serialize = "en-ie")]
    Ireland,
    #[strum(serialize = "en-jm")]
    Jamaica,
    #[strum(serialize = "en-nz")]
    NewZealand,
    #[strum(serialize = "en-ph")]
    Phillipines,
    #[strum(serialize = "en-tt")]
    Trinidad,
    #[strum(serialize = "en-us")]
    UnitedStates,
    #[strum(serialize = "en-za")]
    SouthAfrica,
    #[strum(serialize = "en-zw")]
    Zimbabwe,
}

/// Spanish language regions.
#[derive(Debug, PartialEq, Eq, EnumString, Display, Default)]
pub enum LanguageSpanish {
    #[default]
    #[strum(serialize = "es")]
    Default,
    #[strum(serialize = "es-ar")]
    Argentina,
    #[strum(serialize = "es-bo")]
    Bolivia,
    #[strum(serialize = "es-cl")]
    Chile,
    #[strum(serialize = "es-co")]
    Colombia,
    #[strum(serialize = "es-cr")]
    CostaRica,
    #[strum(serialize = "es-do")]
    DominicanRepublic,
    #[strum(serialize = "es-ec")]
    Ecuador,
    #[strum(serialize = "es-es")]
    Spain,
    #[strum(serialize = "es-gt")]
    Guatemala,
    #[strum(serialize = "es-hn")]
    Honduras,
    #[strum(serialize = "es-mx")]
    Mexico,
    #[strum(serialize = "es-ni")]
    Nicaragua,
    #[strum(serialize = "es-pa")]
    Panama,
    #[strum(serialize = "es-pe")]
    Peru,
    #[strum(serialize = "es-pr")]
    PuertoRico,
    #[strum(serialize = "es-py")]
    Paraguay,
    #[strum(serialize = "es-sv")]
    ElSalvador,
    #[strum(serialize = "es-uy")]
    Uruguay,
    #[strum(serialize = "es-ve")]
    Venezuela,
}

/// French language regions.
#[derive(Debug, PartialEq, Eq, EnumString, Display, Default)]
pub enum LanguageFrench {
    #[default]
    #[strum(serialize = "fr")]
    Default,
    #[strum(serialize = "fr-be")]
    Belgium,
    #[strum(serialize = "fr-ca")]
    Canada,
    #[strum(serialize = "fr-ch")]
    Switzerland,
    #[strum(serialize = "fr-fr")]
    France,
    #[strum(serialize = "fr-lu")]
    Luxembourg,
    #[strum(serialize = "fr-mc")]
    Monaco,
}

/// Italian language regions.
#[derive(Debug, PartialEq, Eq, EnumString, Display, Default)]
pub enum LanguageItalian {
    #[default]
    #[strum(serialize = "it")]
    Default,
    #[strum(serialize = "it-ch")]
    Switzerland,
    #[strum(serialize = "it-it")]
    Italy,
}

/// Dutch language regions.
#[derive(Debug, PartialEq, Eq, EnumString, Display, Default)]
pub enum LanguageDutch {
    #[default]
    #[strum(serialize = "nl")]
    Default,
    #[strum(serialize = "nl-be")]
    Belgium,
    #[strum(serialize = "nl-nl")]
    Netherlands,
}

/// Portuguese language regions.
#[derive(Debug, PartialEq, Eq, EnumString, Display, Default)]
pub enum LanguagePortugese {
    #[default]
    #[strum(serialize = "pt")]
    Default,
    #[strum(serialize = "pt-br")]
    Brazil,
    #[strum(serialize = "pt-pt")]
    Portugal,
}

/// Romanian language regions.
#[derive(Debug, PartialEq, Eq, EnumString, Display, Default)]
pub enum LanguageRomanian {
    #[default]
    #[strum(serialize = "ro")]
    Default,
    #[strum(serialize = "ro-mo")]
    Moldova,
    #[strum(serialize = "ro-ro")]
    Romania,
}

/// Russian language regions.
#[derive(Debug, PartialEq, Eq, EnumString, Display, Default)]
pub enum LanguageRussian {
    #[default]
    #[strum(serialize = "ru")]
    Default,
    #[strum(serialize = "ru-mo")]
    Moldova,
    #[strum(serialize = "ru-ru")]
    Russia,
}

/// Swedish language regions.
#[derive(Debug, PartialEq, Eq, EnumString, Display, Default)]
pub enum LanguageSwedish {
    #[default]
    #[strum(serialize = "sv")]
    Default,
    #[strum(serialize = "sv-fi")]
    Finland,
    #[strum(serialize = "sv-se")]
    Sweden,
}

/// Chinese language regions.
#[derive(Debug, PartialEq, Eq, EnumString, Display, Default)]
pub enum LanguageChinese {
    #[default]
    #[strum(serialize = "zh")]
    Default,
    #[strum(serialize = "zh-cn")]
    Simplified,
    #[strum(serialize = "zh-tw")]
    Traditional,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt() {
        pretty_assertions::assert_eq!(format!("{}", Language::Czech), "cs");
        pretty_assertions::assert_eq!(
            format!("{}", Language::English(LanguageEnglish::Default)),
            "en"
        );
        pretty_assertions::assert_eq!(
            format!("{}", Language::English(LanguageEnglish::UnitedKingdom)),
            "en-gb"
        );
        pretty_assertions::assert_eq!(
            format!("{}", Language::Other("other-language".to_string())),
            "other-language"
        );
    }
}

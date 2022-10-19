use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

/// Used for deserializing languages.
///
/// Language codes taken from <https://www.rssboard.org/rss-language-codes> and
/// <https://www.loc.gov/standards/iso639-2/php/code_list.php>.
#[derive(Debug, PartialEq, Eq, EnumString, Display)]
pub enum Language {
    #[strum(serialize = "aa")]
    Afar,
    #[strum(serialize = "ab")]
    Abkhazian,
    #[strum(serialize = "ae")]
    Avestan,
    #[strum(serialize = "af")]
    Afrikaans,
    #[strum(serialize = "ak")]
    Akan,
    #[strum(serialize = "am")]
    Amharic,
    #[strum(serialize = "an")]
    Aragonese,
    #[strum(serialize = "ar")]
    Arabic,
    #[strum(serialize = "as")]
    Assamese,
    #[strum(serialize = "av")]
    Avaric,
    #[strum(serialize = "ay")]
    Aymara,
    #[strum(serialize = "az")]
    Azerbaijani,
    #[strum(serialize = "ba")]
    Bashkir,
    #[strum(serialize = "be")]
    Belarusian,
    #[strum(serialize = "bg")]
    Bulgarian,
    #[strum(serialize = "bh")]
    BihariLanguages,
    #[strum(serialize = "bi")]
    Bislama,
    #[strum(serialize = "bm")]
    Bambara,
    #[strum(serialize = "bn")]
    Bengali,
    #[strum(serialize = "bo")]
    Tibetan,
    #[strum(serialize = "br")]
    Breton,
    #[strum(serialize = "bs")]
    Bosnian,
    #[strum(serialize = "ca")]
    Catalan,
    #[strum(serialize = "ce")]
    Chechen,
    #[strum(serialize = "ch")]
    Chamorro,
    #[strum(serialize = "co")]
    Corsican,
    #[strum(serialize = "cr")]
    Cree,
    #[strum(serialize = "cs")]
    Czech,
    #[strum(serialize = "cu")]
    SlavicChurch,
    #[strum(serialize = "cv")]
    Chuvash,
    #[strum(serialize = "cy")]
    Welsh,
    #[strum(serialize = "da")]
    Danish,
    #[strum(serialize = "de")]
    German(LanguageGerman),
    #[strum(serialize = "dv")]
    Divehi,
    #[strum(serialize = "dz")]
    Dzongkha,
    #[strum(serialize = "ee")]
    Ewe,
    #[strum(serialize = "el")]
    Greek,
    #[strum(serialize = "en")]
    English(LanguageEnglish),
    #[strum(serialize = "eo")]
    Esperanto,
    #[strum(serialize = "es")]
    Spanish(LanguageSpanish),
    #[strum(serialize = "et")]
    Estonian,
    #[strum(serialize = "eu")]
    Basque,
    #[strum(serialize = "fa")]
    Persian,
    #[strum(serialize = "ff")]
    Fulah,
    #[strum(serialize = "fi")]
    Finnish,
    #[strum(serialize = "fj")]
    Fijian,
    #[strum(serialize = "fo")]
    Faroese,
    #[strum(serialize = "fr")]
    French(LanguageFrench),
    #[strum(serialize = "fy")]
    WesternFrisian,
    #[strum(serialize = "ga")]
    Irish,
    #[strum(serialize = "gd")]
    Gaelic,
    #[strum(serialize = "gl")]
    Galician,
    #[strum(serialize = "gn")]
    Guarani,
    #[strum(serialize = "gu")]
    Gujarati,
    #[strum(serialize = "gv")]
    Manx,
    #[strum(serialize = "ha")]
    Hausa,
    #[strum(serialize = "haw")]
    Hawaiian,
    #[strum(serialize = "he")]
    Hebrew,
    #[strum(serialize = "hi")]
    Hindi,
    #[strum(serialize = "ho")]
    HiriMotu,
    #[strum(serialize = "hr")]
    Croatian,
    #[strum(serialize = "ht")]
    Haitian,
    #[strum(serialize = "hu")]
    Hungarian,
    #[strum(serialize = "hy")]
    Armenian,
    #[strum(serialize = "hz")]
    Herero,
    #[strum(serialize = "ia")]
    Interlingua,
    #[strum(serialize = "ie")]
    Interlingue,
    #[strum(serialize = "ig")]
    Igbo,
    #[strum(serialize = "ii")]
    SichuanYi,
    #[strum(serialize = "ik")]
    Inupiaq,
    #[strum(serialize = "in")]
    Indonesian,
    #[strum(serialize = "io")]
    Ido,
    #[strum(serialize = "is")]
    Icelandic,
    #[strum(serialize = "it")]
    Italian(LanguageItalian),
    #[strum(serialize = "iu")]
    Inuktitut,
    #[strum(serialize = "ja")]
    Japanese,
    #[strum(serialize = "jv")]
    Javanese,
    #[strum(serialize = "ka")]
    Georgian,
    #[strum(serialize = "kg")]
    Kongo,
    #[strum(serialize = "ki")]
    KikuyuGikuyu,
    #[strum(serialize = "kj")]
    Kuanyama,
    #[strum(serialize = "kk")]
    Kazakh,
    #[strum(serialize = "kl")]
    Kalaallisut,
    #[strum(serialize = "km")]
    CentralKhmer,
    #[strum(serialize = "kn")]
    Kannada,
    #[strum(serialize = "ko")]
    Korean,
    #[strum(serialize = "kr")]
    Kanuri,
    #[strum(serialize = "ks")]
    Kashmiri,
    #[strum(serialize = "ku")]
    Kurdish,
    #[strum(serialize = "kv")]
    Komi,
    #[strum(serialize = "kw")]
    Cornish,
    #[strum(serialize = "ky")]
    KirghizKyrgyz,
    #[strum(serialize = "la")]
    Latin,
    #[strum(serialize = "lb")]
    Luxembourgish,
    #[strum(serialize = "lg")]
    Ganda,
    #[strum(serialize = "li")]
    Limburgan,
    #[strum(serialize = "ln")]
    Lingala,
    #[strum(serialize = "lo")]
    Lao,
    #[strum(serialize = "lt")]
    Lithuanian,
    #[strum(serialize = "lu")]
    LubaKatanga,
    #[strum(serialize = "lv")]
    Latvian,
    #[strum(serialize = "mg")]
    Malagasy,
    #[strum(serialize = "mh")]
    Marshallese,
    #[strum(serialize = "mi")]
    Maori,
    #[strum(serialize = "mk")]
    Macedonian,
    #[strum(serialize = "ml")]
    Malayalam,
    #[strum(serialize = "mn")]
    Mongolian,
    #[strum(serialize = "mr")]
    Marathi,
    #[strum(serialize = "ms")]
    Malay,
    #[strum(serialize = "mt")]
    Maltese,
    #[strum(serialize = "my")]
    Burmese,
    #[strum(serialize = "na")]
    Nauru,
    #[strum(serialize = "nb")]
    NorwegianBokmal,
    #[strum(serialize = "ne")]
    Nepali,
    #[strum(serialize = "ng")]
    Ndonga,
    #[strum(serialize = "nl")]
    Dutch(LanguageDutch),
    #[strum(serialize = "nn")]
    NorwegianNynorsk,
    #[strum(serialize = "no")]
    Norwegian,
    #[strum(serialize = "nr")]
    Ndebele,
    #[strum(serialize = "nv")]
    Navajo,
    #[strum(serialize = "ny")]
    Chichewa,
    #[strum(serialize = "oc")]
    Occitan,
    #[strum(serialize = "oj")]
    Ojibwa,
    #[strum(serialize = "om")]
    Oromo,
    #[strum(serialize = "or")]
    Oriya,
    #[strum(serialize = "os")]
    Ossetian,
    #[strum(serialize = "pa")]
    Panjabi,
    #[strum(serialize = "pi")]
    Pali,
    #[strum(serialize = "pl")]
    Polish,
    #[strum(serialize = "ps")]
    Pushto,
    #[strum(serialize = "pt")]
    Portuguese(LanguagePortugese),
    #[strum(serialize = "qu")]
    Quechua,
    #[strum(serialize = "rm")]
    Romansh,
    #[strum(serialize = "rn")]
    Rundi,
    #[strum(serialize = "ro")]
    Romanian(LanguageRomanian),
    #[strum(serialize = "ru")]
    Russian(LanguageRussian),
    #[strum(serialize = "rw")]
    Kinyarwanda,
    #[strum(serialize = "sa")]
    Sanskrit,
    #[strum(serialize = "sc")]
    Sardinian,
    #[strum(serialize = "sd")]
    Sindhi,
    #[strum(serialize = "se")]
    NorthernSami,
    #[strum(serialize = "sg")]
    Sango,
    #[strum(serialize = "si")]
    Sinhala,
    #[strum(serialize = "sk")]
    Slovak,
    #[strum(serialize = "sl")]
    Slovenian,
    #[strum(serialize = "sm")]
    Samoan,
    #[strum(serialize = "sn")]
    Shona,
    #[strum(serialize = "so")]
    Somali,
    #[strum(serialize = "sq")]
    Albanian,
    #[strum(serialize = "sr")]
    Serbian,
    #[strum(serialize = "ss")]
    Swati,
    #[strum(serialize = "st")]
    SothoSouthern,
    #[strum(serialize = "su")]
    Sundanese,
    #[strum(serialize = "sv")]
    Swedish(LanguageSwedish),
    #[strum(serialize = "sw")]
    Swahili,
    #[strum(serialize = "ta")]
    Tamil,
    #[strum(serialize = "te")]
    Telugu,
    #[strum(serialize = "tg")]
    Tajik,
    #[strum(serialize = "th")]
    Thai,
    #[strum(serialize = "ti")]
    Tigrinya,
    #[strum(serialize = "tk")]
    Turkmen,
    #[strum(serialize = "tl")]
    Tagalog,
    #[strum(serialize = "tn")]
    Tswana,
    #[strum(serialize = "to")]
    Tonga,
    #[strum(serialize = "tr")]
    Turkish,
    #[strum(serialize = "ts")]
    Tsonga,
    #[strum(serialize = "tt")]
    Tatar,
    #[strum(serialize = "tw")]
    Twi,
    #[strum(serialize = "ty")]
    Tahitian,
    #[strum(serialize = "ug")]
    Uighur,
    #[strum(serialize = "uk")]
    Ukrainian,
    #[strum(serialize = "ur")]
    Urdu,
    #[strum(serialize = "uz")]
    Uzbek,
    #[strum(serialize = "ve")]
    Venda,
    #[strum(serialize = "vi")]
    Vietnamese,
    #[strum(serialize = "vo")]
    Volapük,
    #[strum(serialize = "wa")]
    Walloon,
    #[strum(serialize = "wo")]
    Wolof,
    #[strum(serialize = "xh")]
    Xhosa,
    #[strum(serialize = "yi")]
    Yiddish,
    #[strum(serialize = "yo")]
    Yoruba,
    #[strum(serialize = "za")]
    Zhuang,
    #[strum(serialize = "zh")]
    Chinese(LanguageChinese),
    #[strum(serialize = "zu")]
    Zulu,

    #[strum(disabled)]
    Other(String),
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

impl<'de> Deserialize<'de> for Language {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let mut s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };
        s = s.to_lowercase();

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

        match Language::from_str(s.as_str()) {
            Ok(x) => Ok(x),
            Err(_) => Ok(Language::Other(s)),
        }
    }
}

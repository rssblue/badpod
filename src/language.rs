use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

/// Used for deserializing timestamps.
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
    German,
    #[strum(serialize = "de-at")]
    GermanAustria,
    #[strum(serialize = "de-ch")]
    GermanSwitzerland,
    #[strum(serialize = "de-de")]
    GermanGermany,
    #[strum(serialize = "de-li")]
    GermanLiechtenstein,
    #[strum(serialize = "de-lu")]
    GermanLuxembourg,
    #[strum(serialize = "dv")]
    Divehi,
    #[strum(serialize = "dz")]
    Dzongkha,
    #[strum(serialize = "ee")]
    Ewe,
    #[strum(serialize = "el")]
    Greek,
    #[strum(serialize = "en")]
    English,
    #[strum(serialize = "en-au")]
    EnglishAustralia,
    #[strum(serialize = "en-bz")]
    EnglishBelize,
    #[strum(serialize = "en-ca")]
    EnglishCanada,
    #[strum(serialize = "en-gb")]
    EnglishUnitedKingdom,
    #[strum(serialize = "en-ie")]
    EnglishIreland,
    #[strum(serialize = "en-jm")]
    EnglishJamaica,
    #[strum(serialize = "en-nz")]
    EnglishNewZealand,
    #[strum(serialize = "en-ph")]
    EnglishPhillipines,
    #[strum(serialize = "en-tt")]
    EnglishTrinidad,
    #[strum(serialize = "en-us")]
    EnglishUnitedStates,
    #[strum(serialize = "en-za")]
    EnglishSouthAfrica,
    #[strum(serialize = "en-zw")]
    EnglishZimbabwe,
    #[strum(serialize = "eo")]
    Esperanto,
    #[strum(serialize = "es")]
    Spanish,
    #[strum(serialize = "es-ar")]
    SpanishArgentina,
    #[strum(serialize = "es-bo")]
    SpanishBolivia,
    #[strum(serialize = "es-cl")]
    SpanishChile,
    #[strum(serialize = "es-co")]
    SpanishColombia,
    #[strum(serialize = "es-cr")]
    SpanishCostaRica,
    #[strum(serialize = "es-do")]
    SpanishDominicanRepublic,
    #[strum(serialize = "es-ec")]
    SpanishEcuador,
    #[strum(serialize = "es-es")]
    SpanishSpain,
    #[strum(serialize = "es-gt")]
    SpanishGuatemala,
    #[strum(serialize = "es-hn")]
    SpanishHonduras,
    #[strum(serialize = "es-mx")]
    SpanishMexico,
    #[strum(serialize = "es-ni")]
    SpanishNicaragua,
    #[strum(serialize = "es-pa")]
    SpanishPanama,
    #[strum(serialize = "es-pe")]
    SpanishPeru,
    #[strum(serialize = "es-pr")]
    SpanishPuertoRico,
    #[strum(serialize = "es-py")]
    SpanishParaguay,
    #[strum(serialize = "es-sv")]
    SpanishElSalvador,
    #[strum(serialize = "es-uy")]
    SpanishUruguay,
    #[strum(serialize = "es-ve")]
    SpanishVenezuela,
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
    French,
    #[strum(serialize = "fr-be")]
    FrenchBelgium,
    #[strum(serialize = "fr-ca")]
    FrenchCanada,
    #[strum(serialize = "fr-ch")]
    FrenchSwitzerland,
    #[strum(serialize = "fr-fr")]
    FrenchFrance,
    #[strum(serialize = "fr-lu")]
    FrenchLuxembourg,
    #[strum(serialize = "fr-mc")]
    FrenchMonaco,
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
    Italian,
    #[strum(serialize = "it-ch")]
    ItalianSwitzerland,
    #[strum(serialize = "it-it")]
    ItalianItaly,
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
    Dutch,
    #[strum(serialize = "nl-be")]
    DutchBelgium,
    #[strum(serialize = "nl-nl")]
    DutchNetherlands,
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
    Portuguese,
    #[strum(serialize = "pt-br")]
    PortugueseBrazil,
    #[strum(serialize = "pt-pt")]
    PortuguesePortugal,
    #[strum(serialize = "qu")]
    Quechua,
    #[strum(serialize = "rm")]
    Romansh,
    #[strum(serialize = "rn")]
    Rundi,
    #[strum(serialize = "ro")]
    Romanian,
    #[strum(serialize = "ro-mo")]
    RomanianMoldova,
    #[strum(serialize = "ro-ro")]
    RomanianRomania,
    #[strum(serialize = "ru")]
    Russian,
    #[strum(serialize = "ru-mo")]
    RussianMoldova,
    #[strum(serialize = "ru-ru")]
    RussianRussia,
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
    Swedish,
    #[strum(serialize = "sv-fi")]
    SwedishFinland,
    #[strum(serialize = "sv-se")]
    SwedishSweden,
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
    Chinese,
    #[strum(serialize = "zh-cn")]
    ChineseSimplified,
    #[strum(serialize = "zh-tw")]
    ChineseTraditional,
    #[strum(serialize = "zu")]
    Zulu,

    #[strum(disabled)]
    Other(String),
}

impl<'de> Deserialize<'de> for Language {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match Language::from_str(s.to_lowercase().as_str()) {
            Ok(x) => Ok(x),
            Err(_) => Ok(Language::Other(s)),
        }
    }
}

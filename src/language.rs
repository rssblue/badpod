use serde_enum_str::Deserialize_enum_str;

// Language codes taken from <https://www.rssboard.org/rss-language-codes> and
// <https://www.loc.gov/standards/iso639-2/php/code_list.php>.
#[derive(Debug, Deserialize_enum_str, PartialEq)]
pub enum Language {
    #[serde(rename = "aa")]
    Afar,
    #[serde(rename = "ab")]
    Abkhazian,
    #[serde(rename = "ae")]
    Avestan,
    #[serde(rename = "af")]
    Afrikaans,
    #[serde(rename = "ak")]
    Akan,
    #[serde(rename = "am")]
    Amharic,
    #[serde(rename = "an")]
    Aragonese,
    #[serde(rename = "ar")]
    Arabic,
    #[serde(rename = "as")]
    Assamese,
    #[serde(rename = "av")]
    Avaric,
    #[serde(rename = "ay")]
    Aymara,
    #[serde(rename = "az")]
    Azerbaijani,
    #[serde(rename = "ba")]
    Bashkir,
    #[serde(rename = "be")]
    Belarusian,
    #[serde(rename = "bg")]
    Bulgarian,
    #[serde(rename = "bh")]
    BihariLanguages,
    #[serde(rename = "bi")]
    Bislama,
    #[serde(rename = "bm")]
    Bambara,
    #[serde(rename = "bn")]
    Bengali,
    #[serde(rename = "bo")]
    Tibetan,
    #[serde(rename = "br")]
    Breton,
    #[serde(rename = "bs")]
    Bosnian,
    #[serde(rename = "ca")]
    Catalan,
    #[serde(rename = "ce")]
    Chechen,
    #[serde(rename = "ch")]
    Chamorro,
    #[serde(rename = "co")]
    Corsican,
    #[serde(rename = "cr")]
    Cree,
    #[serde(rename = "cs")]
    Czech,
    #[serde(rename = "cu")]
    SlavicChurch,
    #[serde(rename = "cv")]
    Chuvash,
    #[serde(rename = "cy")]
    Welsh,
    #[serde(rename = "da")]
    Danish,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "de-at")]
    GermanAustria,
    #[serde(rename = "de-ch")]
    GermanSwitzerland,
    #[serde(rename = "de-de")]
    GermanGermany,
    #[serde(rename = "de-li")]
    GermanLiechtenstein,
    #[serde(rename = "de-lu")]
    GermanLuxembourg,
    #[serde(rename = "dv")]
    Divehi,
    #[serde(rename = "ee")]
    Ewe,
    #[serde(rename = "el")]
    Greek,
    #[serde(rename = "en")]
    English,
    #[serde(rename = "en-au")]
    EnglishAustralia,
    #[serde(rename = "en-bz")]
    EnglishBelize,
    #[serde(rename = "en-ca")]
    EnglishCanada,
    #[serde(rename = "en-gb")]
    EnglishUnitedKingdom,
    #[serde(rename = "en-ie")]
    EnglishIreland,
    #[serde(rename = "en-jm")]
    EnglishJamaica,
    #[serde(rename = "en-nz")]
    EnglishNewZealand,
    #[serde(rename = "en-ph")]
    EnglishPhillipines,
    #[serde(rename = "en-tt")]
    EnglishTrinidad,
    #[serde(rename = "en-us")]
    EnglishUnitedStates,
    #[serde(rename = "en-za")]
    EnglishSouthAfrica,
    #[serde(rename = "en-zw")]
    EnglishZimbabwe,
    #[serde(rename = "eo")]
    Esperanto,
    #[serde(rename = "es")]
    Spanish,
    #[serde(rename = "es-ar")]
    SpanishArgentina,
    #[serde(rename = "es-bo")]
    SpanishBolivia,
    #[serde(rename = "es-cl")]
    SpanishChile,
    #[serde(rename = "es-co")]
    SpanishColombia,
    #[serde(rename = "es-cr")]
    SpanishCostaRica,
    #[serde(rename = "es-do")]
    SpanishDominicanRepublic,
    #[serde(rename = "es-ec")]
    SpanishEcuador,
    #[serde(rename = "es-es")]
    SpanishSpain,
    #[serde(rename = "es-gt")]
    SpanishGuatemala,
    #[serde(rename = "es-hn")]
    SpanishHonduras,
    #[serde(rename = "es-mx")]
    SpanishMexico,
    #[serde(rename = "es-ni")]
    SpanishNicaragua,
    #[serde(rename = "es-pa")]
    SpanishPanama,
    #[serde(rename = "es-pe")]
    SpanishPeru,
    #[serde(rename = "es-pr")]
    SpanishPuertoRico,
    #[serde(rename = "es-py")]
    SpanishParaguay,
    #[serde(rename = "es-sv")]
    SpanishElSalvador,
    #[serde(rename = "es-uy")]
    SpanishUruguay,
    #[serde(rename = "es-ve")]
    SpanishVenezuela,
    #[serde(rename = "et")]
    Estonian,
    #[serde(rename = "eu")]
    Basque,
    #[serde(rename = "fa")]
    Persian,
    #[serde(rename = "ff")]
    Fulah,
    #[serde(rename = "fi")]
    Finnish,
    #[serde(rename = "fj")]
    Fijian,
    #[serde(rename = "fo")]
    Faeroese,
    #[serde(rename = "fo")]
    Faroese,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "fr-be")]
    FrenchBelgium,
    #[serde(rename = "fr-ca")]
    FrenchCanada,
    #[serde(rename = "fr-ch")]
    FrenchSwitzerland,
    #[serde(rename = "fr-fr")]
    FrenchFrance,
    #[serde(rename = "fr-lu")]
    FrenchLuxembourg,
    #[serde(rename = "fr-mc")]
    FrenchMonaco,
    #[serde(rename = "fy")]
    WesternFrisian,
    #[serde(rename = "ga")]
    Irish,
    #[serde(rename = "gd")]
    Gaelic,
    #[serde(rename = "gl")]
    Galician,
    #[serde(rename = "gn")]
    Guarani,
    #[serde(rename = "gu")]
    Gujarati,
    #[serde(rename = "gv")]
    Manx,
    #[serde(rename = "ha")]
    Hausa,
    #[serde(rename = "haw")]
    Hawaiian,
    #[serde(rename = "he")]
    Hebrew,
    #[serde(rename = "hi")]
    Hindi,
    #[serde(rename = "ho")]
    HiriMotu,
    #[serde(rename = "hr")]
    Croatian,
    #[serde(rename = "ht")]
    Haitian,
    #[serde(rename = "hu")]
    Hungarian,
    #[serde(rename = "hy")]
    Armenian,
    #[serde(rename = "hz")]
    Herero,
    #[serde(rename = "ia")]
    Interlingua,
    #[serde(rename = "ie")]
    Interlingue,
    #[serde(rename = "ig")]
    Igbo,
    #[serde(rename = "ii")]
    SichuanYi,
    #[serde(rename = "ik")]
    Inupiaq,
    #[serde(rename = "in")]
    Indonesian,
    #[serde(rename = "io")]
    Ido,
    #[serde(rename = "is")]
    Icelandic,
    #[serde(rename = "it")]
    Italian,
    #[serde(rename = "it-ch")]
    ItalianSwitzerland,
    #[serde(rename = "it-it")]
    ItalianItaly,
    #[serde(rename = "iu")]
    Inuktitut,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "jv")]
    Javanese,
    #[serde(rename = "ka")]
    Georgian,
    #[serde(rename = "kg")]
    Kongo,
    #[serde(rename = "ki")]
    KikuyuGikuyu,
    #[serde(rename = "kj")]
    Kuanyama,
    #[serde(rename = "kk")]
    Kazakh,
    #[serde(rename = "kl")]
    Kalaallisut,
    #[serde(rename = "km")]
    CentralKhmer,
    #[serde(rename = "kn")]
    Kannada,
    #[serde(rename = "ko")]
    Korean,
    #[serde(rename = "kr")]
    Kanuri,
    #[serde(rename = "ks")]
    Kashmiri,
    #[serde(rename = "ku")]
    Kurdish,
    #[serde(rename = "kv")]
    Komi,
    #[serde(rename = "kw")]
    Cornish,
    #[serde(rename = "ky")]
    KirghizKyrgyz,
    #[serde(rename = "la")]
    Latin,
    #[serde(rename = "lb")]
    Luxembourgish,
    #[serde(rename = "lg")]
    Ganda,
    #[serde(rename = "li")]
    Limburgan,
    #[serde(rename = "ln")]
    Lingala,
    #[serde(rename = "lo")]
    Lao,
    #[serde(rename = "lt")]
    Lithuanian,
    #[serde(rename = "lu")]
    LubaKatanga,
    #[serde(rename = "lv")]
    Latvian,
    #[serde(rename = "mg")]
    Malagasy,
    #[serde(rename = "mh")]
    Marshallese,
    #[serde(rename = "mi")]
    Maori,
    #[serde(rename = "mk")]
    Macedonian,
    #[serde(rename = "ml")]
    Malayalam,
    #[serde(rename = "mn")]
    Mongolian,
    #[serde(rename = "mr")]
    Marathi,
    #[serde(rename = "ms")]
    Malay,
    #[serde(rename = "mt")]
    Maltese,
    #[serde(rename = "my")]
    Burmese,
    #[serde(rename = "na")]
    Nauru,
    #[serde(rename = "nb")]
    NorwegianBokmal,
    #[serde(rename = "ne")]
    Nepali,
    #[serde(rename = "ng")]
    Ndonga,
    #[serde(rename = "nl")]
    Dutch,
    #[serde(rename = "nl")]
    Dzongkha,
    #[serde(rename = "nl-be")]
    DutchBelgium,
    #[serde(rename = "nl-nl")]
    DutchNetherlands,
    #[serde(rename = "nn")]
    NorwegianNynorsk,
    #[serde(rename = "no")]
    Norwegian,
    #[serde(rename = "nr")]
    Ndebele,
    #[serde(rename = "nv")]
    Navajo,
    #[serde(rename = "ny")]
    Chichewa,
    #[serde(rename = "oc")]
    Occitan,
    #[serde(rename = "oj")]
    Ojibwa,
    #[serde(rename = "om")]
    Oromo,
    #[serde(rename = "or")]
    Oriya,
    #[serde(rename = "os")]
    Ossetian,
    #[serde(rename = "pa")]
    Panjabi,
    #[serde(rename = "pi")]
    Pali,
    #[serde(rename = "pl")]
    Polish,
    #[serde(rename = "ps")]
    Pushto,
    #[serde(rename = "pt")]
    Portuguese,
    #[serde(rename = "pt-br")]
    PortugueseBrazil,
    #[serde(rename = "pt-pt")]
    PortuguesePortugal,
    #[serde(rename = "qu")]
    Quechua,
    #[serde(rename = "rm")]
    Romansh,
    #[serde(rename = "rn")]
    Rundi,
    #[serde(rename = "ro")]
    Romanian,
    #[serde(rename = "ro-mo")]
    RomanianMoldova,
    #[serde(rename = "ro-ro")]
    RomanianRomania,
    #[serde(rename = "ru")]
    Russian,
    #[serde(rename = "ru-mo")]
    RussianMoldova,
    #[serde(rename = "ru-ru")]
    RussianRussia,
    #[serde(rename = "rw")]
    Kinyarwanda,
    #[serde(rename = "sa")]
    Sanskrit,
    #[serde(rename = "sc")]
    Sardinian,
    #[serde(rename = "sd")]
    Sindhi,
    #[serde(rename = "se")]
    NorthernSami,
    #[serde(rename = "sg")]
    Sango,
    #[serde(rename = "si")]
    Sinhala,
    #[serde(rename = "sk")]
    Slovak,
    #[serde(rename = "sl")]
    Slovenian,
    #[serde(rename = "sm")]
    Samoan,
    #[serde(rename = "sn")]
    Shona,
    #[serde(rename = "so")]
    Somali,
    #[serde(rename = "sq")]
    Albanian,
    #[serde(rename = "sr")]
    Serbian,
    #[serde(rename = "ss")]
    Swati,
    #[serde(rename = "st")]
    SothoSouthern,
    #[serde(rename = "su")]
    Sundanese,
    #[serde(rename = "sv")]
    Swedish,
    #[serde(rename = "sv-fi")]
    SwedishFinland,
    #[serde(rename = "sv-se")]
    SwedishSweden,
    #[serde(rename = "sw")]
    Swahili,
    #[serde(rename = "ta")]
    Tamil,
    #[serde(rename = "te")]
    Telugu,
    #[serde(rename = "tg")]
    Tajik,
    #[serde(rename = "th")]
    Thai,
    #[serde(rename = "ti")]
    Tigrinya,
    #[serde(rename = "tk")]
    Turkmen,
    #[serde(rename = "tl")]
    Tagalog,
    #[serde(rename = "tn")]
    Tswana,
    #[serde(rename = "to")]
    Tonga,
    #[serde(rename = "tr")]
    Turkish,
    #[serde(rename = "ts")]
    Tsonga,
    #[serde(rename = "tt")]
    Tatar,
    #[serde(rename = "tw")]
    Twi,
    #[serde(rename = "ty")]
    Tahitian,
    #[serde(rename = "ug")]
    Uighur,
    #[serde(rename = "uk")]
    Ukrainian,
    #[serde(rename = "uk")]
    Ukranian,
    #[serde(rename = "ur")]
    Urdu,
    #[serde(rename = "uz")]
    Uzbek,
    #[serde(rename = "ve")]
    Venda,
    #[serde(rename = "vi")]
    Vietnamese,
    #[serde(rename = "vo")]
    Volapük,
    #[serde(rename = "wa")]
    Walloon,
    #[serde(rename = "wo")]
    Wolof,
    #[serde(rename = "xh")]
    Xhosa,
    #[serde(rename = "yi")]
    Yiddish,
    #[serde(rename = "yo")]
    Yoruba,
    #[serde(rename = "za")]
    Zhuang,
    #[serde(rename = "zh")]
    Chinese,
    #[serde(rename = "zh-cn")]
    ChineseSimplified,
    #[serde(rename = "zh-tw")]
    ChineseTraditional,
    #[serde(rename = "zu")]
    Zulu,

    #[serde(other)]
    Other(String),
}

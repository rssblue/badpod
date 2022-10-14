use serde_enum_str::Deserialize_enum_str;

// Language codes taken from <https://www.rssboard.org/rss-language-codes> and
// <https://www.loc.gov/standards/iso639-2/php/code_list.php>.
#[derive(Debug, Deserialize_enum_str, PartialEq)]
pub enum Language {
    #[serde(rename = "aa")]
    Afar,
    #[serde(rename = "ab")]
    Abkhazian,
    #[serde(rename = "af")]
    Afrikaans,
    #[serde(rename = "ak")]
    Akan,
    #[serde(rename = "sq")]
    Albanian,
    #[serde(rename = "am")]
    Amharic,
    #[serde(rename = "ar")]
    Arabic,
    #[serde(rename = "an")]
    Aragonese,
    #[serde(rename = "hy")]
    Armenian,
    #[serde(rename = "as")]
    Assamese,
    #[serde(rename = "av")]
    Avaric,
    #[serde(rename = "eu")]
    Basque,
    #[serde(rename = "be")]
    Belarusian,
    #[serde(rename = "bg")]
    Bulgarian,
    #[serde(rename = "ca")]
    Catalan,
    #[serde(rename = "zh-cn")]
    ChineseSimplified,
    #[serde(rename = "zh-tw")]
    ChineseTraditional,
    #[serde(rename = "hr")]
    Croatian,
    #[serde(rename = "cs")]
    Czech,
    #[serde(rename = "da")]
    Danish,
    #[serde(rename = "nl")]
    Dutch,
    #[serde(rename = "nl-be")]
    DutchBelgium,
    #[serde(rename = "nl-nl")]
    DutchNetherlands,
    #[serde(rename = "en")]
    English,
    #[serde(rename = "en-au")]
    EnglishAustralia,
    #[serde(rename = "en-bz")]
    EnglishBelize,
    #[serde(rename = "en-ca")]
    EnglishCanada,
    #[serde(rename = "en-ie")]
    EnglishIreland,
    #[serde(rename = "en-jm")]
    EnglishJamaica,
    #[serde(rename = "en-nz")]
    EnglishNewZealand,
    #[serde(rename = "en-ph")]
    EnglishPhillipines,
    #[serde(rename = "en-za")]
    EnglishSouthAfrica,
    #[serde(rename = "en-tt")]
    EnglishTrinidad,
    #[serde(rename = "en-gb")]
    EnglishUnitedKingdom,
    #[serde(rename = "en-us")]
    EnglishUnitedStates,
    #[serde(rename = "en-zw")]
    EnglishZimbabwe,
    #[serde(rename = "et")]
    Estonian,
    #[serde(rename = "fo")]
    Faeroese,
    #[serde(rename = "fi")]
    Finnish,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "fr-be")]
    FrenchBelgium,
    #[serde(rename = "fr-ca")]
    FrenchCanada,
    #[serde(rename = "fr-fr")]
    FrenchFrance,
    #[serde(rename = "fr-lu")]
    FrenchLuxembourg,
    #[serde(rename = "fr-mc")]
    FrenchMonaco,
    #[serde(rename = "fr-ch")]
    FrenchSwitzerland,
    #[serde(rename = "gl")]
    Galician,
    #[serde(rename = "gd")]
    Gaelic,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "de-at")]
    GermanAustria,
    #[serde(rename = "de-de")]
    GermanGermany,
    #[serde(rename = "de-li")]
    GermanLiechtenstein,
    #[serde(rename = "de-lu")]
    GermanLuxembourg,
    #[serde(rename = "de-ch")]
    GermanSwitzerland,
    #[serde(rename = "el")]
    Greek,
    #[serde(rename = "haw")]
    Hawaiian,
    #[serde(rename = "hu")]
    Hungarian,
    #[serde(rename = "is")]
    Icelandic,
    #[serde(rename = "in")]
    Indonesian,
    #[serde(rename = "ga")]
    Irish,
    #[serde(rename = "it")]
    Italian,
    #[serde(rename = "it-it")]
    ItalianItaly,
    #[serde(rename = "it-ch")]
    ItalianSwitzerland,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "ko")]
    Korean,
    #[serde(rename = "mk")]
    Macedonian,
    #[serde(rename = "no")]
    Norwegian,
    #[serde(rename = "pl")]
    Polish,
    #[serde(rename = "pt")]
    Portuguese,
    #[serde(rename = "pt-br")]
    PortugueseBrazil,
    #[serde(rename = "pt-pt")]
    PortuguesePortugal,
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
    #[serde(rename = "sr")]
    Serbian,
    #[serde(rename = "sk")]
    Slovak,
    #[serde(rename = "sl")]
    Slovenian,
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
    #[serde(rename = "es-sv")]
    SpanishElSalvador,
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
    #[serde(rename = "es-py")]
    SpanishParaguay,
    #[serde(rename = "es-pe")]
    SpanishPeru,
    #[serde(rename = "es-pr")]
    SpanishPuertoRico,
    #[serde(rename = "es-es")]
    SpanishSpain,
    #[serde(rename = "es-uy")]
    SpanishUruguay,
    #[serde(rename = "es-ve")]
    SpanishVenezuela,
    #[serde(rename = "sv")]
    Swedish,
    #[serde(rename = "sv-fi")]
    SwedishFinland,
    #[serde(rename = "sv-se")]
    SwedishSweden,
    #[serde(rename = "tr")]
    Turkish,
    #[serde(rename = "uk")]
    Ukranian,
    #[serde(rename = "ae")]
    Avestan,
    #[serde(rename = "ay")]
    Aymara,
    #[serde(rename = "az")]
    Azerbaijani,
    #[serde(rename = "ba")]
    Bashkir,
    #[serde(rename = "bm")]
    Bambara,
    #[serde(rename = "bn")]
    Bengali,
    #[serde(rename = "bh")]
    BihariLanguages,
    #[serde(rename = "bi")]
    Bislama,
    #[serde(rename = "bo")]
    Tibetan,
    #[serde(rename = "bs")]
    Bosnian,
    #[serde(rename = "br")]
    Breton,
    #[serde(rename = "my")]
    Burmese,
    #[serde(rename = "ch")]
    Chamorro,
    #[serde(rename = "ce")]
    Chechen,
    #[serde(rename = "zh")]
    Chinese,
    #[serde(rename = "cu")]
    SlavicChurch,
    #[serde(rename = "cv")]
    Chuvash,
    #[serde(rename = "kw")]
    Cornish,
    #[serde(rename = "co")]
    Corsican,
    #[serde(rename = "cr")]
    Cree,
    #[serde(rename = "cy")]
    Welsh,
    #[serde(rename = "dv")]
    Divehi,
    #[serde(rename = "nl")]
    Dzongkha,
    #[serde(rename = "eo")]
    Esperanto,
    #[serde(rename = "ee")]
    Ewe,
    #[serde(rename = "fo")]
    Faroese,
    #[serde(rename = "fa")]
    Persian,
    #[serde(rename = "fj")]
    Fijian,
    #[serde(rename = "fy")]
    WesternFrisian,
    #[serde(rename = "ff")]
    Fulah,
    #[serde(rename = "ka")]
    Georgian,
    #[serde(rename = "gv")]
    Manx,
    #[serde(rename = "gn")]
    Guarani,
    #[serde(rename = "gu")]
    Gujarati,
    #[serde(rename = "ht")]
    Haitian,
    #[serde(rename = "ha")]
    Hausa,
    #[serde(rename = "he")]
    Hebrew,
    #[serde(rename = "hz")]
    Herero,
    #[serde(rename = "hi")]
    Hindi,
    #[serde(rename = "ho")]
    HiriMotu,
    #[serde(rename = "ig")]
    Igbo,
    #[serde(rename = "io")]
    Ido,
    #[serde(rename = "ii")]
    SichuanYi,
    #[serde(rename = "iu")]
    Inuktitut,
    #[serde(rename = "ie")]
    Interlingue,
    #[serde(rename = "ia")]
    Interlingua,
    #[serde(rename = "ik")]
    Inupiaq,
    #[serde(rename = "jv")]
    Javanese,
    #[serde(rename = "kl")]
    Kalaallisut,
    #[serde(rename = "kn")]
    Kannada,
    #[serde(rename = "ks")]
    Kashmiri,
    #[serde(rename = "kr")]
    Kanuri,
    #[serde(rename = "kk")]
    Kazakh,
    #[serde(rename = "km")]
    CentralKhmer,
    #[serde(rename = "ki")]
    KikuyuGikuyu,
    #[serde(rename = "rw")]
    Kinyarwanda,
    #[serde(rename = "ky")]
    KirghizKyrgyz,
    #[serde(rename = "kv")]
    Komi,
    #[serde(rename = "kg")]
    Kongo,
    #[serde(rename = "kj")]
    Kuanyama,
    #[serde(rename = "ku")]
    Kurdish,
    #[serde(rename = "lo")]
    Lao,
    #[serde(rename = "la")]
    Latin,
    #[serde(rename = "lv")]
    Latvian,
    #[serde(rename = "li")]
    Limburgan,
    #[serde(rename = "ln")]
    Lingala,
    #[serde(rename = "lt")]
    Lithuanian,
    #[serde(rename = "lb")]
    Luxembourgish,
    #[serde(rename = "lu")]
    LubaKatanga,
    #[serde(rename = "lg")]
    Ganda,
    #[serde(rename = "mh")]
    Marshallese,
    #[serde(rename = "ml")]
    Malayalam,
    #[serde(rename = "mi")]
    Maori,
    #[serde(rename = "mr")]
    Marathi,
    #[serde(rename = "ms")]
    Malay,
    #[serde(rename = "mg")]
    Malagasy,
    #[serde(rename = "mt")]
    Maltese,
    #[serde(rename = "mn")]
    Mongolian,
    #[serde(rename = "na")]
    Nauru,
    #[serde(rename = "nv")]
    Navajo,
    #[serde(rename = "nr")]
    Ndebele,
    #[serde(rename = "ng")]
    Ndonga,
    #[serde(rename = "ne")]
    Nepali,
    #[serde(rename = "nn")]
    NorwegianNynorsk,
    #[serde(rename = "nb")]
    NorwegianBokmal,
    #[serde(rename = "ny")]
    Chichewa,
    #[serde(rename = "oc")]
    Occitan,
    #[serde(rename = "oj")]
    Ojibwa,
    #[serde(rename = "or")]
    Oriya,
    #[serde(rename = "om")]
    Oromo,
    #[serde(rename = "os")]
    Ossetian,
    #[serde(rename = "pa")]
    Panjabi,
    #[serde(rename = "pi")]
    Pali,
    #[serde(rename = "ps")]
    Pushto,
    #[serde(rename = "qu")]
    Quechua,
    #[serde(rename = "rm")]
    Romansh,
    #[serde(rename = "rn")]
    Rundi,
    #[serde(rename = "sg")]
    Sango,
    #[serde(rename = "sa")]
    Sanskrit,
    #[serde(rename = "si")]
    Sinhala,
    #[serde(rename = "se")]
    NorthernSami,
    #[serde(rename = "sm")]
    Samoan,
    #[serde(rename = "sn")]
    Shona,
    #[serde(rename = "sd")]
    Sindhi,
    #[serde(rename = "so")]
    Somali,
    #[serde(rename = "st")]
    Sotho,
    Southern,
    #[serde(rename = "sc")]
    Sardinian,
    #[serde(rename = "ss")]
    Swati,
    #[serde(rename = "su")]
    Sundanese,
    #[serde(rename = "sw")]
    Swahili,
    #[serde(rename = "ty")]
    Tahitian,
    #[serde(rename = "ta")]
    Tamil,
    #[serde(rename = "tt")]
    Tatar,
    #[serde(rename = "te")]
    Telugu,
    #[serde(rename = "tg")]
    Tajik,
    #[serde(rename = "tl")]
    Tagalog,
    #[serde(rename = "th")]
    Thai,
    #[serde(rename = "ti")]
    Tigrinya,
    #[serde(rename = "to")]
    Tonga,
    #[serde(rename = "tn")]
    Tswana,
    #[serde(rename = "ts")]
    Tsonga,
    #[serde(rename = "tk")]
    Turkmen,
    #[serde(rename = "tw")]
    Twi,
    #[serde(rename = "ug")]
    Uighur,
    #[serde(rename = "uk")]
    Ukrainian,
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
    #[serde(rename = "zu")]
    Zulu,

    #[serde(other)]
    Other(String),
}

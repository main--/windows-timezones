#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "strum", derive(strum::EnumIter))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "sqlx",
    derive(sqlx::Type),
    sqlx(type_name = "windows_timezone", rename_all = "snake_case")
)]
pub enum Timezone {
    #[doc = "(UTC-12:00) International Date Line West"]
    DatelineStandardTime,
    #[doc = "(UTC-11:00) Coordinated Universal Time-11"]
    UtcMinus11,
    #[doc = "(UTC-10:00) Aleutian Islands"]
    AleutianStandardTime,
    #[doc = "(UTC-10:00) Hawaii"]
    HawaiianStandardTime,
    #[doc = "(UTC-09:30) Marquesas Islands"]
    MarquesasStandardTime,
    #[doc = "(UTC-09:00) Alaska"]
    AlaskanStandardTime,
    #[doc = "(UTC-09:00) Coordinated Universal Time-09"]
    UtcMinus09,
    #[doc = "(UTC-08:00) Baja California"]
    PacificStandardTimeMexico,
    #[doc = "(UTC-08:00) Coordinated Universal Time-08"]
    UtcMinus08,
    #[doc = "(UTC-08:00) Pacific Time (US & Canada)"]
    PacificStandardTime,
    #[doc = "(UTC-07:00) Arizona"]
    UsMountainStandardTime,
    #[doc = "(UTC-07:00) Chihuahua, La Paz, Mazatlan"]
    MountainStandardTimeMexico,
    #[doc = "(UTC-07:00) Mountain Time (US & Canada)"]
    MountainStandardTime,
    #[doc = "(UTC-07:00) Yukon"]
    YukonStandardTime,
    #[doc = "(UTC-06:00) Central America"]
    CentralAmericaStandardTime,
    #[doc = "(UTC-06:00) Central Time (US & Canada)"]
    CentralStandardTime,
    #[doc = "(UTC-06:00) Easter Island"]
    EasterIslandStandardTime,
    #[doc = "(UTC-06:00) Guadalajara, Mexico City, Monterrey"]
    CentralStandardTimeMexico,
    #[doc = "(UTC-06:00) Saskatchewan"]
    CanadaCentralStandardTime,
    #[doc = "(UTC-05:00) Bogota, Lima, Quito, Rio Branco"]
    SaPacificStandardTime,
    #[doc = "(UTC-05:00) Chetumal"]
    EasternStandardTimeMexico,
    #[doc = "(UTC-05:00) Eastern Time (US & Canada)"]
    EasternStandardTime,
    #[doc = "(UTC-05:00) Haiti"]
    HaitiStandardTime,
    #[doc = "(UTC-05:00) Havana"]
    CubaStandardTime,
    #[doc = "(UTC-05:00) Indiana (East)"]
    UsEasternStandardTime,
    #[doc = "(UTC-05:00) Turks and Caicos"]
    TurksAndCaicosStandardTime,
    #[doc = "(UTC-04:00) Asuncion"]
    ParaguayStandardTime,
    #[doc = "(UTC-04:00) Atlantic Time (Canada)"]
    AtlanticStandardTime,
    #[doc = "(UTC-04:00) Caracas"]
    VenezuelaStandardTime,
    #[doc = "(UTC-04:00) Cuiaba"]
    CentralBrazilianStandardTime,
    #[doc = "(UTC-04:00) Georgetown, La Paz, Manaus, San Juan"]
    SaWesternStandardTime,
    #[doc = "(UTC-04:00) Santiago"]
    PacificSaStandardTime,
    #[doc = "(UTC-03:30) Newfoundland"]
    NewfoundlandStandardTime,
    #[doc = "(UTC-03:00) Araguaina"]
    TocantinsStandardTime,
    #[doc = "(UTC-03:00) Brasilia"]
    ESouthAmericaStandardTime,
    #[doc = "(UTC-03:00) Cayenne, Fortaleza"]
    SaEasternStandardTime,
    #[doc = "(UTC-03:00) City of Buenos Aires"]
    ArgentinaStandardTime,
    #[doc = "(UTC-03:00) Greenland"]
    GreenlandStandardTime,
    #[doc = "(UTC-03:00) Montevideo"]
    MontevideoStandardTime,
    #[doc = "(UTC-03:00) Punta Arenas"]
    MagallanesStandardTime,
    #[doc = "(UTC-03:00) Saint Pierre and Miquelon"]
    SaintPierreStandardTime,
    #[doc = "(UTC-03:00) Salvador"]
    BahiaStandardTime,
    #[doc = "(UTC-02:00) Coordinated Universal Time-02"]
    UtcMinus02,
    #[doc = "(UTC-01:00) Azores"]
    AzoresStandardTime,
    #[doc = "(UTC-01:00) Cabo Verde Is."]
    CapeVerdeStandardTime,
    #[doc = "(UTC) Coordinated Universal Time"]
    #[default]
    Utc,
    #[doc = "(UTC+00:00) Dublin, Edinburgh, Lisbon, London"]
    GmtStandardTime,
    #[doc = "(UTC+00:00) Monrovia, Reykjavik"]
    GreenwichStandardTime,
    #[doc = "(UTC+00:00) Sao Tome"]
    SaoTomeStandardTime,
    #[doc = "(UTC+01:00) Casablanca"]
    MoroccoStandardTime,
    #[doc = "(UTC+01:00) Amsterdam, Berlin, Bern, Rome, Stockholm, Vienna"]
    WEuropeStandardTime,
    #[doc = "(UTC+01:00) Belgrade, Bratislava, Budapest, Ljubljana, Prague"]
    CentralEuropeStandardTime,
    #[doc = "(UTC+01:00) Brussels, Copenhagen, Madrid, Paris"]
    RomanceStandardTime,
    #[doc = "(UTC+01:00) Sarajevo, Skopje, Warsaw, Zagreb"]
    CentralEuropeanStandardTime,
    #[doc = "(UTC+01:00) West Central Africa"]
    WCentralAfricaStandardTime,
    #[doc = "(UTC+02:00) Amman"]
    JordanStandardTime,
    #[doc = "(UTC+02:00) Athens, Bucharest"]
    GtbStandardTime,
    #[doc = "(UTC+02:00) Beirut"]
    MiddleEastStandardTime,
    #[doc = "(UTC+02:00) Cairo"]
    EgyptStandardTime,
    #[doc = "(UTC+02:00) Chisinau"]
    EEuropeStandardTime,
    #[doc = "(UTC+02:00) Damascus"]
    SyriaStandardTime,
    #[doc = "(UTC+02:00) Gaza, Hebron"]
    WestBankStandardTime,
    #[doc = "(UTC+02:00) Harare, Pretoria"]
    SouthAfricaStandardTime,
    #[doc = "(UTC+02:00) Helsinki, Kyiv, Riga, Sofia, Tallinn, Vilnius"]
    FleStandardTime,
    #[doc = "(UTC+02:00) Jerusalem"]
    IsraelStandardTime,
    #[doc = "(UTC+02:00) Juba"]
    SouthSudanStandardTime,
    #[doc = "(UTC+02:00) Kaliningrad"]
    KaliningradStandardTime,
    #[doc = "(UTC+02:00) Khartoum"]
    SudanStandardTime,
    #[doc = "(UTC+02:00) Tripoli"]
    LibyaStandardTime,
    #[doc = "(UTC+02:00) Windhoek"]
    NamibiaStandardTime,
    #[doc = "(UTC+03:00) Baghdad"]
    ArabicStandardTime,
    #[doc = "(UTC+03:00) Istanbul"]
    TurkeyStandardTime,
    #[doc = "(UTC+03:00) Kuwait, Riyadh"]
    ArabStandardTime,
    #[doc = "(UTC+03:00) Minsk"]
    BelarusStandardTime,
    #[doc = "(UTC+03:00) Moscow, St. Petersburg"]
    RussianStandardTime,
    #[doc = "(UTC+03:00) Nairobi"]
    EAfricaStandardTime,
    #[doc = "(UTC+03:30) Tehran"]
    IranStandardTime,
    #[doc = "(UTC+04:00) Abu Dhabi, Muscat"]
    ArabianStandardTime,
    #[doc = "(UTC+04:00) Astrakhan, Ulyanovsk"]
    AstrakhanStandardTime,
    #[doc = "(UTC+04:00) Baku"]
    AzerbaijanStandardTime,
    #[doc = "(UTC+04:00) Izhevsk, Samara"]
    RussiaTimeZone3,
    #[doc = "(UTC+04:00) Port Louis"]
    MauritiusStandardTime,
    #[doc = "(UTC+04:00) Saratov"]
    SaratovStandardTime,
    #[doc = "(UTC+04:00) Tbilisi"]
    GeorgianStandardTime,
    #[doc = "(UTC+04:00) Volgograd"]
    VolgogradStandardTime,
    #[doc = "(UTC+04:00) Yerevan"]
    CaucasusStandardTime,
    #[doc = "(UTC+04:30) Kabul"]
    AfghanistanStandardTime,
    #[doc = "(UTC+05:00) Ashgabat, Tashkent"]
    WestAsiaStandardTime,
    #[doc = "(UTC+05:00) Ekaterinburg"]
    EkaterinburgStandardTime,
    #[doc = "(UTC+05:00) Islamabad, Karachi"]
    PakistanStandardTime,
    #[doc = "(UTC+05:00) Qyzylorda"]
    QyzylordaStandardTime,
    #[doc = "(UTC+05:30) Chennai, Kolkata, Mumbai, New Delhi"]
    IndiaStandardTime,
    #[doc = "(UTC+05:30) Sri Jayawardenepura"]
    SriLankaStandardTime,
    #[doc = "(UTC+05:45) Kathmandu"]
    NepalStandardTime,
    #[doc = "(UTC+06:00) Astana"]
    CentralAsiaStandardTime,
    #[doc = "(UTC+06:00) Dhaka"]
    BangladeshStandardTime,
    #[doc = "(UTC+06:00) Omsk"]
    OmskStandardTime,
    #[doc = "(UTC+06:30) Yangon (Rangoon)"]
    MyanmarStandardTime,
    #[doc = "(UTC+07:00) Bangkok, Hanoi, Jakarta"]
    SeAsiaStandardTime,
    #[doc = "(UTC+07:00) Barnaul, Gorno-Altaysk"]
    AltaiStandardTime,
    #[doc = "(UTC+07:00) Hovd"]
    WMongoliaStandardTime,
    #[doc = "(UTC+07:00) Krasnoyarsk"]
    NorthAsiaStandardTime,
    #[doc = "(UTC+07:00) Novosibirsk"]
    NCentralAsiaStandardTime,
    #[doc = "(UTC+07:00) Tomsk"]
    TomskStandardTime,
    #[doc = "(UTC+08:00) Beijing, Chongqing, Hong Kong, Urumqi"]
    ChinaStandardTime,
    #[doc = "(UTC+08:00) Irkutsk"]
    NorthAsiaEastStandardTime,
    #[doc = "(UTC+08:00) Kuala Lumpur, Singapore"]
    SingaporeStandardTime,
    #[doc = "(UTC+08:00) Perth"]
    WAustraliaStandardTime,
    #[doc = "(UTC+08:00) Taipei"]
    TaipeiStandardTime,
    #[doc = "(UTC+08:00) Ulaanbaatar"]
    UlaanbaatarStandardTime,
    #[doc = "(UTC+08:45) Eucla"]
    AusCentralWStandardTime,
    #[doc = "(UTC+09:00) Chita"]
    TransbaikalStandardTime,
    #[doc = "(UTC+09:00) Osaka, Sapporo, Tokyo"]
    TokyoStandardTime,
    #[doc = "(UTC+09:00) Pyongyang"]
    NorthKoreaStandardTime,
    #[doc = "(UTC+09:00) Seoul"]
    KoreaStandardTime,
    #[doc = "(UTC+09:00) Yakutsk"]
    YakutskStandardTime,
    #[doc = "(UTC+09:30) Adelaide"]
    CenAustraliaStandardTime,
    #[doc = "(UTC+09:30) Darwin"]
    AusCentralStandardTime,
    #[doc = "(UTC+10:00) Brisbane"]
    EAustraliaStandardTime,
    #[doc = "(UTC+10:00) Canberra, Melbourne, Sydney"]
    AusEasternStandardTime,
    #[doc = "(UTC+10:00) Guam, Port Moresby"]
    WestPacificStandardTime,
    #[doc = "(UTC+10:00) Hobart"]
    TasmaniaStandardTime,
    #[doc = "(UTC+10:00) Vladivostok"]
    VladivostokStandardTime,
    #[doc = "(UTC+10:30) Lord Howe Island"]
    LordHoweStandardTime,
    #[doc = "(UTC+11:00) Bougainville Island"]
    BougainvilleStandardTime,
    #[doc = "(UTC+11:00) Chokurdakh"]
    RussiaTimeZone10,
    #[doc = "(UTC+11:00) Magadan"]
    MagadanStandardTime,
    #[doc = "(UTC+11:00) Norfolk Island"]
    NorfolkStandardTime,
    #[doc = "(UTC+11:00) Sakhalin"]
    SakhalinStandardTime,
    #[doc = "(UTC+11:00) Solomon Is., New Caledonia"]
    CentralPacificStandardTime,
    #[doc = "(UTC+12:00) Anadyr, Petropavlovsk-Kamchatsky"]
    RussiaTimeZone11,
    #[doc = "(UTC+12:00) Auckland, Wellington"]
    NewZealandStandardTime,
    #[doc = "(UTC+12:00) Coordinated Universal Time+12"]
    UtcPlus12,
    #[doc = "(UTC+12:00) Fiji"]
    FijiStandardTime,
    #[doc = "(UTC+12:45) Chatham Islands"]
    ChathamIslandsStandardTime,
    #[doc = "(UTC+13:00) Coordinated Universal Time+13"]
    UtcPlus13,
    #[doc = "(UTC+13:00) Nuku'alofa"]
    TongaStandardTime,
    #[doc = "(UTC+13:00) Samoa"]
    SamoaStandardTime,
    #[doc = "(UTC+14:00) Kiritimati Island"]
    LineIslandsStandardTime,
}
impl Timezone {
    pub fn description(self) -> &'static str {
        match self {
            Self::DatelineStandardTime => "(UTC-12:00) International Date Line West",
            Self::UtcMinus11 => "(UTC-11:00) Coordinated Universal Time-11",
            Self::AleutianStandardTime => "(UTC-10:00) Aleutian Islands",
            Self::HawaiianStandardTime => "(UTC-10:00) Hawaii",
            Self::MarquesasStandardTime => "(UTC-09:30) Marquesas Islands",
            Self::AlaskanStandardTime => "(UTC-09:00) Alaska",
            Self::UtcMinus09 => "(UTC-09:00) Coordinated Universal Time-09",
            Self::PacificStandardTimeMexico => "(UTC-08:00) Baja California",
            Self::UtcMinus08 => "(UTC-08:00) Coordinated Universal Time-08",
            Self::PacificStandardTime => "(UTC-08:00) Pacific Time (US & Canada)",
            Self::UsMountainStandardTime => "(UTC-07:00) Arizona",
            Self::MountainStandardTimeMexico => "(UTC-07:00) Chihuahua, La Paz, Mazatlan",
            Self::MountainStandardTime => "(UTC-07:00) Mountain Time (US & Canada)",
            Self::YukonStandardTime => "(UTC-07:00) Yukon",
            Self::CentralAmericaStandardTime => "(UTC-06:00) Central America",
            Self::CentralStandardTime => "(UTC-06:00) Central Time (US & Canada)",
            Self::EasterIslandStandardTime => "(UTC-06:00) Easter Island",
            Self::CentralStandardTimeMexico => "(UTC-06:00) Guadalajara, Mexico City, Monterrey",
            Self::CanadaCentralStandardTime => "(UTC-06:00) Saskatchewan",
            Self::SaPacificStandardTime => "(UTC-05:00) Bogota, Lima, Quito, Rio Branco",
            Self::EasternStandardTimeMexico => "(UTC-05:00) Chetumal",
            Self::EasternStandardTime => "(UTC-05:00) Eastern Time (US & Canada)",
            Self::HaitiStandardTime => "(UTC-05:00) Haiti",
            Self::CubaStandardTime => "(UTC-05:00) Havana",
            Self::UsEasternStandardTime => "(UTC-05:00) Indiana (East)",
            Self::TurksAndCaicosStandardTime => "(UTC-05:00) Turks and Caicos",
            Self::ParaguayStandardTime => "(UTC-04:00) Asuncion",
            Self::AtlanticStandardTime => "(UTC-04:00) Atlantic Time (Canada)",
            Self::VenezuelaStandardTime => "(UTC-04:00) Caracas",
            Self::CentralBrazilianStandardTime => "(UTC-04:00) Cuiaba",
            Self::SaWesternStandardTime => "(UTC-04:00) Georgetown, La Paz, Manaus, San Juan",
            Self::PacificSaStandardTime => "(UTC-04:00) Santiago",
            Self::NewfoundlandStandardTime => "(UTC-03:30) Newfoundland",
            Self::TocantinsStandardTime => "(UTC-03:00) Araguaina",
            Self::ESouthAmericaStandardTime => "(UTC-03:00) Brasilia",
            Self::SaEasternStandardTime => "(UTC-03:00) Cayenne, Fortaleza",
            Self::ArgentinaStandardTime => "(UTC-03:00) City of Buenos Aires",
            Self::GreenlandStandardTime => "(UTC-03:00) Greenland",
            Self::MontevideoStandardTime => "(UTC-03:00) Montevideo",
            Self::MagallanesStandardTime => "(UTC-03:00) Punta Arenas",
            Self::SaintPierreStandardTime => "(UTC-03:00) Saint Pierre and Miquelon",
            Self::BahiaStandardTime => "(UTC-03:00) Salvador",
            Self::UtcMinus02 => "(UTC-02:00) Coordinated Universal Time-02",
            Self::AzoresStandardTime => "(UTC-01:00) Azores",
            Self::CapeVerdeStandardTime => "(UTC-01:00) Cabo Verde Is.",
            Self::Utc => "(UTC) Coordinated Universal Time",
            Self::GmtStandardTime => "(UTC+00:00) Dublin, Edinburgh, Lisbon, London",
            Self::GreenwichStandardTime => "(UTC+00:00) Monrovia, Reykjavik",
            Self::SaoTomeStandardTime => "(UTC+00:00) Sao Tome",
            Self::MoroccoStandardTime => "(UTC+01:00) Casablanca",
            Self::WEuropeStandardTime => {
                "(UTC+01:00) Amsterdam, Berlin, Bern, Rome, Stockholm, Vienna"
            }
            Self::CentralEuropeStandardTime => {
                "(UTC+01:00) Belgrade, Bratislava, Budapest, Ljubljana, Prague"
            }
            Self::RomanceStandardTime => "(UTC+01:00) Brussels, Copenhagen, Madrid, Paris",
            Self::CentralEuropeanStandardTime => "(UTC+01:00) Sarajevo, Skopje, Warsaw, Zagreb",
            Self::WCentralAfricaStandardTime => "(UTC+01:00) West Central Africa",
            Self::JordanStandardTime => "(UTC+02:00) Amman",
            Self::GtbStandardTime => "(UTC+02:00) Athens, Bucharest",
            Self::MiddleEastStandardTime => "(UTC+02:00) Beirut",
            Self::EgyptStandardTime => "(UTC+02:00) Cairo",
            Self::EEuropeStandardTime => "(UTC+02:00) Chisinau",
            Self::SyriaStandardTime => "(UTC+02:00) Damascus",
            Self::WestBankStandardTime => "(UTC+02:00) Gaza, Hebron",
            Self::SouthAfricaStandardTime => "(UTC+02:00) Harare, Pretoria",
            Self::FleStandardTime => "(UTC+02:00) Helsinki, Kyiv, Riga, Sofia, Tallinn, Vilnius",
            Self::IsraelStandardTime => "(UTC+02:00) Jerusalem",
            Self::SouthSudanStandardTime => "(UTC+02:00) Juba",
            Self::KaliningradStandardTime => "(UTC+02:00) Kaliningrad",
            Self::SudanStandardTime => "(UTC+02:00) Khartoum",
            Self::LibyaStandardTime => "(UTC+02:00) Tripoli",
            Self::NamibiaStandardTime => "(UTC+02:00) Windhoek",
            Self::ArabicStandardTime => "(UTC+03:00) Baghdad",
            Self::TurkeyStandardTime => "(UTC+03:00) Istanbul",
            Self::ArabStandardTime => "(UTC+03:00) Kuwait, Riyadh",
            Self::BelarusStandardTime => "(UTC+03:00) Minsk",
            Self::RussianStandardTime => "(UTC+03:00) Moscow, St. Petersburg",
            Self::EAfricaStandardTime => "(UTC+03:00) Nairobi",
            Self::IranStandardTime => "(UTC+03:30) Tehran",
            Self::ArabianStandardTime => "(UTC+04:00) Abu Dhabi, Muscat",
            Self::AstrakhanStandardTime => "(UTC+04:00) Astrakhan, Ulyanovsk",
            Self::AzerbaijanStandardTime => "(UTC+04:00) Baku",
            Self::RussiaTimeZone3 => "(UTC+04:00) Izhevsk, Samara",
            Self::MauritiusStandardTime => "(UTC+04:00) Port Louis",
            Self::SaratovStandardTime => "(UTC+04:00) Saratov",
            Self::GeorgianStandardTime => "(UTC+04:00) Tbilisi",
            Self::VolgogradStandardTime => "(UTC+04:00) Volgograd",
            Self::CaucasusStandardTime => "(UTC+04:00) Yerevan",
            Self::AfghanistanStandardTime => "(UTC+04:30) Kabul",
            Self::WestAsiaStandardTime => "(UTC+05:00) Ashgabat, Tashkent",
            Self::EkaterinburgStandardTime => "(UTC+05:00) Ekaterinburg",
            Self::PakistanStandardTime => "(UTC+05:00) Islamabad, Karachi",
            Self::QyzylordaStandardTime => "(UTC+05:00) Qyzylorda",
            Self::IndiaStandardTime => "(UTC+05:30) Chennai, Kolkata, Mumbai, New Delhi",
            Self::SriLankaStandardTime => "(UTC+05:30) Sri Jayawardenepura",
            Self::NepalStandardTime => "(UTC+05:45) Kathmandu",
            Self::CentralAsiaStandardTime => "(UTC+06:00) Astana",
            Self::BangladeshStandardTime => "(UTC+06:00) Dhaka",
            Self::OmskStandardTime => "(UTC+06:00) Omsk",
            Self::MyanmarStandardTime => "(UTC+06:30) Yangon (Rangoon)",
            Self::SeAsiaStandardTime => "(UTC+07:00) Bangkok, Hanoi, Jakarta",
            Self::AltaiStandardTime => "(UTC+07:00) Barnaul, Gorno-Altaysk",
            Self::WMongoliaStandardTime => "(UTC+07:00) Hovd",
            Self::NorthAsiaStandardTime => "(UTC+07:00) Krasnoyarsk",
            Self::NCentralAsiaStandardTime => "(UTC+07:00) Novosibirsk",
            Self::TomskStandardTime => "(UTC+07:00) Tomsk",
            Self::ChinaStandardTime => "(UTC+08:00) Beijing, Chongqing, Hong Kong, Urumqi",
            Self::NorthAsiaEastStandardTime => "(UTC+08:00) Irkutsk",
            Self::SingaporeStandardTime => "(UTC+08:00) Kuala Lumpur, Singapore",
            Self::WAustraliaStandardTime => "(UTC+08:00) Perth",
            Self::TaipeiStandardTime => "(UTC+08:00) Taipei",
            Self::UlaanbaatarStandardTime => "(UTC+08:00) Ulaanbaatar",
            Self::AusCentralWStandardTime => "(UTC+08:45) Eucla",
            Self::TransbaikalStandardTime => "(UTC+09:00) Chita",
            Self::TokyoStandardTime => "(UTC+09:00) Osaka, Sapporo, Tokyo",
            Self::NorthKoreaStandardTime => "(UTC+09:00) Pyongyang",
            Self::KoreaStandardTime => "(UTC+09:00) Seoul",
            Self::YakutskStandardTime => "(UTC+09:00) Yakutsk",
            Self::CenAustraliaStandardTime => "(UTC+09:30) Adelaide",
            Self::AusCentralStandardTime => "(UTC+09:30) Darwin",
            Self::EAustraliaStandardTime => "(UTC+10:00) Brisbane",
            Self::AusEasternStandardTime => "(UTC+10:00) Canberra, Melbourne, Sydney",
            Self::WestPacificStandardTime => "(UTC+10:00) Guam, Port Moresby",
            Self::TasmaniaStandardTime => "(UTC+10:00) Hobart",
            Self::VladivostokStandardTime => "(UTC+10:00) Vladivostok",
            Self::LordHoweStandardTime => "(UTC+10:30) Lord Howe Island",
            Self::BougainvilleStandardTime => "(UTC+11:00) Bougainville Island",
            Self::RussiaTimeZone10 => "(UTC+11:00) Chokurdakh",
            Self::MagadanStandardTime => "(UTC+11:00) Magadan",
            Self::NorfolkStandardTime => "(UTC+11:00) Norfolk Island",
            Self::SakhalinStandardTime => "(UTC+11:00) Sakhalin",
            Self::CentralPacificStandardTime => "(UTC+11:00) Solomon Is., New Caledonia",
            Self::RussiaTimeZone11 => "(UTC+12:00) Anadyr, Petropavlovsk-Kamchatsky",
            Self::NewZealandStandardTime => "(UTC+12:00) Auckland, Wellington",
            Self::UtcPlus12 => "(UTC+12:00) Coordinated Universal Time+12",
            Self::FijiStandardTime => "(UTC+12:00) Fiji",
            Self::ChathamIslandsStandardTime => "(UTC+12:45) Chatham Islands",
            Self::UtcPlus13 => "(UTC+13:00) Coordinated Universal Time+13",
            Self::TongaStandardTime => "(UTC+13:00) Nuku'alofa",
            Self::SamoaStandardTime => "(UTC+13:00) Samoa",
            Self::LineIslandsStandardTime => "(UTC+14:00) Kiritimati Island",
        }
    }
    pub fn tzdb_id(self) -> &'static str {
        match self {
            Self::DatelineStandardTime => "Etc/GMT+12",
            Self::UtcMinus11 => "Etc/GMT+11",
            Self::AleutianStandardTime => "America/Adak",
            Self::HawaiianStandardTime => "Pacific/Honolulu",
            Self::MarquesasStandardTime => "Pacific/Marquesas",
            Self::AlaskanStandardTime => "America/Anchorage",
            Self::UtcMinus09 => "Etc/GMT+9",
            Self::PacificStandardTimeMexico => "America/Tijuana",
            Self::UtcMinus08 => "Etc/GMT+8",
            Self::PacificStandardTime => "America/Los_Angeles",
            Self::UsMountainStandardTime => "America/Phoenix",
            Self::MountainStandardTimeMexico => "America/Chihuahua",
            Self::MountainStandardTime => "America/Denver",
            Self::YukonStandardTime => "America/Whitehorse",
            Self::CentralAmericaStandardTime => "America/Guatemala",
            Self::CentralStandardTime => "America/Chicago",
            Self::EasterIslandStandardTime => "Pacific/Easter",
            Self::CentralStandardTimeMexico => "America/Mexico_City",
            Self::CanadaCentralStandardTime => "America/Regina",
            Self::SaPacificStandardTime => "America/Bogota",
            Self::EasternStandardTimeMexico => "America/Cancun",
            Self::EasternStandardTime => "America/New_York",
            Self::HaitiStandardTime => "America/Port-au-Prince",
            Self::CubaStandardTime => "America/Havana",
            Self::UsEasternStandardTime => "America/Indianapolis",
            Self::TurksAndCaicosStandardTime => "America/Grand_Turk",
            Self::ParaguayStandardTime => "America/Asuncion",
            Self::AtlanticStandardTime => "America/Halifax",
            Self::VenezuelaStandardTime => "America/Caracas",
            Self::CentralBrazilianStandardTime => "America/Cuiaba",
            Self::SaWesternStandardTime => "America/La_Paz",
            Self::PacificSaStandardTime => "America/Santiago",
            Self::NewfoundlandStandardTime => "America/St_Johns",
            Self::TocantinsStandardTime => "America/Araguaina",
            Self::ESouthAmericaStandardTime => "America/Sao_Paulo",
            Self::SaEasternStandardTime => "America/Cayenne",
            Self::ArgentinaStandardTime => "America/Buenos_Aires",
            Self::GreenlandStandardTime => "America/Godthab",
            Self::MontevideoStandardTime => "America/Montevideo",
            Self::MagallanesStandardTime => "America/Punta_Arenas",
            Self::SaintPierreStandardTime => "America/Miquelon",
            Self::BahiaStandardTime => "America/Bahia",
            Self::UtcMinus02 => "Etc/GMT+2",
            Self::AzoresStandardTime => "Atlantic/Azores",
            Self::CapeVerdeStandardTime => "Atlantic/Cape_Verde",
            Self::Utc => "Etc/UTC",
            Self::GmtStandardTime => "Europe/London",
            Self::GreenwichStandardTime => "Atlantic/Reykjavik",
            Self::SaoTomeStandardTime => "Africa/Sao_Tome",
            Self::MoroccoStandardTime => "Africa/Casablanca",
            Self::WEuropeStandardTime => "Europe/Berlin",
            Self::CentralEuropeStandardTime => "Europe/Budapest",
            Self::RomanceStandardTime => "Europe/Paris",
            Self::CentralEuropeanStandardTime => "Europe/Warsaw",
            Self::WCentralAfricaStandardTime => "Africa/Lagos",
            Self::JordanStandardTime => "Asia/Amman",
            Self::GtbStandardTime => "Europe/Bucharest",
            Self::MiddleEastStandardTime => "Asia/Beirut",
            Self::EgyptStandardTime => "Africa/Cairo",
            Self::EEuropeStandardTime => "Europe/Chisinau",
            Self::SyriaStandardTime => "Asia/Damascus",
            Self::WestBankStandardTime => "Asia/Hebron",
            Self::SouthAfricaStandardTime => "Africa/Johannesburg",
            Self::FleStandardTime => "Europe/Kiev",
            Self::IsraelStandardTime => "Asia/Jerusalem",
            Self::SouthSudanStandardTime => "Africa/Juba",
            Self::KaliningradStandardTime => "Europe/Kaliningrad",
            Self::SudanStandardTime => "Africa/Khartoum",
            Self::LibyaStandardTime => "Africa/Tripoli",
            Self::NamibiaStandardTime => "Africa/Windhoek",
            Self::ArabicStandardTime => "Asia/Baghdad",
            Self::TurkeyStandardTime => "Europe/Istanbul",
            Self::ArabStandardTime => "Asia/Riyadh",
            Self::BelarusStandardTime => "Europe/Minsk",
            Self::RussianStandardTime => "Europe/Moscow",
            Self::EAfricaStandardTime => "Africa/Nairobi",
            Self::IranStandardTime => "Asia/Tehran",
            Self::ArabianStandardTime => "Asia/Dubai",
            Self::AstrakhanStandardTime => "Europe/Astrakhan",
            Self::AzerbaijanStandardTime => "Asia/Baku",
            Self::RussiaTimeZone3 => "Europe/Samara",
            Self::MauritiusStandardTime => "Indian/Mauritius",
            Self::SaratovStandardTime => "Europe/Saratov",
            Self::GeorgianStandardTime => "Asia/Tbilisi",
            Self::VolgogradStandardTime => "Europe/Volgograd",
            Self::CaucasusStandardTime => "Asia/Yerevan",
            Self::AfghanistanStandardTime => "Asia/Kabul",
            Self::WestAsiaStandardTime => "Asia/Tashkent",
            Self::EkaterinburgStandardTime => "Asia/Yekaterinburg",
            Self::PakistanStandardTime => "Asia/Karachi",
            Self::QyzylordaStandardTime => "Asia/Qyzylorda",
            Self::IndiaStandardTime => "Asia/Calcutta",
            Self::SriLankaStandardTime => "Asia/Colombo",
            Self::NepalStandardTime => "Asia/Katmandu",
            Self::CentralAsiaStandardTime => "Asia/Almaty",
            Self::BangladeshStandardTime => "Asia/Dhaka",
            Self::OmskStandardTime => "Asia/Omsk",
            Self::MyanmarStandardTime => "Asia/Rangoon",
            Self::SeAsiaStandardTime => "Asia/Bangkok",
            Self::AltaiStandardTime => "Asia/Barnaul",
            Self::WMongoliaStandardTime => "Asia/Hovd",
            Self::NorthAsiaStandardTime => "Asia/Krasnoyarsk",
            Self::NCentralAsiaStandardTime => "Asia/Novosibirsk",
            Self::TomskStandardTime => "Asia/Tomsk",
            Self::ChinaStandardTime => "Asia/Shanghai",
            Self::NorthAsiaEastStandardTime => "Asia/Irkutsk",
            Self::SingaporeStandardTime => "Asia/Singapore",
            Self::WAustraliaStandardTime => "Australia/Perth",
            Self::TaipeiStandardTime => "Asia/Taipei",
            Self::UlaanbaatarStandardTime => "Asia/Ulaanbaatar",
            Self::AusCentralWStandardTime => "Australia/Eucla",
            Self::TransbaikalStandardTime => "Asia/Chita",
            Self::TokyoStandardTime => "Asia/Tokyo",
            Self::NorthKoreaStandardTime => "Asia/Pyongyang",
            Self::KoreaStandardTime => "Asia/Seoul",
            Self::YakutskStandardTime => "Asia/Yakutsk",
            Self::CenAustraliaStandardTime => "Australia/Adelaide",
            Self::AusCentralStandardTime => "Australia/Darwin",
            Self::EAustraliaStandardTime => "Australia/Brisbane",
            Self::AusEasternStandardTime => "Australia/Sydney",
            Self::WestPacificStandardTime => "Pacific/Port_Moresby",
            Self::TasmaniaStandardTime => "Australia/Hobart",
            Self::VladivostokStandardTime => "Asia/Vladivostok",
            Self::LordHoweStandardTime => "Australia/Lord_Howe",
            Self::BougainvilleStandardTime => "Pacific/Bougainville",
            Self::RussiaTimeZone10 => "Asia/Srednekolymsk",
            Self::MagadanStandardTime => "Asia/Magadan",
            Self::NorfolkStandardTime => "Pacific/Norfolk",
            Self::SakhalinStandardTime => "Asia/Sakhalin",
            Self::CentralPacificStandardTime => "Pacific/Guadalcanal",
            Self::RussiaTimeZone11 => "Asia/Kamchatka",
            Self::NewZealandStandardTime => "Pacific/Auckland",
            Self::UtcPlus12 => "Etc/GMT-12",
            Self::FijiStandardTime => "Pacific/Fiji",
            Self::ChathamIslandsStandardTime => "Pacific/Chatham",
            Self::UtcPlus13 => "Etc/GMT-13",
            Self::TongaStandardTime => "Pacific/Tongatapu",
            Self::SamoaStandardTime => "Pacific/Apia",
            Self::LineIslandsStandardTime => "Pacific/Kiritimati",
        }
    }
}
#[cfg(feature = "chrono-tz")]
impl From<Timezone> for ::chrono_tz::Tz {
    fn from(value: Timezone) -> Self {
        match value {
            Timezone::DatelineStandardTime => ::chrono_tz::Tz::Etc__GMTPlus12,
            Timezone::UtcMinus11 => ::chrono_tz::Tz::Etc__GMTPlus11,
            Timezone::AleutianStandardTime => ::chrono_tz::Tz::America__Adak,
            Timezone::HawaiianStandardTime => ::chrono_tz::Tz::Pacific__Honolulu,
            Timezone::MarquesasStandardTime => ::chrono_tz::Tz::Pacific__Marquesas,
            Timezone::AlaskanStandardTime => ::chrono_tz::Tz::America__Anchorage,
            Timezone::UtcMinus09 => ::chrono_tz::Tz::Etc__GMTPlus9,
            Timezone::PacificStandardTimeMexico => ::chrono_tz::Tz::America__Tijuana,
            Timezone::UtcMinus08 => ::chrono_tz::Tz::Etc__GMTPlus8,
            Timezone::PacificStandardTime => ::chrono_tz::Tz::America__Los_Angeles,
            Timezone::UsMountainStandardTime => ::chrono_tz::Tz::America__Phoenix,
            Timezone::MountainStandardTimeMexico => ::chrono_tz::Tz::America__Chihuahua,
            Timezone::MountainStandardTime => ::chrono_tz::Tz::America__Denver,
            Timezone::YukonStandardTime => ::chrono_tz::Tz::America__Whitehorse,
            Timezone::CentralAmericaStandardTime => ::chrono_tz::Tz::America__Guatemala,
            Timezone::CentralStandardTime => ::chrono_tz::Tz::America__Chicago,
            Timezone::EasterIslandStandardTime => ::chrono_tz::Tz::Pacific__Easter,
            Timezone::CentralStandardTimeMexico => ::chrono_tz::Tz::America__Mexico_City,
            Timezone::CanadaCentralStandardTime => ::chrono_tz::Tz::America__Regina,
            Timezone::SaPacificStandardTime => ::chrono_tz::Tz::America__Bogota,
            Timezone::EasternStandardTimeMexico => ::chrono_tz::Tz::America__Cancun,
            Timezone::EasternStandardTime => ::chrono_tz::Tz::America__New_York,
            Timezone::HaitiStandardTime => ::chrono_tz::Tz::America__PortauPrince,
            Timezone::CubaStandardTime => ::chrono_tz::Tz::America__Havana,
            Timezone::UsEasternStandardTime => ::chrono_tz::Tz::America__Indianapolis,
            Timezone::TurksAndCaicosStandardTime => ::chrono_tz::Tz::America__Grand_Turk,
            Timezone::ParaguayStandardTime => ::chrono_tz::Tz::America__Asuncion,
            Timezone::AtlanticStandardTime => ::chrono_tz::Tz::America__Halifax,
            Timezone::VenezuelaStandardTime => ::chrono_tz::Tz::America__Caracas,
            Timezone::CentralBrazilianStandardTime => ::chrono_tz::Tz::America__Cuiaba,
            Timezone::SaWesternStandardTime => ::chrono_tz::Tz::America__La_Paz,
            Timezone::PacificSaStandardTime => ::chrono_tz::Tz::America__Santiago,
            Timezone::NewfoundlandStandardTime => ::chrono_tz::Tz::America__St_Johns,
            Timezone::TocantinsStandardTime => ::chrono_tz::Tz::America__Araguaina,
            Timezone::ESouthAmericaStandardTime => ::chrono_tz::Tz::America__Sao_Paulo,
            Timezone::SaEasternStandardTime => ::chrono_tz::Tz::America__Cayenne,
            Timezone::ArgentinaStandardTime => ::chrono_tz::Tz::America__Buenos_Aires,
            Timezone::GreenlandStandardTime => ::chrono_tz::Tz::America__Godthab,
            Timezone::MontevideoStandardTime => ::chrono_tz::Tz::America__Montevideo,
            Timezone::MagallanesStandardTime => ::chrono_tz::Tz::America__Punta_Arenas,
            Timezone::SaintPierreStandardTime => ::chrono_tz::Tz::America__Miquelon,
            Timezone::BahiaStandardTime => ::chrono_tz::Tz::America__Bahia,
            Timezone::UtcMinus02 => ::chrono_tz::Tz::Etc__GMTPlus2,
            Timezone::AzoresStandardTime => ::chrono_tz::Tz::Atlantic__Azores,
            Timezone::CapeVerdeStandardTime => ::chrono_tz::Tz::Atlantic__Cape_Verde,
            Timezone::Utc => ::chrono_tz::Tz::Etc__UTC,
            Timezone::GmtStandardTime => ::chrono_tz::Tz::Europe__London,
            Timezone::GreenwichStandardTime => ::chrono_tz::Tz::Atlantic__Reykjavik,
            Timezone::SaoTomeStandardTime => ::chrono_tz::Tz::Africa__Sao_Tome,
            Timezone::MoroccoStandardTime => ::chrono_tz::Tz::Africa__Casablanca,
            Timezone::WEuropeStandardTime => ::chrono_tz::Tz::Europe__Berlin,
            Timezone::CentralEuropeStandardTime => ::chrono_tz::Tz::Europe__Budapest,
            Timezone::RomanceStandardTime => ::chrono_tz::Tz::Europe__Paris,
            Timezone::CentralEuropeanStandardTime => ::chrono_tz::Tz::Europe__Warsaw,
            Timezone::WCentralAfricaStandardTime => ::chrono_tz::Tz::Africa__Lagos,
            Timezone::JordanStandardTime => ::chrono_tz::Tz::Asia__Amman,
            Timezone::GtbStandardTime => ::chrono_tz::Tz::Europe__Bucharest,
            Timezone::MiddleEastStandardTime => ::chrono_tz::Tz::Asia__Beirut,
            Timezone::EgyptStandardTime => ::chrono_tz::Tz::Africa__Cairo,
            Timezone::EEuropeStandardTime => ::chrono_tz::Tz::Europe__Chisinau,
            Timezone::SyriaStandardTime => ::chrono_tz::Tz::Asia__Damascus,
            Timezone::WestBankStandardTime => ::chrono_tz::Tz::Asia__Hebron,
            Timezone::SouthAfricaStandardTime => ::chrono_tz::Tz::Africa__Johannesburg,
            Timezone::FleStandardTime => ::chrono_tz::Tz::Europe__Kiev,
            Timezone::IsraelStandardTime => ::chrono_tz::Tz::Asia__Jerusalem,
            Timezone::SouthSudanStandardTime => ::chrono_tz::Tz::Africa__Juba,
            Timezone::KaliningradStandardTime => ::chrono_tz::Tz::Europe__Kaliningrad,
            Timezone::SudanStandardTime => ::chrono_tz::Tz::Africa__Khartoum,
            Timezone::LibyaStandardTime => ::chrono_tz::Tz::Africa__Tripoli,
            Timezone::NamibiaStandardTime => ::chrono_tz::Tz::Africa__Windhoek,
            Timezone::ArabicStandardTime => ::chrono_tz::Tz::Asia__Baghdad,
            Timezone::TurkeyStandardTime => ::chrono_tz::Tz::Europe__Istanbul,
            Timezone::ArabStandardTime => ::chrono_tz::Tz::Asia__Riyadh,
            Timezone::BelarusStandardTime => ::chrono_tz::Tz::Europe__Minsk,
            Timezone::RussianStandardTime => ::chrono_tz::Tz::Europe__Moscow,
            Timezone::EAfricaStandardTime => ::chrono_tz::Tz::Africa__Nairobi,
            Timezone::IranStandardTime => ::chrono_tz::Tz::Asia__Tehran,
            Timezone::ArabianStandardTime => ::chrono_tz::Tz::Asia__Dubai,
            Timezone::AstrakhanStandardTime => ::chrono_tz::Tz::Europe__Astrakhan,
            Timezone::AzerbaijanStandardTime => ::chrono_tz::Tz::Asia__Baku,
            Timezone::RussiaTimeZone3 => ::chrono_tz::Tz::Europe__Samara,
            Timezone::MauritiusStandardTime => ::chrono_tz::Tz::Indian__Mauritius,
            Timezone::SaratovStandardTime => ::chrono_tz::Tz::Europe__Saratov,
            Timezone::GeorgianStandardTime => ::chrono_tz::Tz::Asia__Tbilisi,
            Timezone::VolgogradStandardTime => ::chrono_tz::Tz::Europe__Volgograd,
            Timezone::CaucasusStandardTime => ::chrono_tz::Tz::Asia__Yerevan,
            Timezone::AfghanistanStandardTime => ::chrono_tz::Tz::Asia__Kabul,
            Timezone::WestAsiaStandardTime => ::chrono_tz::Tz::Asia__Tashkent,
            Timezone::EkaterinburgStandardTime => ::chrono_tz::Tz::Asia__Yekaterinburg,
            Timezone::PakistanStandardTime => ::chrono_tz::Tz::Asia__Karachi,
            Timezone::QyzylordaStandardTime => ::chrono_tz::Tz::Asia__Qyzylorda,
            Timezone::IndiaStandardTime => ::chrono_tz::Tz::Asia__Calcutta,
            Timezone::SriLankaStandardTime => ::chrono_tz::Tz::Asia__Colombo,
            Timezone::NepalStandardTime => ::chrono_tz::Tz::Asia__Katmandu,
            Timezone::CentralAsiaStandardTime => ::chrono_tz::Tz::Asia__Almaty,
            Timezone::BangladeshStandardTime => ::chrono_tz::Tz::Asia__Dhaka,
            Timezone::OmskStandardTime => ::chrono_tz::Tz::Asia__Omsk,
            Timezone::MyanmarStandardTime => ::chrono_tz::Tz::Asia__Rangoon,
            Timezone::SeAsiaStandardTime => ::chrono_tz::Tz::Asia__Bangkok,
            Timezone::AltaiStandardTime => ::chrono_tz::Tz::Asia__Barnaul,
            Timezone::WMongoliaStandardTime => ::chrono_tz::Tz::Asia__Hovd,
            Timezone::NorthAsiaStandardTime => ::chrono_tz::Tz::Asia__Krasnoyarsk,
            Timezone::NCentralAsiaStandardTime => ::chrono_tz::Tz::Asia__Novosibirsk,
            Timezone::TomskStandardTime => ::chrono_tz::Tz::Asia__Tomsk,
            Timezone::ChinaStandardTime => ::chrono_tz::Tz::Asia__Shanghai,
            Timezone::NorthAsiaEastStandardTime => ::chrono_tz::Tz::Asia__Irkutsk,
            Timezone::SingaporeStandardTime => ::chrono_tz::Tz::Asia__Singapore,
            Timezone::WAustraliaStandardTime => ::chrono_tz::Tz::Australia__Perth,
            Timezone::TaipeiStandardTime => ::chrono_tz::Tz::Asia__Taipei,
            Timezone::UlaanbaatarStandardTime => ::chrono_tz::Tz::Asia__Ulaanbaatar,
            Timezone::AusCentralWStandardTime => ::chrono_tz::Tz::Australia__Eucla,
            Timezone::TransbaikalStandardTime => ::chrono_tz::Tz::Asia__Chita,
            Timezone::TokyoStandardTime => ::chrono_tz::Tz::Asia__Tokyo,
            Timezone::NorthKoreaStandardTime => ::chrono_tz::Tz::Asia__Pyongyang,
            Timezone::KoreaStandardTime => ::chrono_tz::Tz::Asia__Seoul,
            Timezone::YakutskStandardTime => ::chrono_tz::Tz::Asia__Yakutsk,
            Timezone::CenAustraliaStandardTime => ::chrono_tz::Tz::Australia__Adelaide,
            Timezone::AusCentralStandardTime => ::chrono_tz::Tz::Australia__Darwin,
            Timezone::EAustraliaStandardTime => ::chrono_tz::Tz::Australia__Brisbane,
            Timezone::AusEasternStandardTime => ::chrono_tz::Tz::Australia__Sydney,
            Timezone::WestPacificStandardTime => ::chrono_tz::Tz::Pacific__Port_Moresby,
            Timezone::TasmaniaStandardTime => ::chrono_tz::Tz::Australia__Hobart,
            Timezone::VladivostokStandardTime => ::chrono_tz::Tz::Asia__Vladivostok,
            Timezone::LordHoweStandardTime => ::chrono_tz::Tz::Australia__Lord_Howe,
            Timezone::BougainvilleStandardTime => ::chrono_tz::Tz::Pacific__Bougainville,
            Timezone::RussiaTimeZone10 => ::chrono_tz::Tz::Asia__Srednekolymsk,
            Timezone::MagadanStandardTime => ::chrono_tz::Tz::Asia__Magadan,
            Timezone::NorfolkStandardTime => ::chrono_tz::Tz::Pacific__Norfolk,
            Timezone::SakhalinStandardTime => ::chrono_tz::Tz::Asia__Sakhalin,
            Timezone::CentralPacificStandardTime => ::chrono_tz::Tz::Pacific__Guadalcanal,
            Timezone::RussiaTimeZone11 => ::chrono_tz::Tz::Asia__Kamchatka,
            Timezone::NewZealandStandardTime => ::chrono_tz::Tz::Pacific__Auckland,
            Timezone::UtcPlus12 => ::chrono_tz::Tz::Etc__GMTMinus12,
            Timezone::FijiStandardTime => ::chrono_tz::Tz::Pacific__Fiji,
            Timezone::ChathamIslandsStandardTime => ::chrono_tz::Tz::Pacific__Chatham,
            Timezone::UtcPlus13 => ::chrono_tz::Tz::Etc__GMTMinus13,
            Timezone::TongaStandardTime => ::chrono_tz::Tz::Pacific__Tongatapu,
            Timezone::SamoaStandardTime => ::chrono_tz::Tz::Pacific__Apia,
            Timezone::LineIslandsStandardTime => ::chrono_tz::Tz::Pacific__Kiritimati,
        }
    }
}

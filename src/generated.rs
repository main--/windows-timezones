#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "strum", derive(strum::EnumIter))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "sqlx",
    derive(sqlx::Type),
    sqlx(type_name = "windows_timezone", rename_all = "snake_case")
)]
pub enum WindowsTimezone {
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
impl WindowsTimezone {
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
            Self::MountainStandardTimeMexico => "America/Mazatlan",
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
impl From<WindowsTimezone> for ::chrono_tz::Tz {
    fn from(value: WindowsTimezone) -> Self {
        match value {
            WindowsTimezone::DatelineStandardTime => ::chrono_tz::Tz::Etc__GMTPlus12,
            WindowsTimezone::UtcMinus11 => ::chrono_tz::Tz::Etc__GMTPlus11,
            WindowsTimezone::AleutianStandardTime => ::chrono_tz::Tz::America__Adak,
            WindowsTimezone::HawaiianStandardTime => ::chrono_tz::Tz::Pacific__Honolulu,
            WindowsTimezone::MarquesasStandardTime => ::chrono_tz::Tz::Pacific__Marquesas,
            WindowsTimezone::AlaskanStandardTime => ::chrono_tz::Tz::America__Anchorage,
            WindowsTimezone::UtcMinus09 => ::chrono_tz::Tz::Etc__GMTPlus9,
            WindowsTimezone::PacificStandardTimeMexico => ::chrono_tz::Tz::America__Tijuana,
            WindowsTimezone::UtcMinus08 => ::chrono_tz::Tz::Etc__GMTPlus8,
            WindowsTimezone::PacificStandardTime => ::chrono_tz::Tz::America__Los_Angeles,
            WindowsTimezone::UsMountainStandardTime => ::chrono_tz::Tz::America__Phoenix,
            WindowsTimezone::MountainStandardTimeMexico => ::chrono_tz::Tz::America__Mazatlan,
            WindowsTimezone::MountainStandardTime => ::chrono_tz::Tz::America__Denver,
            WindowsTimezone::YukonStandardTime => ::chrono_tz::Tz::America__Whitehorse,
            WindowsTimezone::CentralAmericaStandardTime => ::chrono_tz::Tz::America__Guatemala,
            WindowsTimezone::CentralStandardTime => ::chrono_tz::Tz::America__Chicago,
            WindowsTimezone::EasterIslandStandardTime => ::chrono_tz::Tz::Pacific__Easter,
            WindowsTimezone::CentralStandardTimeMexico => ::chrono_tz::Tz::America__Mexico_City,
            WindowsTimezone::CanadaCentralStandardTime => ::chrono_tz::Tz::America__Regina,
            WindowsTimezone::SaPacificStandardTime => ::chrono_tz::Tz::America__Bogota,
            WindowsTimezone::EasternStandardTimeMexico => ::chrono_tz::Tz::America__Cancun,
            WindowsTimezone::EasternStandardTime => ::chrono_tz::Tz::America__New_York,
            WindowsTimezone::HaitiStandardTime => ::chrono_tz::Tz::America__PortauPrince,
            WindowsTimezone::CubaStandardTime => ::chrono_tz::Tz::America__Havana,
            WindowsTimezone::UsEasternStandardTime => ::chrono_tz::Tz::America__Indianapolis,
            WindowsTimezone::TurksAndCaicosStandardTime => ::chrono_tz::Tz::America__Grand_Turk,
            WindowsTimezone::ParaguayStandardTime => ::chrono_tz::Tz::America__Asuncion,
            WindowsTimezone::AtlanticStandardTime => ::chrono_tz::Tz::America__Halifax,
            WindowsTimezone::VenezuelaStandardTime => ::chrono_tz::Tz::America__Caracas,
            WindowsTimezone::CentralBrazilianStandardTime => ::chrono_tz::Tz::America__Cuiaba,
            WindowsTimezone::SaWesternStandardTime => ::chrono_tz::Tz::America__La_Paz,
            WindowsTimezone::PacificSaStandardTime => ::chrono_tz::Tz::America__Santiago,
            WindowsTimezone::NewfoundlandStandardTime => ::chrono_tz::Tz::America__St_Johns,
            WindowsTimezone::TocantinsStandardTime => ::chrono_tz::Tz::America__Araguaina,
            WindowsTimezone::ESouthAmericaStandardTime => ::chrono_tz::Tz::America__Sao_Paulo,
            WindowsTimezone::SaEasternStandardTime => ::chrono_tz::Tz::America__Cayenne,
            WindowsTimezone::ArgentinaStandardTime => ::chrono_tz::Tz::America__Buenos_Aires,
            WindowsTimezone::GreenlandStandardTime => ::chrono_tz::Tz::America__Godthab,
            WindowsTimezone::MontevideoStandardTime => ::chrono_tz::Tz::America__Montevideo,
            WindowsTimezone::MagallanesStandardTime => ::chrono_tz::Tz::America__Punta_Arenas,
            WindowsTimezone::SaintPierreStandardTime => ::chrono_tz::Tz::America__Miquelon,
            WindowsTimezone::BahiaStandardTime => ::chrono_tz::Tz::America__Bahia,
            WindowsTimezone::UtcMinus02 => ::chrono_tz::Tz::Etc__GMTPlus2,
            WindowsTimezone::AzoresStandardTime => ::chrono_tz::Tz::Atlantic__Azores,
            WindowsTimezone::CapeVerdeStandardTime => ::chrono_tz::Tz::Atlantic__Cape_Verde,
            WindowsTimezone::Utc => ::chrono_tz::Tz::Etc__UTC,
            WindowsTimezone::GmtStandardTime => ::chrono_tz::Tz::Europe__London,
            WindowsTimezone::GreenwichStandardTime => ::chrono_tz::Tz::Atlantic__Reykjavik,
            WindowsTimezone::SaoTomeStandardTime => ::chrono_tz::Tz::Africa__Sao_Tome,
            WindowsTimezone::MoroccoStandardTime => ::chrono_tz::Tz::Africa__Casablanca,
            WindowsTimezone::WEuropeStandardTime => ::chrono_tz::Tz::Europe__Berlin,
            WindowsTimezone::CentralEuropeStandardTime => ::chrono_tz::Tz::Europe__Budapest,
            WindowsTimezone::RomanceStandardTime => ::chrono_tz::Tz::Europe__Paris,
            WindowsTimezone::CentralEuropeanStandardTime => ::chrono_tz::Tz::Europe__Warsaw,
            WindowsTimezone::WCentralAfricaStandardTime => ::chrono_tz::Tz::Africa__Lagos,
            WindowsTimezone::JordanStandardTime => ::chrono_tz::Tz::Asia__Amman,
            WindowsTimezone::GtbStandardTime => ::chrono_tz::Tz::Europe__Bucharest,
            WindowsTimezone::MiddleEastStandardTime => ::chrono_tz::Tz::Asia__Beirut,
            WindowsTimezone::EgyptStandardTime => ::chrono_tz::Tz::Africa__Cairo,
            WindowsTimezone::EEuropeStandardTime => ::chrono_tz::Tz::Europe__Chisinau,
            WindowsTimezone::SyriaStandardTime => ::chrono_tz::Tz::Asia__Damascus,
            WindowsTimezone::WestBankStandardTime => ::chrono_tz::Tz::Asia__Hebron,
            WindowsTimezone::SouthAfricaStandardTime => ::chrono_tz::Tz::Africa__Johannesburg,
            WindowsTimezone::FleStandardTime => ::chrono_tz::Tz::Europe__Kiev,
            WindowsTimezone::IsraelStandardTime => ::chrono_tz::Tz::Asia__Jerusalem,
            WindowsTimezone::SouthSudanStandardTime => ::chrono_tz::Tz::Africa__Juba,
            WindowsTimezone::KaliningradStandardTime => ::chrono_tz::Tz::Europe__Kaliningrad,
            WindowsTimezone::SudanStandardTime => ::chrono_tz::Tz::Africa__Khartoum,
            WindowsTimezone::LibyaStandardTime => ::chrono_tz::Tz::Africa__Tripoli,
            WindowsTimezone::NamibiaStandardTime => ::chrono_tz::Tz::Africa__Windhoek,
            WindowsTimezone::ArabicStandardTime => ::chrono_tz::Tz::Asia__Baghdad,
            WindowsTimezone::TurkeyStandardTime => ::chrono_tz::Tz::Europe__Istanbul,
            WindowsTimezone::ArabStandardTime => ::chrono_tz::Tz::Asia__Riyadh,
            WindowsTimezone::BelarusStandardTime => ::chrono_tz::Tz::Europe__Minsk,
            WindowsTimezone::RussianStandardTime => ::chrono_tz::Tz::Europe__Moscow,
            WindowsTimezone::EAfricaStandardTime => ::chrono_tz::Tz::Africa__Nairobi,
            WindowsTimezone::IranStandardTime => ::chrono_tz::Tz::Asia__Tehran,
            WindowsTimezone::ArabianStandardTime => ::chrono_tz::Tz::Asia__Dubai,
            WindowsTimezone::AstrakhanStandardTime => ::chrono_tz::Tz::Europe__Astrakhan,
            WindowsTimezone::AzerbaijanStandardTime => ::chrono_tz::Tz::Asia__Baku,
            WindowsTimezone::RussiaTimeZone3 => ::chrono_tz::Tz::Europe__Samara,
            WindowsTimezone::MauritiusStandardTime => ::chrono_tz::Tz::Indian__Mauritius,
            WindowsTimezone::SaratovStandardTime => ::chrono_tz::Tz::Europe__Saratov,
            WindowsTimezone::GeorgianStandardTime => ::chrono_tz::Tz::Asia__Tbilisi,
            WindowsTimezone::VolgogradStandardTime => ::chrono_tz::Tz::Europe__Volgograd,
            WindowsTimezone::CaucasusStandardTime => ::chrono_tz::Tz::Asia__Yerevan,
            WindowsTimezone::AfghanistanStandardTime => ::chrono_tz::Tz::Asia__Kabul,
            WindowsTimezone::WestAsiaStandardTime => ::chrono_tz::Tz::Asia__Tashkent,
            WindowsTimezone::EkaterinburgStandardTime => ::chrono_tz::Tz::Asia__Yekaterinburg,
            WindowsTimezone::PakistanStandardTime => ::chrono_tz::Tz::Asia__Karachi,
            WindowsTimezone::QyzylordaStandardTime => ::chrono_tz::Tz::Asia__Qyzylorda,
            WindowsTimezone::IndiaStandardTime => ::chrono_tz::Tz::Asia__Calcutta,
            WindowsTimezone::SriLankaStandardTime => ::chrono_tz::Tz::Asia__Colombo,
            WindowsTimezone::NepalStandardTime => ::chrono_tz::Tz::Asia__Katmandu,
            WindowsTimezone::CentralAsiaStandardTime => ::chrono_tz::Tz::Asia__Almaty,
            WindowsTimezone::BangladeshStandardTime => ::chrono_tz::Tz::Asia__Dhaka,
            WindowsTimezone::OmskStandardTime => ::chrono_tz::Tz::Asia__Omsk,
            WindowsTimezone::MyanmarStandardTime => ::chrono_tz::Tz::Asia__Rangoon,
            WindowsTimezone::SeAsiaStandardTime => ::chrono_tz::Tz::Asia__Bangkok,
            WindowsTimezone::AltaiStandardTime => ::chrono_tz::Tz::Asia__Barnaul,
            WindowsTimezone::WMongoliaStandardTime => ::chrono_tz::Tz::Asia__Hovd,
            WindowsTimezone::NorthAsiaStandardTime => ::chrono_tz::Tz::Asia__Krasnoyarsk,
            WindowsTimezone::NCentralAsiaStandardTime => ::chrono_tz::Tz::Asia__Novosibirsk,
            WindowsTimezone::TomskStandardTime => ::chrono_tz::Tz::Asia__Tomsk,
            WindowsTimezone::ChinaStandardTime => ::chrono_tz::Tz::Asia__Shanghai,
            WindowsTimezone::NorthAsiaEastStandardTime => ::chrono_tz::Tz::Asia__Irkutsk,
            WindowsTimezone::SingaporeStandardTime => ::chrono_tz::Tz::Asia__Singapore,
            WindowsTimezone::WAustraliaStandardTime => ::chrono_tz::Tz::Australia__Perth,
            WindowsTimezone::TaipeiStandardTime => ::chrono_tz::Tz::Asia__Taipei,
            WindowsTimezone::UlaanbaatarStandardTime => ::chrono_tz::Tz::Asia__Ulaanbaatar,
            WindowsTimezone::AusCentralWStandardTime => ::chrono_tz::Tz::Australia__Eucla,
            WindowsTimezone::TransbaikalStandardTime => ::chrono_tz::Tz::Asia__Chita,
            WindowsTimezone::TokyoStandardTime => ::chrono_tz::Tz::Asia__Tokyo,
            WindowsTimezone::NorthKoreaStandardTime => ::chrono_tz::Tz::Asia__Pyongyang,
            WindowsTimezone::KoreaStandardTime => ::chrono_tz::Tz::Asia__Seoul,
            WindowsTimezone::YakutskStandardTime => ::chrono_tz::Tz::Asia__Yakutsk,
            WindowsTimezone::CenAustraliaStandardTime => ::chrono_tz::Tz::Australia__Adelaide,
            WindowsTimezone::AusCentralStandardTime => ::chrono_tz::Tz::Australia__Darwin,
            WindowsTimezone::EAustraliaStandardTime => ::chrono_tz::Tz::Australia__Brisbane,
            WindowsTimezone::AusEasternStandardTime => ::chrono_tz::Tz::Australia__Sydney,
            WindowsTimezone::WestPacificStandardTime => ::chrono_tz::Tz::Pacific__Port_Moresby,
            WindowsTimezone::TasmaniaStandardTime => ::chrono_tz::Tz::Australia__Hobart,
            WindowsTimezone::VladivostokStandardTime => ::chrono_tz::Tz::Asia__Vladivostok,
            WindowsTimezone::LordHoweStandardTime => ::chrono_tz::Tz::Australia__Lord_Howe,
            WindowsTimezone::BougainvilleStandardTime => ::chrono_tz::Tz::Pacific__Bougainville,
            WindowsTimezone::RussiaTimeZone10 => ::chrono_tz::Tz::Asia__Srednekolymsk,
            WindowsTimezone::MagadanStandardTime => ::chrono_tz::Tz::Asia__Magadan,
            WindowsTimezone::NorfolkStandardTime => ::chrono_tz::Tz::Pacific__Norfolk,
            WindowsTimezone::SakhalinStandardTime => ::chrono_tz::Tz::Asia__Sakhalin,
            WindowsTimezone::CentralPacificStandardTime => ::chrono_tz::Tz::Pacific__Guadalcanal,
            WindowsTimezone::RussiaTimeZone11 => ::chrono_tz::Tz::Asia__Kamchatka,
            WindowsTimezone::NewZealandStandardTime => ::chrono_tz::Tz::Pacific__Auckland,
            WindowsTimezone::UtcPlus12 => ::chrono_tz::Tz::Etc__GMTMinus12,
            WindowsTimezone::FijiStandardTime => ::chrono_tz::Tz::Pacific__Fiji,
            WindowsTimezone::ChathamIslandsStandardTime => ::chrono_tz::Tz::Pacific__Chatham,
            WindowsTimezone::UtcPlus13 => ::chrono_tz::Tz::Etc__GMTMinus13,
            WindowsTimezone::TongaStandardTime => ::chrono_tz::Tz::Pacific__Tongatapu,
            WindowsTimezone::SamoaStandardTime => ::chrono_tz::Tz::Pacific__Apia,
            WindowsTimezone::LineIslandsStandardTime => ::chrono_tz::Tz::Pacific__Kiritimati,
        }
    }
}
#[cfg(feature = "schemars")]
impl schemars::JsonSchema for WindowsTimezone {
    fn schema_name() -> String {
        "WindowsTimezone".to_string()
    }
    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::{InstanceType, Schema, SchemaObject};
        Schema::Object(SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            enum_values: Some(vec![
                "datelineStandardTime".into(),
                "utcMinus11".into(),
                "aleutianStandardTime".into(),
                "hawaiianStandardTime".into(),
                "marquesasStandardTime".into(),
                "alaskanStandardTime".into(),
                "utcMinus09".into(),
                "pacificStandardTimeMexico".into(),
                "utcMinus08".into(),
                "pacificStandardTime".into(),
                "usMountainStandardTime".into(),
                "mountainStandardTimeMexico".into(),
                "mountainStandardTime".into(),
                "yukonStandardTime".into(),
                "centralAmericaStandardTime".into(),
                "centralStandardTime".into(),
                "easterIslandStandardTime".into(),
                "centralStandardTimeMexico".into(),
                "canadaCentralStandardTime".into(),
                "saPacificStandardTime".into(),
                "easternStandardTimeMexico".into(),
                "easternStandardTime".into(),
                "haitiStandardTime".into(),
                "cubaStandardTime".into(),
                "usEasternStandardTime".into(),
                "turksAndCaicosStandardTime".into(),
                "paraguayStandardTime".into(),
                "atlanticStandardTime".into(),
                "venezuelaStandardTime".into(),
                "centralBrazilianStandardTime".into(),
                "saWesternStandardTime".into(),
                "pacificSaStandardTime".into(),
                "newfoundlandStandardTime".into(),
                "tocantinsStandardTime".into(),
                "eSouthAmericaStandardTime".into(),
                "saEasternStandardTime".into(),
                "argentinaStandardTime".into(),
                "greenlandStandardTime".into(),
                "montevideoStandardTime".into(),
                "magallanesStandardTime".into(),
                "saintPierreStandardTime".into(),
                "bahiaStandardTime".into(),
                "utcMinus02".into(),
                "azoresStandardTime".into(),
                "capeVerdeStandardTime".into(),
                "utc".into(),
                "gmtStandardTime".into(),
                "greenwichStandardTime".into(),
                "saoTomeStandardTime".into(),
                "moroccoStandardTime".into(),
                "wEuropeStandardTime".into(),
                "centralEuropeStandardTime".into(),
                "romanceStandardTime".into(),
                "centralEuropeanStandardTime".into(),
                "wCentralAfricaStandardTime".into(),
                "jordanStandardTime".into(),
                "gtbStandardTime".into(),
                "middleEastStandardTime".into(),
                "egyptStandardTime".into(),
                "eEuropeStandardTime".into(),
                "syriaStandardTime".into(),
                "westBankStandardTime".into(),
                "southAfricaStandardTime".into(),
                "fleStandardTime".into(),
                "israelStandardTime".into(),
                "southSudanStandardTime".into(),
                "kaliningradStandardTime".into(),
                "sudanStandardTime".into(),
                "libyaStandardTime".into(),
                "namibiaStandardTime".into(),
                "arabicStandardTime".into(),
                "turkeyStandardTime".into(),
                "arabStandardTime".into(),
                "belarusStandardTime".into(),
                "russianStandardTime".into(),
                "eAfricaStandardTime".into(),
                "iranStandardTime".into(),
                "arabianStandardTime".into(),
                "astrakhanStandardTime".into(),
                "azerbaijanStandardTime".into(),
                "russiaTimeZone3".into(),
                "mauritiusStandardTime".into(),
                "saratovStandardTime".into(),
                "georgianStandardTime".into(),
                "volgogradStandardTime".into(),
                "caucasusStandardTime".into(),
                "afghanistanStandardTime".into(),
                "westAsiaStandardTime".into(),
                "ekaterinburgStandardTime".into(),
                "pakistanStandardTime".into(),
                "qyzylordaStandardTime".into(),
                "indiaStandardTime".into(),
                "sriLankaStandardTime".into(),
                "nepalStandardTime".into(),
                "centralAsiaStandardTime".into(),
                "bangladeshStandardTime".into(),
                "omskStandardTime".into(),
                "myanmarStandardTime".into(),
                "seAsiaStandardTime".into(),
                "altaiStandardTime".into(),
                "wMongoliaStandardTime".into(),
                "northAsiaStandardTime".into(),
                "nCentralAsiaStandardTime".into(),
                "tomskStandardTime".into(),
                "chinaStandardTime".into(),
                "northAsiaEastStandardTime".into(),
                "singaporeStandardTime".into(),
                "wAustraliaStandardTime".into(),
                "taipeiStandardTime".into(),
                "ulaanbaatarStandardTime".into(),
                "ausCentralWStandardTime".into(),
                "transbaikalStandardTime".into(),
                "tokyoStandardTime".into(),
                "northKoreaStandardTime".into(),
                "koreaStandardTime".into(),
                "yakutskStandardTime".into(),
                "cenAustraliaStandardTime".into(),
                "ausCentralStandardTime".into(),
                "eAustraliaStandardTime".into(),
                "ausEasternStandardTime".into(),
                "westPacificStandardTime".into(),
                "tasmaniaStandardTime".into(),
                "vladivostokStandardTime".into(),
                "lordHoweStandardTime".into(),
                "bougainvilleStandardTime".into(),
                "russiaTimeZone10".into(),
                "magadanStandardTime".into(),
                "norfolkStandardTime".into(),
                "sakhalinStandardTime".into(),
                "centralPacificStandardTime".into(),
                "russiaTimeZone11".into(),
                "newZealandStandardTime".into(),
                "utcPlus12".into(),
                "fijiStandardTime".into(),
                "chathamIslandsStandardTime".into(),
                "utcPlus13".into(),
                "tongaStandardTime".into(),
                "samoaStandardTime".into(),
                "lineIslandsStandardTime".into(),
            ]),
            ..Default::default()
        })
    }
    fn is_referenceable() -> bool {
        true
    }
}

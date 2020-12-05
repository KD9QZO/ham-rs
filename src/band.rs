#[derive(Debug, Serialize, Deserialize,PartialEq,Clone)]
pub enum Band {
    OneHundredSixtyMeters(i32),
    EightyMeters(i32),
    FortyMeters(i32),
    ThirtyMeters(i32),
    TwentyMeters(i32),
    SeventeenMeters(i32),
    FifteenMeters(i32),
    TwelveMeters(i32),
    TenMeters(i32),
    SixMeters(i32),
    TwoMeters(i32),
    OnePointTwoFiveMeters(i32),
    SeventyCM(i32),
    ThirtyThreeCM(i32),
    Unknown(i32)
}
pub const AMATEUR_BANDS: [Band; 14] =
    [Band::OneHundredSixtyMeters(1800),
     Band::EightyMeters(3500000),
     Band::FortyMeters(7000000),
     Band::ThirtyMeters(10100000),
     Band::TwentyMeters(18068000),
     Band::SeventeenMeters(18068000),
     Band::FifteenMeters(21000000),
     Band::TwelveMeters(24890000),
     Band::TenMeters(28000000),
     Band::SixMeters(50000000),
     Band::TwoMeters(144000000),
     Band::OnePointTwoFiveMeters(222000000),
     Band::SeventyCM(420000000),
     Band::ThirtyThreeCM(902000000)];

impl Band {
    pub fn new(freq:i32) -> Band {
        match freq {
            freq if freq >= 1800 && freq <= 2000 => Band::OneHundredSixtyMeters(freq),
            freq if freq >= 3500000 && freq <= 4000000 => Band::EightyMeters(freq),
            freq if freq >= 7000000 && freq <= 7300000 => Band::FortyMeters(freq),
            freq if freq >= 10100000 && freq <= 10150000 => Band::ThirtyMeters(freq),
            freq if freq >= 14000000 && freq <= 14350000 => Band::TwentyMeters(freq),
            freq if freq >= 18068000 && freq <= 18168000 => Band::SeventeenMeters(freq),
            freq if freq >= 21000000 && freq <= 21450000 => Band::FifteenMeters(freq),
            freq if freq >= 24890000 && freq <= 24990000 => Band::TwelveMeters(freq),
            freq if freq >= 28000000 && freq <= 29700000 => Band::TenMeters(freq),
            freq if freq >= 50000000 && freq <= 54000000 => Band::SixMeters(freq),
            freq if freq >= 144000000 && freq <= 148000000 => Band::TwoMeters(freq),
            freq if freq >= 222000000 && freq <= 225000000 => Band::OnePointTwoFiveMeters(freq),
            freq if freq >= 420000000 && freq <= 450000000 => Band::SeventyCM(freq),
            freq if freq >= 902000000 && freq <= 928000000 => Band::ThirtyThreeCM(freq),
            _ => Band::Unknown(freq)
        }
    }
    pub fn mhz(&self) -> f32 {
        self.frequency() as f32 / 1000000.0
    }
    pub fn frequency(&self) -> i32 {
        match self {
            Band::OneHundredSixtyMeters(f) |
            Band::EightyMeters(f) |
            Band::FortyMeters(f) | 
            Band::ThirtyMeters(f) |
            Band::TwentyMeters(f) |
            Band::SeventeenMeters(f) |
            Band::FifteenMeters(f) |
            Band::TwelveMeters(f) |
            Band::TenMeters(f) |
            Band::SixMeters(f) |
            Band::TwoMeters(f) |
            Band::OnePointTwoFiveMeters(f) |
            Band::SeventyCM(f) |
            Band::ThirtyThreeCM(f) |
            Band::Unknown(f) => *f
        }
    }
    pub fn band(&self) -> Option<&'static str> {
        match self {
            Band::OneHundredSixtyMeters(_) => Some("160m"),
            Band::EightyMeters(_) => Some("80m"),
            Band::FortyMeters(_) => Some("40m"),
            Band::ThirtyMeters(_) => Some("30m"),
            Band::TwentyMeters(_) => Some("20m"),
            Band::SeventeenMeters(_) => Some("17m"),
            Band::FifteenMeters(_) => Some("15m"),
            Band::TwelveMeters(_) => Some("12m"),
            Band::TenMeters(_) => Some("10m"),
            Band::SixMeters(_) => Some("6m"),
            Band::TwoMeters(_) => Some("2m"),
            Band::OnePointTwoFiveMeters(_) => Some("1.25m"),
            Band::SeventyCM(_) => Some("70cm"),
            Band::ThirtyThreeCM(_) => Some("33cm"),
            Band::Unknown(_) => None
        }
    }
    pub fn color(&self) -> Option<&'static str> {
        match self {
            Band::OneHundredSixtyMeters(_) => Some("#7cfc00"),
            Band::EightyMeters(_) => Some("#e550e5"),
            Band::FortyMeters(_) => Some("#5959ff"),
            Band::ThirtyMeters(_) => Some("#62d962"),
            Band::TwentyMeters(_) => Some("#f2c40c"),
            Band::SeventeenMeters(_) => Some("#f2f261"),
            Band::FifteenMeters(_) => Some("#cca166"),
            Band::TwelveMeters(_) => Some("#b22222"),
            Band::TenMeters(_) => Some("#ff69b4"),
            Band::SixMeters(_) => Some("#FF0000"),
            Band::TwoMeters(_) => Some("#FF1493"),
            Band::OnePointTwoFiveMeters(_) => Some("#CCFF00"),
            Band::SeventyCM(_) => Some("#999900"),
            Band::ThirtyThreeCM(_) => Some("#5AB8C7"),
            Band::Unknown(_) => None
        }
    }
}
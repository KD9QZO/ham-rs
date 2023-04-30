#[macro_use]
extern crate serde_derive;

pub mod adif;
pub mod band;
pub mod call;
pub mod countries;
pub mod fcc;
pub mod grid;
pub mod log;
pub mod lotw;

pub use band::Band;
pub use call::Call;
pub use countries::{Country, CountryInfo};
pub use grid::Grid;
pub use log::LogEntry;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Mode {
    DigiU,
    DigiL,
    USB,
    LSB,
    FT8,
    FT4,
    JT9,
    JT65,
    AM,
    FM,
    NFM,
    WSPR,
    PSK,
    Multipsk,
    Sig,
    Hell,
    CW,
    Other(String),
}

impl Mode {
    pub fn new<S: Into<String>>(mode: S) -> Mode {
        let mode = mode.into();
        let upper = mode.to_uppercase();

        match upper.as_str() {
            "FT8" => Mode::FT8,
            "DIGIU" => Mode::DigiU,
            "DIGIL" => Mode::DigiL,
            "USB" => Mode::USB,
            "LSB" => Mode::LSB,
            "FT4" => Mode::FT4,
            "JT9" => Mode::JT9,
            "JT65" => Mode::JT65,
            "AM" => Mode::AM,
            "FM" => Mode::FM,
            "NFM" => Mode::NFM,
            "WSPR" => Mode::WSPR,
            "PSK" => Mode::PSK,
            "MULTIPSK" => Mode::Multipsk,
            "SIG" => Mode::Sig,
            "HELL" => Mode::Hell,
            "CW" => Mode::CW,
            _ => Mode::Other(mode),
        }
    }
    pub fn mode(&self) -> String {
        let mode_string = match self {
            Mode::DigiU => "DigiU",
            Mode::DigiL => "DigiL",
            Mode::USB => "USB",
            Mode::LSB => "LSB",
            Mode::FT8 => "FT8",
            Mode::FT4 => "FT4",
            Mode::JT9 => "JT9",
            Mode::JT65 => "JT65",
            Mode::AM => "AM",
            Mode::FM => "FM",
            Mode::NFM => "NFM",
            Mode::WSPR => "WSPR",
            Mode::PSK => "PSK",
            Mode::Multipsk => "Multipsk",
            Mode::Sig => "Sig",
            Mode::Hell => "Hell",
            Mode::CW => "CW",
            Mode::Other(other) => other,
        };
        mode_string.to_string()
    }
}

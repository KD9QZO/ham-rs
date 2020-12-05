use crate::Call;
use crate::adif::CallsignInfo;
use crate::{Band,Mode};
use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub id: uuid::Uuid,
    pub from_id: Call,
    pub date: DateTime<Utc>,
    pub call: Call,
    pub frequency: Band,
    pub mode: Mode,
    pub comment: Option<String>,
    pub rst_sent: Option<String>,
    pub rst_received: Option<String>,
    pub lotw_qsl_sent: bool,
    pub lotw_qsl_rcvd: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ImportError {
    MissingCall,
    MissingMyCall,
    MissingFrequency,
    MissingMode,
    MissingDateTime
}

impl LogEntry {
    pub fn from_adif_record(record: &crate::adif::AdifRecord) -> Result<LogEntry,ImportError> {
        match (Call::call_from_adif_record(record), Call::my_call_from_adif_record(record), qso_freq(record), qso_date(record), record.adir_field_values.get("mode"), record.adir_field_values.get("comment"), record.adir_field_values.get("lotw_qsl_sent"), record.adir_field_values.get("lotw_qsl_rcvd")) {
            (Some(call), Some(my_call), Some(freq), Some(date), Some(mode), comment, lotw_sent, lotw_rcvd) => {
                let comment =
                    match comment {
                        Some(comment) => Some(comment.to_string()),
                        None => None,
                    };
                let lotw_sent = 
                    match lotw_sent {
                        Some(x) if x == "Y" => true,
                        _ => false
                    };
                let lotw_rcvd = 
                    match lotw_rcvd {
                        Some(x) if x == "Y" => true,
                        _ => false
                    };

                Ok(LogEntry {
                    id: uuid::Uuid::new_v4(),
                    from_id: my_call,
                    date: date,
                    call: call,
                    frequency: freq,
                    mode: Mode::new(mode),
                    comment: comment,
                    rst_sent: None,
                    rst_received: None,
                    lotw_qsl_sent: lotw_sent,
                    lotw_qsl_rcvd: lotw_rcvd
                })
            },
            (None, _, _, _, _, _, _, _) => Err(ImportError::MissingCall),
            (_, None, _, _, _, _, _, _) => Err(ImportError::MissingMyCall),
            (_, _, None, _, _, _, _, _) => Err(ImportError::MissingFrequency),
            (_, _, _, None, _, _, _, _) => Err(ImportError::MissingDateTime),
            (_, _, _, _, None, _, _, _) => Err(ImportError::MissingMode)
        }
    }
}

fn qso_date(record: &crate::adif::AdifRecord) -> Option<DateTime<Utc>> {
    match (record.adir_field_values.get("qso_date"), record.adir_field_values.get("time_on")) {
        (Some(date), Some(time)) if date.len() == 8 && time.len() == 4 => {
            let year = &date[0..4];
            let month = &date[4..6];
            let day = &date[6..8];
            let hour = &time[0..2];
            let min = &time[2..4];
            match (year.parse(), month.parse(), day.parse(), hour.parse(), min.parse()) {
                (Ok(year), Ok(month), Ok(day), Ok(hour), Ok(min)) => {
                    let date = Utc.ymd(year, month, day).and_hms(hour, min, 0);
                    Some(date)
                },
                _ => {
                    None
                }
            }
        },
        _ => None
    }
}

fn qso_freq(record: &crate::adif::AdifRecord) -> Option<Band> {
    match record.adir_field_values.get("freq") {
        Some(freq) => {
            match freq.parse::<f32>() {
                Ok(freq) => {
                    let freq: i32 = (freq * 1000000.0) as i32;
                    Some(Band::new(freq))
                },
                _ => None,
            }
        },
        None => None,
    }
}
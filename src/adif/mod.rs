use crate::lotw::LoTWStatus;
use crate::{Call, Grid};
//
// Amateur Data Interchange Format (ADIF) is a standardized file format used for
// exchanging data about amateur radio contacts ("QSOs").  This crate seeks to
// implement an ADIF importer and a small reporting program.
//
// As of this writing, the latest ADIF standard is v3.0.8, available here:
//
//   http://www.adif.org/308/ADIF_308.htm
//
// Note that much of ADIF describes a logical form for the data.  There are
// currently two physical file formats: ADI (a somewhat baroque format described
// originally in version 1, which dates back to 1996) and ADX (a more modern
// XML-based format).  ADI appears to be more widely used, while ADX is marked
// optional in the standard.  For that reason, this crate currently only seeks
// to implement ADI.
//
// Section II.A ("Upward Compatibility") guarantees that "an ADIF file compliant
// with ADIF version N will comply with any future ADIF version M where M>N."
// By implementing v3, we support all v1 and v2 files.
//

use std::fmt;
use std::io;

mod adi;
mod adif;
mod adifutil;

//
// TODO decide whether there's a cleaner way to structure this.
//
pub use crate::adif::adif::adif_dump;
pub use crate::adif::adif::AdifDumpWhichRecords;
pub use crate::adif::adif::AdifRecord;

//
// AdifParseError is used to represent any sort of operational error we may
// encounter during parsing.
//

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum AdifParseError {
    ADIF_EIO(io::Error),               // error from underlying I/O
    ADIF_EBADINPUT(String),            // invalid input
    ADIF_ENOT_YET_IMPLEMENTED(String), // feature that's not yet implemented
}

impl From<io::Error> for AdifParseError {
    fn from(error: io::Error) -> Self {
        AdifParseError::ADIF_EIO(error)
    }
}

impl fmt::Display for AdifParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AdifParseError::ADIF_EIO(ioerror) => {
                write!(f, "{}", ioerror.to_string())
            }
            AdifParseError::ADIF_EBADINPUT(message) => {
                write!(f, "input error: {}", message)
            }
            AdifParseError::ADIF_ENOT_YET_IMPLEMENTED(message) => {
                write!(f, "not yet implemented: {}", message)
            }
        }
    }
}

pub fn adif_parse(label: &str, source: &mut io::Read) -> Result<adif::AdifFile, AdifParseError> {
    let adi = adi::adi_parse(source)?;
    adif::adif_parse_adi(label, &adi)
}

pub trait CallsignInfo {
    fn my_call_from_adif_record(record: &adif::AdifRecord) -> Option<Call>;
    fn call_from_adif_record(record: &adif::AdifRecord) -> Option<Call>;
}

impl CallsignInfo for Call {
    fn my_call_from_adif_record(record: &crate::adif::AdifRecord) -> Option<Call> {
        match record.adir_field_values.get("station_callsign") {
            None => None,
            Some(call) => {
                let name = match record.adir_field_values.get("my_name") {
                    Some(name) => Some(name.to_string()),
                    None => None,
                };
                let state = match record.adir_field_values.get("my_state") {
                    Some(state) => Some(state.to_string()),
                    None => None,
                };
                let grid = match record.adir_field_values.get("my_gridsquare") {
                    Some(grid) => Some(Grid::new(grid.to_string()).unwrap()),
                    None => None,
                };
                let qth = match record.adir_field_values.get("my_city") {
                    Some(city) => Some(city.to_string()),
                    None => None,
                };
                let lotw_confirmed = match record.adir_field_values.get("lotw_qsl_sent") {
                    Some(qsl_sent) if qsl_sent == "Y" => LoTWStatus::Registered,
                    _ => LoTWStatus::Unknown,
                };
                Some(Call::full(
                    call.to_uppercase(),
                    name,
                    None,
                    qth,
                    state,
                    grid,
                    lotw_confirmed,
                ))
            }
        }
    }

    fn call_from_adif_record(record: &crate::adif::AdifRecord) -> Option<Call> {
        match record.adir_field_values.get("call") {
            None => None,
            Some(call) => {
                let name = match record.adir_field_values.get("name") {
                    Some(name) => Some(name.to_string()),
                    None => None,
                };
                let state = match record.adir_field_values.get("state") {
                    Some(state) => Some(state.to_string()),
                    None => None,
                };
                let grid = match record.adir_field_values.get("gridsquare") {
                    Some(grid) => Some(Grid::new(grid.to_string()).unwrap()),
                    None => None,
                };
                let qth = match record.adir_field_values.get("qth") {
                    Some(city) => Some(city.to_string()),
                    None => None,
                };
                let lotw_confirmed = match record.adir_field_values.get("lotw_qsl_rcvd") {
                    Some(qsl_rcvd) if qsl_rcvd == "Y" => LoTWStatus::Registered,
                    _ => LoTWStatus::Unknown,
                };
                Some(Call::full(
                    call.to_uppercase(),
                    name,
                    None,
                    qth,
                    state,
                    grid,
                    lotw_confirmed,
                ))
            }
        }
    }
}

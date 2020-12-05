use serde::{Deserialize};
use std::fmt;
use chrono::prelude::*;
use crate::Grid;
use crate::lotw::LoTWStatus;

#[derive(Debug,Deserialize,Serialize,Clone,PartialEq)]
pub struct Call {
    call: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    op: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    qth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grid: Option<Grid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lotw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_lotw_upload: Option<DateTime<Utc>>
}

impl Call {
    pub fn new<S: Into<String>>(call: S) -> Call {
        Call { 
            call: call.into().to_uppercase(),
            op: None,
            address: None,
            qth: None,
            state: None,
            grid: None,
            lotw: None,
            last_lotw_upload: None,
        }
    }

    pub fn full(call: String, op: Option<String>, address: Option<String>, qth: Option<String>, state: Option<String>, grid: Option<Grid>, lotw: LoTWStatus) -> Call {
        let (lotw, last_lotw_upload) =
            match lotw {
                LoTWStatus::Registered => (Some(true), None),
                LoTWStatus::Unregistered => (Some(false),None),
                LoTWStatus::LastUpload(last) => (Some(true),Some(last)),
                LoTWStatus::Unknown => (None, None)
            };
        Call {
            call: call.to_uppercase(),
            op: op,
            address: address,
            qth: qth,
            state: state,
            grid: grid,
            lotw: lotw,
            last_lotw_upload: last_lotw_upload,
        }
    }

    pub fn prefix(&self) -> Option<String> {
        let index = self.call.to_string().chars().position(|c| c.is_digit(10) );
        match index {
            Some(index) => {
                Some(self.call.to_string()[..index+1].to_string())
            },
            None => None,
        }
    }

    pub fn call(&self) -> String {
        self.call.to_string()
    }

    pub fn op(&self) -> Option<String> {
        self.op.clone()
    }

    pub fn set_op(&mut self, op: Option<String>) {
        self.op = op;
    }

    pub fn qth(&self) -> Option<String> {
        self.qth.clone()
    }

    pub fn set_qth(&mut self, qth: Option<String>) {
        self.qth = qth;
    }

    pub fn state(&self) -> Option<String> {
        self.state.clone()
    }

    pub fn set_state(&mut self, state: Option<String>) {
        self.state = state;
    }

    pub fn grid(&self) -> &Option<Grid> {
        &self.grid
    }

    pub fn set_grid(&mut self, grid: Option<Grid>) {
        self.grid = grid;
    }

    pub fn address(&self) -> Option<String> {
        self.address.clone()
    }

    pub fn set_address(&mut self, address: Option<String>) {
        self.address = address;
    }

    pub fn lotw(&self) -> LoTWStatus {
        match (self.lotw, self.last_lotw_upload) {
            (Some(true), Some(last)) => LoTWStatus::LastUpload(last),
            (Some(true), None) => LoTWStatus::Registered,
            (Some(false), None) => LoTWStatus::Unregistered,
            _ => LoTWStatus::Unknown
        }
    }

    pub fn set_lotw(&mut self, lotw: LoTWStatus) {
        let (lotw, last_lotw_upload) =
            match lotw {
                LoTWStatus::Registered => (Some(true), None),
                LoTWStatus::Unregistered => (Some(false),None),
                LoTWStatus::LastUpload(last) => (Some(true),Some(last)),
                LoTWStatus::Unknown => (None, None)
            };
        self.lotw = lotw;
        self.last_lotw_upload = last_lotw_upload;
    }
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.call())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix() {
        let call = Call::new("KK4WJS");
        assert_eq!(call.prefix(), Some("KK4".to_string()));
    }
}
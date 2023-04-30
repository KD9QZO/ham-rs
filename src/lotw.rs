use crate::Call;
use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LoTWStatus {
    Registered,
    Unregistered,
    LastUpload(DateTime<Utc>),
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CertificateMeta {
    Callsign(String),
    Name(String),
    Email(String),
    UnknownKeyValue((String, String)),
    Unknown(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LotwCertificate {
    call: Call,
    name: Option<String>,
    email: Option<String>,
}

impl LotwCertificate {
    pub fn from_call(call: &Call) -> LotwCertificate {
        LotwCertificate {
            call: call.clone(),
            name: None,
            email: None,
        }
    }
    pub fn with_name(&self, name: String) -> LotwCertificate {
        LotwCertificate {
            call: self.call.clone(),
            name: Some(name),
            email: self.email.clone(),
        }
    }
    pub fn with_email(&self, email: String) -> LotwCertificate {
        LotwCertificate {
            call: self.call.clone(),
            name: self.name.clone(),
            email: Some(email),
        }
    }
    pub fn call(&self) -> Call {
        self.call.clone()
    }
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }
    pub fn email(&self) -> Option<String> {
        self.email.clone()
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Identity {
    Unknown,
    ClientCertificate(LotwCertificate),
}

impl Identity {
    pub fn call(&self) -> Option<Call> {
        match self {
            Identity::Unknown => None,
            Identity::ClientCertificate(cert) => Some(cert.call().clone()),
        }
    }
}

use crate::Call;
use crate::lotw::LoTWStatus;

#[derive(Debug, Serialize, Deserialize,Clone,PartialEq)]
pub enum OperatorClass {
    Tech,
    General,
    Extra,
    Novice,
    TechPlus,
    Advanced
}

#[derive(Debug, Serialize, Deserialize,Clone,PartialEq)]
pub struct FccData {
    pub class: Option<OperatorClass>,
    pub previous_class: Option<OperatorClass>,
    pub previous_call: Option<String>,
    pub status: Option<char>,
    pub radio_service_code: Option<String>,
    pub grant_date: Option<String>,
    pub expiration_date: Option<String>,
    pub cancellation_date: Option<String>
}

impl FccData {
    pub fn default() -> FccData {
        FccData {
            class: None,
            previous_class: None,
            previous_call: None,
            status: None,
            radio_service_code: None,
            grant_date: None,
            expiration_date: None,
            cancellation_date: None
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Entity {
    pub RecordType: String,
    pub USI: u64,
    pub ULS: Option<String>,
    pub EFB: Option<String>,
    pub CallSign: String,
    pub EntityType: String,
    pub LicenceID: String,
    pub FullName: String,
    pub FirstName: String,
    pub MiddleInitial: String,
    pub LastName: String,
    pub Suffix: Option<String>,
    pub Phone: Option<String>,
    pub Fax: Option<String>,
    pub Email: Option<String>,
    pub Address: String,
    pub City: String,
    pub State: String,
    pub ZipCode: String,
    pub PoBox: Option<String>,
    pub FRN: String,
}

#[derive(Debug, Deserialize)]
pub struct Amateur {
    pub RecordType: String,
    pub USI: u64,
    pub ULS: Option<String>,
    pub EFB: Option<String>,
    pub CallSign: String,
    pub OperatorClass: Option<char>,
    pub GroupCode: String,
    pub RegionCode: String,
    pub TrusteeCallSign: Option<String>,
    pub TrusteeIndicator: Option<String>,
    pub PhysicianCertification: Option<String>,
    pub VESignature: Option<String>,
    pub SystematicCallChange: Option<String>,
    pub VanityCallSignCange: Option<String>,
    pub VanityRelationship: Option<String>,
    pub PreviousCallSign: Option<String>,
    pub PreviousOperatorClass: Option<char>,
    pub TrusteeName: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Application {
    pub RecordType: String,
    pub USI: u64,
    pub ULS: Option<String>,
    pub EFB: Option<String>,
    pub CallSign: String,
    pub Status: char,
    pub RadioServiceCode: String,
    pub GrantDate: String,
    pub ExpiredDate: String,
    pub CancellationDate: Option<String>,
}

pub trait FccInfo {
    fn from_entity(entity: Entity) -> Call;
    fn with_entity(&self, entity: Entity) -> Call;
}

impl FccInfo for Call {
    fn from_entity(entity: Entity) -> Call {
        Call::full(entity.CallSign.to_uppercase(), 
                   Some(entity.FullName),
                   Some(entity.Address),
                   Some(entity.City),
                   Some(entity.State),
                   None,
                   LoTWStatus::Unknown)
    }

    fn with_entity(&self, entity: Entity) -> Call {
        Call::full(self.call(), 
                   Some(entity.FullName),
                   Some(entity.Address),
                   Some(entity.City),
                   Some(entity.State),
                   self.grid().clone(),
                   self.lotw())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_entity() {
        //let call = Call::from_entity(entity);
        //assert_eq!(grid, Err(GridError::InvalidLength(5)));
    }
}
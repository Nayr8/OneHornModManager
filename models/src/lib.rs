use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use serde::{Deserialize, Serialize};

mod logging;
mod errors;
mod fs;

pub use logging::*;
pub use errors::*;
pub use fs::*;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Profiles {
    pub current_profile: usize,
    pub profiles: HashMap<usize, String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Mod {
    pub name: String,
    pub description: String,
    pub version: String,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Status<OK: PartialEq, ERR: PartialEq = ()> {
    Loading,
    Loaded(OK),
    Error(ERR)
}

impl<OK: PartialEq, ERR: PartialEq> Status<OK, ERR> {
    pub const fn as_ref(&self) -> Status<&OK, &ERR> {
        match self {
            Status::Loading => Status::Loading,
            Status::Loaded(ok) => Status::Loaded(ok),
            Status::Error(err) => Status::Error(err),
        }
    }
}

// This is needed because of the awkward way results are handled in the translation from tauri -> js -> yew
#[derive(Serialize, Deserialize)]
pub enum MMResult<OK, ERR> {
    Ok(OK),
    Err(ERR)
}

impl<OK: Debug, ERR: Debug> Debug for MMResult<OK, ERR> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MMResult::Ok(ok) => write!(f, "Ok({ok:?})"),
            MMResult::Err(err) => write!(f, "Err({err:?})"),
        }
    }
}

impl<OK, ERR> From<Result<OK, ERR>> for MMResult<OK, ERR> {
    fn from(value: Result<OK, ERR>) -> Self {
        match value {
            Ok(ok) => MMResult::Ok(ok),
            Err(err) => MMResult::Err(err),
        }
    }
}
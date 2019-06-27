use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::types::PasteClient;

pub mod bpaste;
pub mod dpaste;
pub mod dpaste_com;
pub mod fiche;
pub mod haste;
pub mod ix;
pub mod modern_paste;
pub mod onetimesecret;
pub mod paste_rs;
pub mod pipfi;
pub mod sprunge;
pub mod ubuntu;
pub mod vpaste;

lazy_static! {
    pub static ref BACKENDS_INFO: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert(bpaste::NAME, bpaste::INFO);
        m.insert(dpaste::NAME, dpaste::INFO);
        m.insert(dpaste_com::NAME, dpaste_com::INFO);
        m.insert(fiche::NAME, fiche::INFO);
        m.insert(haste::NAME, haste::INFO);
        m.insert(ix::NAME, ix::INFO);
        m.insert(modern_paste::NAME, modern_paste::INFO);
        m.insert(onetimesecret::NAME, onetimesecret::INFO);
        m.insert(paste_rs::NAME, paste_rs::INFO);
        m.insert(pipfi::NAME, pipfi::INFO);
        m.insert(sprunge::NAME, sprunge::INFO);
        m.insert(ubuntu::NAME, ubuntu::INFO);
        m.insert(vpaste::NAME, vpaste::INFO);
        m
    };
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "backend")]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum BackendConfig {
    Bpaste(bpaste::Backend),
    Dpaste(dpaste::Backend),
    DpasteCom(dpaste_com::Backend),
    Fiche(fiche::Backend),
    Haste(haste::Backend),
    Ix(ix::Backend),
    ModernPaste(modern_paste::Backend),
    Onetimesecret(onetimesecret::Backend),
    PasteRs(paste_rs::Backend),
    Pipfi(pipfi::Backend),
    Sprunge(sprunge::Backend),
    Ubuntu(ubuntu::Backend),
    Vpaste(vpaste::Backend),
}

impl BackendConfig {
    pub fn extract_backend(self) -> Box<dyn PasteClient> {
        match self {
            BackendConfig::Bpaste(backend) => Box::new(backend),
            BackendConfig::Dpaste(backend) => Box::new(backend),
            BackendConfig::DpasteCom(backend) => Box::new(backend),
            BackendConfig::Fiche(backend) => Box::new(backend),
            BackendConfig::Haste(backend) => Box::new(backend),
            BackendConfig::Ix(backend) => Box::new(backend),
            BackendConfig::ModernPaste(backend) => Box::new(backend),
            BackendConfig::Onetimesecret(backend) => Box::new(backend),
            BackendConfig::PasteRs(backend) => Box::new(backend),
            BackendConfig::Pipfi(backend) => Box::new(backend),
            BackendConfig::Sprunge(backend) => Box::new(backend),
            BackendConfig::Ubuntu(backend) => Box::new(backend),
            BackendConfig::Vpaste(backend) => Box::new(backend),
        }
    }
}

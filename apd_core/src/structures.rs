use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PackagesData {
    pub request_args: HashMap<String, String>,
    pub length: u32,
    pub packages: Vec<Package>,
}

#[derive(Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub epoch: u32,
    pub version: String,
    pub release: String,
    pub arch: String,
    pub disttag: String,
    pub buildtime: u32,
    pub source: String,
}

#[derive(Debug, PartialEq)]
pub struct PackagesDifference {
    pub only_in_first: HashSet<String>,
    pub only_in_second: HashSet<String>,
    pub intersection: HashSet<String>,
}

impl PackagesDifference {
    pub fn new() -> PackagesDifference {
        PackagesDifference {
            only_in_first: HashSet::new(),
            only_in_second: HashSet::new(),
            intersection: HashSet::new(),
        }
    }
}

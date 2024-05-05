use std::collections::HashMap;

use reqwest::blocking::Client;

use crate::structures::PackagesData;

fn get_branch_packages_data(repo: &str) -> Option<String> {
    let api_url = "https://rdb.altlinux.org/api/export/branch_binary_packages/";

    let full_url = format!("{api_url}{repo}");

    let client = Client::new();
    let response = match client.get(full_url).send() {
        Ok(response) => response,
        _ => {
            println!("[-] Request error!");
            return None;
        }
    };

    if response.status() == 200 {
        match response.text() {
            Ok(response_text) => Some(response_text),
            _ => {
                println!("[-] Response data unpacking error!");
                return None;
            }
        }
    } else {
        println!("[-] Request error!");
        println!("[-] Error code: {}", response.status());
        return None;
    }
}

#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn get_branch_packages(repo: &str) -> Option<HashMap<String, Vec<String>>> {
    let response_data = match get_branch_packages_data(&repo) {
        Some(response_data) => response_data,
        None => return None,
    };

    let packages_data: PackagesData = match serde_json::from_str(&response_data) {
        Ok(packages_data) => packages_data,
        _ => {
            println!("[-] Error response json reading!");
            return None;
        }
    };

    let mut packages: HashMap<String, Vec<String>> = HashMap::new();
    for package_data in &packages_data.packages {
        let arch = package_data.arch.clone();
        if !packages.contains_key(&arch) {
            packages.insert(String::from(&arch), Vec::new());
        }
        packages
            .get_mut(&arch)
            .unwrap()
            .push(package_data.name.clone());
    }

    Some(packages)
}

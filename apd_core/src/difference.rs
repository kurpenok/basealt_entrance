use std::collections::HashSet;

use crate::structures::PackagesDifference;

#[allow(dead_code)]
pub fn get_packages_difference(
    packages_1: Vec<String>,
    packages_2: Vec<String>,
) -> PackagesDifference {
    let packages_1: HashSet<String> = HashSet::from_iter(packages_1);
    let packages_2: HashSet<String> = HashSet::from_iter(packages_2);

    let mut difference = PackagesDifference::new();

    for package in packages_1.difference(&packages_2) {
        difference.only_in_first.insert(package.to_owned());
    }

    for package in packages_2.difference(&packages_1) {
        difference.only_in_second.insert(package.to_owned());
    }

    for package in packages_1.intersection(&packages_2) {
        difference.intersection.insert(package.to_owned());
    }

    difference
}

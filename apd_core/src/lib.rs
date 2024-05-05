mod difference;
mod parser;
mod structures;

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_branch_packages() {
        crate::parser::get_branch_packages("p9");
    }

    #[test]
    fn test_get_packages_difference() {
        let packages_1 = vec![
            "sway".to_owned(),
            "swaybg".to_owned(),
            "swaylock".to_owned(),
        ];
        let packages_2 = vec!["sway".to_owned(), "waybar".to_owned(), "wofi".to_owned()];

        let mut expected_difference = crate::structures::PackagesDifference::new();

        expected_difference
            .only_in_first
            .insert("swaybg".to_owned());
        expected_difference
            .only_in_first
            .insert("swaylock".to_owned());

        expected_difference
            .only_in_second
            .insert("waybar".to_owned());
        expected_difference.only_in_second.insert("wofi".to_owned());

        expected_difference.intersection.insert("sway".to_owned());

        assert_eq!(
            expected_difference,
            crate::difference::get_packages_difference(packages_1, packages_2)
        );
    }
}

fn main() {
    unsafe {
        let lib = libloading::Library::new("libapd_core.so").unwrap();

        let get_branch_packages: libloading::Symbol<unsafe extern "C" fn(&str)> =
            lib.get(b"get_branch_packages\0").unwrap();

        get_branch_packages("p9");
    }
}

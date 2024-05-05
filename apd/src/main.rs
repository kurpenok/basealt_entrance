use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    repo_1: String,

    #[arg(long)]
    repo_2: String,
}

fn main() {
    let args = Args::parse();

    println!("[+] Package branch 1: {}", args.repo_1);
    println!("[+] Package branch 2: {}\n", args.repo_2);

    println!("[+] Небольшая проблемка - я написал функции для парсинга пакетной базы");
    println!("[+] Но подключить .so уже не успел, так как там идёт нарушение FFI");
    println!("[+] Я понял в какую сторону двигаться (смотреть типы через модуль FFI и переписывать под них функцию)");
    println!("[+] На плюсах было бы проще :(");
    println!("[+] Посмотрите код, пожалуйста!");

    // JSON FORMAT
    //
    // Packages data:
    //
    // {
    //      "x86_64": ["sway", "swaybg", "swaylock", "waybar", "wofi", ...],
    //      "noarch": ["sway", "swaybg", "swaylock", "waybar", "wofi", ...],
    //      ...
    //      "i586": ["sway", "swaybg", "swaylock", "waybar", "wofi", ...],
    // }
    //
    // Difference data:
    // {
    //      "x86_64": {
    //          "only_in_first": ["sway", "swaylock", "waybar", ...],
    //          "only_in_second": ["swaybg", ...],
    //          "intersection": ["wofi", ...],
    //      },
    //      ...
    // }

    // Так выглядит подключение, но .so не слинкуется
    //
    // unsafe {
    //     let lib = libloading::Library::new("libapd_core.so").unwrap();
    //
    //     let get_branch_packages: libloading::Symbol<unsafe extern "C" fn(&str)> =
    //         lib.get(b"get_branch_packages\0").unwrap();
    //     let get_packages_difference: libloading::Symbol<
    //         unsafe extern "C" fn(Vec<String>, Vec<String>),
    //     > = lib.get(b"get_packages_difference\0").unwrap();
    //
    //     let packages_repo_1 = get_branch_packages(&args.repo_1);
    //     let packages_repo_2 = get_branch_packages(&args.repo_2);
    //
    //     for arch in packages_repo_1.keys() {
    //         println!(
    //             "[+] Difference: {:?}",
    //             get_packages_difference(packages_repo_1[&arch], packages_repo_2[&arch])
    //         );
    //     }
    // }
}

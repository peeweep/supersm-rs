pub struct AppOptions {
    pub add: String,
    pub delete: String,
    pub target: String,
}

impl AppOptions {
    pub fn new() -> Self {
        let matches = clap::App::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .author("jinqiangzhang <peeweep@0x0.ee>")
            .about("A dotfiles manager https://github.com/peeweep/supersm-rs")
            .arg(
                clap::Arg::with_name("add")
                    .short("A")
                    .long("add")
                    .value_name("project folder")
                    .help("Add links"),
            )
            .arg(
                clap::Arg::with_name("delete")
                    .short("D")
                    .long("delete")
                    .value_name("project folder")
                    .conflicts_with("add")
                    .help("Remove links"),
            )
            .arg(
                clap::Arg::with_name("target")
                    .short("T")
                    .long("target")
                    .value_name("target folder")
                    .help("Set target"),
            )
            .get_matches();

        let add = matches.value_of("add").unwrap_or("").to_string();
        let delete = matches.value_of("delete").unwrap_or("").to_string();

        let currentdir = std::env::current_dir().unwrap();
        let parentdir = currentdir.parent().unwrap();
        let parentdir_str = parentdir.to_str().unwrap();
        let target = matches
            .value_of("target")
            .unwrap_or(&parentdir_str)
            .to_string();

        AppOptions {
            add,
            delete,
            target,
        }
    }
}

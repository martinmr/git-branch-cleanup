use clap::App;

pub struct CleanupOptions {
    pub dry_run: bool,
    pub force: bool,
}

pub fn parse_flags() -> Option<CleanupOptions> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let force = matches.is_present("force");
    let dry_run = matches.is_present("dry-run");

    Some(CleanupOptions {
        dry_run: dry_run,
        force: force,
    })
}

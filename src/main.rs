use std::env;
use gumdrop::Options;
use chrono::{DateTime, Local};

#[derive(Debug, Options)]
struct KitOptions {
    #[options(help = "print help message")]
    help: bool,

    #[options(short = 'd', help = "devel branch")]
    devel_branch: bool,

    #[options(help = "branch destination")]
    branch_destination: Option<String>,
}

fn main() {
    let opts = KitOptions::parse_args_default_or_exit();

    println!("{:#?}", opts);

    let branch_type = if opts.devel_branch {
        "dev"
    } else {
        "aci"
    };

    let user = match env::var("USER") {
        Ok(val) => val,
        Err(_e) => "lthevenoux".to_string(),
    };

    let now: DateTime<Local> = Local::now();
    let date = now.format("%Y%m%d%H%M");

    let branch_dest = match opts.branch_destination {
        None => "csw_coolidge".to_string(),
        Some(b) => b
    };

    println!("{}/{}.{}/{}", branch_type, user, date, branch_dest);
}

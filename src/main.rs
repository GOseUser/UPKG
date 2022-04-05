use std::env;
use upkg::*;
fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in args{
        if arg.starts_with("-") {
            if arg == "-h" || arg == "--help"{
            help();
            }
            if arg == "-v" || arg == "--version"{
                version();
            }
            if arg=="-u" || arg=="--update"{
                let srcs = get_sources();
                let app_names_and_urls=update_data();
            }
        }
    }
}

pub struct Database {
    pub app_names_and_urls: Vec<String>,
    pub app_names_and_versions: Vec<String>,
}

pub fn update_data() -> app_names_and_urls{
    let srcs = get_sources();
    for src in srcs.repos_names {
        let outfile: String = String::from(config_path.to_string()+&src+".Ud");
        for repo in srcs.repos_urls.clone() {
            if repo.contains(&src) {
                download_file(&repo, &outfile.clone(), true);
            }
        }
    }
    let data = upkg::get_updated_data_urls();
    return data;
}

fn help() {
    println!("Usage: upkg [options] <package>");
    println!("Options:");
    println!("-h, --help      Show this help message");
    println!("-v, --version   Show libupkg version");
    println!("-u, --update    Update database");
    println!("-i, --install   Install package (not implemented)");
    println!("-r, --remove    Remove package (not implemented)");
    println!("-l, --list      List packages (not implemented)");
    println!("-s, --search    Search for packages (not implemented)");
    println!("-g, --upgrade   Upgrade Packages (not implemented)");
    return;
}
fn version() {
    println!("upkg version {}", env!("CARGO_PKG_VERSION"));
    return;
}

use std::env;
use upkg;
fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in args{
        if arg == "-h" || arg == "--help"{
            help();
        }
        if arg == "-v" || arg == "--version"{
            version();
        }
    }
}
fn help() {
    println!("Usage: upkg [options] <package>");
    println!("Options:");
    println!("  -h, --help    Show this help message");
    println!("  -v, --version Show libupkg version");
    return;
}
fn version() {
    println!("upkg version {}", env!("CARGO_PKG_VERSION"));
    return;
}

use chrono::Local;
use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: std::path::PathBuf,

    #[arg(short, long)]
    bak: bool,

    #[arg(short, long)]
    original: bool,

    #[arg(short, long)]
    simple: bool,
}

fn main() {
    let args = Args::parse();

    // 存在チェック
    if !Path::new(&args.path).exists() {
        panic!("No such file or directory");
    }

    let backup_suffix = get_file_suffix(&args);

    let to_path = Path::new(&args.path);
    let to_file_name_string = to_path.file_name().unwrap().to_str().unwrap().to_string();
    let new_filename = to_file_name_string + "." + &backup_suffix;
    let to = to_path.with_file_name(new_filename);

    std::fs::copy(&args.path, to).expect("Could not copy file");
}

fn get_file_suffix(args: &Args) -> String {
    let dt = Local::now();
    if args.bak {
        String::from("bak")
    } else if args.original {
        String::from("org")
    } else if args.simple {
        dt.format("%Y%m%d").to_string()
    } else {
        dt.format("%Y%m%d%H%M%S").to_string()
    }
}

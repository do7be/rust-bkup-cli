use chrono::Local;
use clap::Parser;
use std::path::{Path, PathBuf};

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

    let to = get_to_path(&args);
    let to = match to {
        Some(v) => v,
        None => panic!("Could not copy file"),
    };
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

fn get_to_path(args: &Args) -> Option<PathBuf> {
    let to_path = Path::new(&args.path);
    let to_file_name = to_path.file_name()?.to_str()?.to_string();

    let backup_suffix = get_file_suffix(&args);

    let new_filename = to_file_name + "." + &backup_suffix;

    Some(to_path.with_file_name(new_filename))
}

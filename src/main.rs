use chrono::Local;
use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: std::path::PathBuf,

    /// Create filename.bak
    #[arg(short, long)]
    bak: bool,

    /// Create filename.org
    #[arg(short, long)]
    original: bool,

    /// Create filename.YYYYmmdd
    #[arg(short, long)]
    simple: bool,
}

fn main() {
    let args = Args::parse();

    let to = get_to_path(&args);
    let to = match to {
        Some(v) => v,
        None => panic!("Wrong file path"),
    };
    std::fs::copy(&args.path, &to).expect("Could not copy file");
    println!("Created: {}", to.display());
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

#[cfg(test)]
mod tests {
    use crate::{get_file_suffix, get_to_path, Args};
    use chrono::Local;
    use std::path::PathBuf;

    #[test]
    fn suffix_org_ok() {
        let args = Args {
            path: PathBuf::from("hoge.txt"),
            original: true,
            bak: false,
            simple: false,
        };
        let expected = "org";
        let actual = get_file_suffix(&args);
        assert_eq!(expected, actual);
    }

    #[test]
    fn suffix_bak_ok() {
        let args = Args {
            path: PathBuf::from("hoge.txt"),
            original: false,
            bak: true,
            simple: false,
        };
        let expected = "bak";
        let actual = get_file_suffix(&args);
        assert_eq!(expected, actual);
    }

    // now()のmockがまだ実装されていないため、simpleとdefaultのケースのテストは雑に書いておく。
    // https://github.com/chronotope/chrono/pull/580
    #[test]
    fn suffix_simple_ok() {
        let args = Args {
            path: PathBuf::from("hoge.txt"),
            original: false,
            bak: false,
            simple: true,
        };
        let dt = Local::now();
        let expected = dt.format("%Y%m%d").to_string();
        let actual = get_file_suffix(&args);
        assert_eq!(expected, actual);
    }

    #[test]
    fn suffix_default_ok() {
        let args = Args {
            path: PathBuf::from("hoge.txt"),
            original: false,
            bak: false,
            simple: false,
        };
        let dt = Local::now();
        // TODO: 状況・環境によっては落ちるかもしれない
        let expected = dt.format("%Y%m%d%H%M%S").to_string();
        let actual = get_file_suffix(&args);
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_path_org_ok() {
        let args = Args {
            path: PathBuf::from("hoge.txt"),
            original: true,
            bak: false,
            simple: false,
        };
        let expected = PathBuf::from("hoge.txt.org");
        let actual = get_to_path(&args);
        assert_eq!(expected, actual.expect("Could not get to path"));
    }
}

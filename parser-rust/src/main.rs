use std::fs;
use std::path::Path;
use regex::Regex;

#[derive(Debug)]
struct LogData {
    dir_name: String,
    pub_val: String,
    sub_val: String,
}

fn parse_log_file(file_path: &Path) -> String {
    let content = fs::read_to_string(file_path).unwrap();
    let re = Regex::new(r"(\d{1,3}(?:,\d{3})*|\d+)\s+msgs/sec").unwrap();
    if let Some(captures) = re.captures(&content) {
        if let Some(match_str) = captures.get(1) {
            return match_str.as_str().replace(",", "");
        }
    }
    "N/A".to_string()
}

fn get_sort_key(dir_name: &str) -> u64 {
    if let Some(val_str) = dir_name.strip_suffix("ms") {
        val_str.parse().unwrap()
    } else if let Some(val_str) = dir_name.strip_suffix("s") {
        val_str.parse::<u64>().unwrap() * 1000
    } else if let Some(val_str) = dir_name.strip_suffix("m") {
        val_str.parse::<u64>().unwrap() * 60 * 1000
    } else {
        u64::MAX
    }
}

fn main() {
    let mut data = Vec::new();
    for entry in fs::read_dir(".").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let dir_name = entry.file_name().into_string().unwrap();
            let pub_log_path = path.join("bench-pub.log");
            let sub_log_path = path.join("bench-sub.log");

            if pub_log_path.exists() && sub_log_path.exists() {
                let pub_val = parse_log_file(&pub_log_path);
                let sub_val = parse_log_file(&sub_log_path);
                data.push(LogData {
                    dir_name,
                    pub_val,
                    sub_val,
                });
            }
        }
    }

    data.sort_by_key(|d| get_sort_key(&d.dir_name));

    println!("| Sync Period | Pub r/s | Sub r/s |");
    println!("|---|---|---|");
    for d in data {
        println!("| {} | {} | {} |", d.dir_name, d.pub_val, d.sub_val);
    }
}


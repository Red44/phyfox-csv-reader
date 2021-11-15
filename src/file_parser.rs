use std::ops::Range;
use std::path::PathBuf;

pub fn valid_files() -> Vec<PathBuf> {
    let mut res = Vec::<PathBuf>::new();
    for files in std::fs::read_dir(std::env::current_dir().unwrap())
        .unwrap()
        .into_iter()
    {
        let file = files.unwrap();
        if !file.file_type().unwrap().is_file()
            || !file.file_name().to_str().unwrap().ends_with(".csv")
        {
            continue;
        }
        res.push(file.path());
    }
    res
}

pub fn create_output_dir() {
    std::fs::create_dir("out");
}

pub fn attributes(file_string_data: &str) -> Vec<&str> {
    let attributes = file_string_data.split_once("\n").unwrap().0;
    attributes
        .split(",")
        .enumerate()
        .filter(|(p, v)| p != &0)
        .map(|x| x.1)
        .collect()
}
pub fn file_content_integrity_check(file_string_data: &str) -> bool {
    if !file_string_data.len() >= 12 {
        return false;
    }
    file_string_data.starts_with("\"Time (s)\",")
}
pub fn extract_data(file_string_data: &str) -> Vec<Vec<f64>> {
    let mut res: Vec<Vec<f64>> = Vec::new();
    for line in file_string_data.split("\n").enumerate() {
        if line.0 == 0 {
            for _ in 0..line.1.split(',').count() {
                res.push(Vec::new());
            }
            continue;
        }
        for (p, v) in line.1.split(',').enumerate() {
            res[p].push(v.parse().unwrap_or(0.0));
        }
    }
    res
}

fn extract_from_raw_data(parsed_file_data: &mut Vec<Vec<f64>>, time_range: Range<i64>) {
    parsed_file_data[0].retain(|point_in_time| time_range.contains(&(point_in_time.round() as i64)))
}

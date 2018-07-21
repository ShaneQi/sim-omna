extern crate chrono;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::ffi::OsStr;
use chrono::prelude::*;

fn main() {
    let mut args = env::args();
    let _ = args.next();
    let input_path = args.next().unwrap();
    let output_path = args.next().unwrap();
    let mut file_paths = files(input_path);
    file_paths.sort_by(|a, b| {
        let file_a_name = PathBuf::from(a)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let file_b_name = PathBuf::from(b)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        file_a_name.cmp(&file_b_name)
    });
    let output_path_clone = output_path.clone();
    let output_path_clone_2 = output_path.clone();
    if !PathBuf::from(output_path_clone).exists() {
        let _ = fs::create_dir_all(output_path_clone_2);
    }
    for file in file_paths {
        let file_path = PathBuf::from(file.clone());
        if file_path.extension() == Some(OsStr::new("mp4")) {
            let file_name = file_path.file_stem().unwrap().to_str().unwrap();
            let mut date = Utc.datetime_from_str(&file_name, "%Y%m%d_%H%M%S").unwrap();
            let dallas_date = date.with_timezone(&FixedOffset::east(-5*3600));
            let dallas_date_string = dallas_date.format("%Y%m%d_%H%M%S").to_string();
            let _ = fs::copy(file, output_path.clone() + "/" + &dallas_date_string + ".mp4");
        }
    }
}

fn files(path: String) -> Vec<String> {
    let mut file_paths: Vec<String> = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            for file_path in files(path.into_os_string().into_string().unwrap()) {
                file_paths.push(file_path)
            }
        } else {
            file_paths.push(path.into_os_string().into_string().unwrap());
        }
    }
    file_paths
}

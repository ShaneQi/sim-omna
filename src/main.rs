extern crate chrono;
extern crate clap;

use chrono::prelude::*;
use clap::{App, Arg};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

fn main() {
    let matches = App::new("Simple Omna")
        .version("1.0")
        .author("Shane Qi <qizengtai@gmail.com>")
        .about("Simplify your Omna Cam video storage.")
        .arg(
            Arg::with_name("timezone")
                .short("c")
                .long("timezone")
                .value_name("TIMEZONE")
                .help("Convert video file names from a timezone to another one. (requires 2 values) (e.g. convert from UTC to CDT: `-c UTC CDT` or `-c 0000 0500`)")
                .takes_value(true)
                .number_of_values(2)
        )
        .arg(
            Arg::with_name("thumbnail")
                .short("t")
                .long("thumbnail")
                .value_name("THUMBNAIL")
                .help("Use this flag if you want to keep video's thumbnails.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input directory to use. (e.g. /Volumns/Omna/DSH-C310/20180721)")
                .required(true),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Sets the output directory to put simpified videos.")
                .required(true),
        )
        .get_matches();

    let needs_thumbnail = matches.is_present("thumbnail");
    let input_path_string = matches.value_of("INPUT").unwrap();
    let output_path_string = matches.value_of("OUTPUT").unwrap();
    let filter: Fn(PathBuf) -> bool = &|x: PathBuf| true;
    
    // |path| {
    //     if path.extension() == Some(OsStr::new("mp4")) {
    //         return true;
    //     } else if needs_thumbnail && path.extension() == Some(OsStr::new("jpg")) {
    //         return true;
    //     }
    //     false
    // };
    let all_files = all_files(PathBuf::from(input_path_string.to_string()), &filter);

    // let mut file_paths = files(input_path);
    // file_paths.sort_by(|a, b| {
    //     let file_a_name = PathBuf::from(a)
    //         .file_name()
    //         .unwrap()
    //         .to_str()
    //         .unwrap()
    //         .to_string();
    //     let file_b_name = PathBuf::from(b)
    //         .file_name()
    //         .unwrap()
    //         .to_str()
    //         .unwrap()
    //         .to_string();
    //     file_a_name.cmp(&file_b_name)
    // });
    // let output_path_clone = output_path.clone();
    // let output_path_clone_2 = output_path.clone();
    // if !PathBuf::from(output_path_clone).exists() {
    //     let _ = fs::create_dir_all(output_path_clone_2);
    // }
    // for file in file_paths {
    //     let file_path = PathBuf::from(file.clone());
    //     if file_path.extension() == Some(OsStr::new("mp4")) {
    //         let file_name = file_path.file_stem().unwrap().to_str().unwrap();
    //         let mut date = Utc.datetime_from_str(&file_name, "%Y%m%d_%H%M%S").unwrap();
    //         let dallas_date = date.with_timezone(&FixedOffset::east(-5 * 3600));
    //         let dallas_date_string = dallas_date.format("%Y%m%d_%H%M%S").to_string();
    //         let _ = fs::copy(
    //             file,
    //             output_path.clone() + "/" + &dallas_date_string + ".mp4",
    //         );
    //     }
    // }
}

fn all_files<F>(path: PathBuf, filter: &F) -> Vec<PathBuf>
where
    F: Fn(PathBuf) -> bool,
{
    let mut file_paths: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            for file_path in all_files(
                PathBuf::from(path.into_os_string().into_string().unwrap()),
                filter,
            ) {
                file_paths.push(file_path)
            }
        } else if filter(path) {
            file_paths.push(PathBuf::from(path.into_os_string().into_string().unwrap()));
        }
    }
    file_paths
}

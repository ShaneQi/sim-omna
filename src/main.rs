extern crate chrono;
extern crate clap;

use chrono::prelude::*;
use clap::{App, Arg};
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
                .help("Convert video file names from a timezone to another one. (requires 2 values) (e.g. convert from UTC to CDT: `-c 0000 -0500`)")
                .takes_value(true)
                .number_of_values(2)
                .allow_hyphen_values(true)
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
    let timezone_inputs = matches.values_of("timezone");
    let mut timezone_conversion: Option<(FixedOffset, FixedOffset)> = None;
    if let Some(mut timezone_inputs) = timezone_inputs {
        if timezone_inputs.len() >= 2 {
            let from_str = timezone_inputs.next().unwrap();
            let from_timezone = from_str
                .to_string()
                .parse::<i32>()
                .ok()
                .and_then(|offset| timezone(offset));
            let to_str = timezone_inputs.next().unwrap();
            let to_timezone = to_str
                .to_string()
                .parse::<i32>()
                .ok()
                .and_then(|offset| timezone(offset));
            if let Some(from_timezone) = from_timezone {
                if let Some(to_timezone) = to_timezone {
                    timezone_conversion = Some((from_timezone, to_timezone));
                } else {
                    println!(
                        "\"{}\" isn't a valid timezone offset, an example of valid value is '0500'",
                        to_str
                    );
                }
            } else {
                println!(
                    "\"{}\" isn't a valid timezone offset, an example of valid value is '0500'",
                    from_str
                );
            }
        }
    }
    let input_path_string = matches.value_of("INPUT").unwrap();
    let output_path_string = matches.value_of("OUTPUT").unwrap();

    let mut all_files = all_files(PathBuf::from(input_path_string.to_string()));
    let len = all_files.len();
    for i in 1..=len {
        let index = len - i;
        if !is_omna_file(&all_files[index]) {
            all_files.remove(index);
        } else if all_files[index].extension() != Some(OsStr::new("mp4")) {
            if !needs_thumbnail || all_files[index].extension() != Some(OsStr::new("jpg")) {
                all_files.remove(index);
            }
        }
    }

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

    if !PathBuf::from(output_path_string).exists() {
        let _ = fs::create_dir_all(output_path_string);
    }
    for file_path in all_files {
        let mut output_path = PathBuf::from(output_path_string);
        if let Some((from_timezone, to_timezone)) = timezone_conversion {
            let file_path_clone = file_path.clone();
            let file_path_clone_2 = file_path.clone();
            if let (Some(file_stem), Some(file_extension)) = (
                file_path_clone.file_stem().and_then(|stem| stem.to_str()),
                file_path_clone_2.extension().and_then(|stem| stem.to_str()),
            ) {
                if let Ok(date) = from_timezone.datetime_from_str(&file_stem, &omna_date_pattern())
                {
                    let date_format = omna_date_pattern();
                    let converted_date_string =
                        date.with_timezone(&to_timezone).format(&date_format);
                    output_path.push(
                        format!("{}", converted_date_string).to_string() + "." + file_extension,
                    );
                }
            }
        } else {
            let file_path_clone = file_path.clone();
            if let Some(file_name) = file_path_clone.file_name().and_then(|name| name.to_str()) {
                output_path.push(file_name);
            }
        }
        let _ = fs::copy(file_path, output_path);
    }
}

fn all_files(path: PathBuf) -> Vec<PathBuf> {
    let mut file_paths: Vec<PathBuf> = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            let path = entry.unwrap().path();
            if path.is_dir() {
                for file_path in all_files(path) {
                    file_paths.push(file_path)
                }
            } else {
                file_paths.push(path);
            }
        }
    }

    file_paths
}

fn is_omna_file(path: &PathBuf) -> bool {
    if let Some(_) = path.file_stem()
        .and_then(|stem| stem.to_str())
        .and_then(|path_str| Utc.datetime_from_str(path_str, &omna_date_pattern()).ok())
    {
        return true;
    }
    false
}

fn omna_date_pattern() -> String {
    "%Y%m%d_%H%M%S".to_string()
}

fn timezone(offset: i32) -> Option<FixedOffset> {
    let hours = offset / 100;
    let mins = offset % 100;
    if hours > 12 {
        return None;
    } else if mins > 60 {
        return None;
    } else if hours == 12 && mins != 0 {
        return None;
    } else {
        return Some(FixedOffset::east(hours * 3600 + mins * 60));
    }
}

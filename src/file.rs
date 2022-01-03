use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;
use chrono::{DateTime, Local};

use crate::constants::*;

#[derive(Debug)]
pub struct FileDescription{
    href: String,
    filename: String,
    date: String,
    size: u64,
    file_type: String
}

impl FileDescription {
    pub fn to_json(&self) -> String {

        let mut result = String::from("{");
        result += &format!("\"href\":\"{}\",", self.href);
        result += &format!("\"filename\":\"{}\",", self.filename);
        result += &format!("\"date\":\"{}\",", self.date);
        result += &format!("\"size\":{},", self.size);
        result += &format!("\"file_type\":\"{}\"", self.file_type);
        result += "}";

        return result;
    }
}

pub fn get_capture_files(path: &str) -> Vec<FileDescription> {

    let content_dir = fs::read_dir(path);
    let mut result : Vec<FileDescription> = vec!();

    match content_dir {
        Ok(content) => {
            for entry in content {
                if !is_valid_file(&entry) { continue; }
                
                let pathbuf = entry.unwrap().path();

                result.push( FileDescription{
                    href: format!("{}{}", HREF_CAPTURE_PATH, pathbuf.file_name().unwrap().to_str().unwrap()),
                    filename: String::from(pathbuf.file_stem().unwrap().to_str().unwrap()),
                    date: get_creation_date(&pathbuf),
                    size: get_size_file(&pathbuf),
                    file_type: get_type_file(&pathbuf)
                });

            }
        },
        Err(e) => eprint!("{}", e)
    };

    return result;
}

fn is_valid_file(file_path: &std::io::Result<DirEntry>) -> bool {

    if file_path.is_err() { return false; }

    let path = file_path.as_ref().unwrap().path();
    let extension = path.extension();

    if extension.is_none() { return false; }

    match extension.unwrap().to_str(){
        Some("jpg") |
        Some("h264") => true,
        _ => false
    }
}

fn get_creation_date(path: &PathBuf) -> String {

    let meta = fs::metadata(path);

    if meta.is_err() {
        eprintln!("[get_creation_date] {}", meta.unwrap_err());
        return String::from("Unknown");
    }

    let datetime : DateTime<Local> = DateTime::from(meta.unwrap().created().unwrap());

    format!("{}", datetime.format("%Y-%m-%d %H:%M:%S"))
}

fn get_size_file(path: &PathBuf) -> u64 {
    let meta = fs::metadata(path);

    if meta.is_err() {
        eprintln!("[get_size_file] {}", meta.unwrap_err());
        return 0;
    }

    meta.unwrap().len()
}

fn get_type_file(path: &PathBuf) -> String {

    match path.extension().unwrap().to_str(){
        Some("jpg") => String::from("Photo"),
        Some("h264") => String::from("Video"),
        _ => String::from("Unknown")
    }
}
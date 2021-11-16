mod file_parser;
mod file_questionary;
mod html_file_parser;

use crate::file_questionary::start_questionary;
use std::fs;
use std::fs::{File, FileType};
use std::io::Error;
use std::ops::Range;
use std::path::PathBuf;

fn main() {
    for file_contents in file_parser::valid_files()
        .iter()
        .map(|file| match fs::read_to_string(file) {
            Ok(file_content) => {
                return (file.file_name(), file_content);
            }
            Err(err) => {
                println!(
                    "{} affected file : {}",
                    err,
                    file.file_name().unwrap().to_str().unwrap()
                );
                return (file.file_name(), "".to_owned());
            }
        })
        .filter(|file_content| !file_content.1.eq(""))
    {
        if file_parser::file_content_integrity_check(&file_contents.1) {
            println!(
                "{} is not in a valid format ",
                file_contents.0.unwrap().to_str().unwrap()
            );
            continue;
        }
        let file_attrs = file_parser::attributes(&file_contents.1);
        let build_handle = file_questionary::start_questionary(
            file_attrs.clone(),
            file_contents.0.unwrap().to_str().unwrap(),
        );
        let mut file_data = file_parser::extract_data(&file_contents.1);
        file_data = build_handle.config.data_adapter(file_data);
        for (p, v) in build_handle.nodes.iter().enumerate() {
            file_data[p + 1] = v.data_adapter(&file_data[p + 1]);
        }
        let page = html_file_parser::buid_html_page_str(file_data, file_attrs, build_handle);
        fs::write(
            format!("{}.html", file_contents.0.unwrap().to_str().unwrap()),
            page,
        )
        .unwrap_or_default();
    }
}

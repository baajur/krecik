use std::io::{Error, ErrorKind};
use gotham::state::State;
use gotham::handler::IntoResponse;
use hyper::{StatusCode, Body, Response};
use serde_json;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs;
use glob::glob;

use crate::products::expected::*;
use crate::checks::page::Pages;
use crate::checks::domain::Domains;
use crate::configuration::CHECKS_DIR;


/// Produce list of dirs/files matching given glob pattern:
pub fn produce_list(glob_pattern: &str) -> Vec<String> {
    let mut list = vec!();
    for entry in glob(&glob_pattern).unwrap() {
        match entry {
            Ok(path) => {
                match path.file_name() {
                    Some(element) => {
                        element
                            .to_str()
                            .and_then(|elem| {
                                list.push(elem.to_string());
                                Some(elem.to_string())
                            });
                    },
                    None => (),
                }
            },
            Err(err) => {
                error!("Error: produce_list(): {}", err);
            },
        }
    }
    debug!("produce_list(): Elements: {:?}", list);
    list
}


/// List all check files found in default checks dir
pub fn list_check_files() -> Vec<String> {
    list_check_files_from(CHECKS_DIR)
}


/// List all check files from given dir
pub fn list_check_files_from(checks_dir: &str) -> Vec<String> {
    let glob_pattern = format!("{}/*.json", checks_dir);
    debug!("list_check_files(): {}", glob_pattern);
    produce_list(&glob_pattern)
}


/// Read text file
pub fn read_text_file(name: &str) -> Result<String, Error> {
    fs::read_to_string(name)
}

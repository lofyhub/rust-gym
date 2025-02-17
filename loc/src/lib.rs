mod utils;

pub use crate::utils::{get_language_name, is_hidden};
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{collections::HashMap, env};

use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Files {
    pub counts: HashMap<String, FileInfo>,
    pub loc: HashMap<String, i32>,
}

#[derive(Debug, Clone, Copy)]
pub struct FileInfo {
    pub comments_count: i32,
    pub blank_spaces_count: i32,
    // this includes number of lines that might include code
    pub lines_of_code: i32,
    // includes the number of files with the given file extension
    pub nums_of_files: i32,
}

impl FileInfo {
    pub fn new() -> Self {
        FileInfo {
            comments_count: 0,
            blank_spaces_count: 0,
            lines_of_code: 0,
            nums_of_files: 1,
        }
    }
    pub fn update(&mut self, file_info: FileInfo) {
        self.comments_count += file_info.comments_count;
        self.blank_spaces_count += file_info.blank_spaces_count;
        self.lines_of_code += file_info.lines_of_code;
        self.nums_of_files += file_info.nums_of_files;
    }
}

impl Files {
    pub fn new(path: Option<PathBuf>) -> Self {
        let mut files = Files {
            counts: HashMap::new(),
            loc: HashMap::new(),
        };

        let current_dir = env::current_dir().unwrap();

        let pwd: PathBuf = match path {
            Some(path) => path,
            None => current_dir,
        };

        for path in WalkDir::new(pwd)
            .into_iter()
            .filter_entry(|e| !is_hidden(e))
            .filter_map(|dir| dir.ok())
        {
            if path.metadata().unwrap().is_file() {
                let file_name = path.path().to_string_lossy().to_string();

                // Check file metadata for read permissions
                if !Files::is_file_readable(&file_name) {
                    continue;
                }

                let file_ext = Files::get_extension_from_filename(&file_name);

                if let Some(file_ext) = file_ext {
                    if get_language_name(file_ext) == "unrecognized" {
                        // the file extention doesnt seem to be in out match so we ignore and continue
                        continue;
                    }
                    let file_info = Files::read_all_lines_of_code(&file_name);
                    files.update_file_info(&file_ext.to_string(), file_info);
                }
            }
        }
        files
    }
    pub fn update_file_info(&mut self, file_ext: &str, new_info: FileInfo) {
        self.counts
            .entry(file_ext.to_string())
            .and_modify(|existing_info| existing_info.update(new_info))
            .or_insert_with(|| new_info);
    }
    pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
        Path::new(filename).extension().and_then(OsStr::to_str)
    }
    pub fn is_file_readable(filename: &str) -> bool {
        match fs::metadata(filename) {
            Ok(metadata) => !metadata.permissions().readonly(),
            Err(_e) => false,
        }
    }
    pub fn read_all_lines_of_code(filename: &str) -> FileInfo {
        let mut file = File::open(filename).unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();

        let mut line_count = 0;
        let mut comments_count = 0;
        let mut blank_spaces_count = 0;
        let nums_of_files = 1;

        let contents = String::from_utf8_lossy(&contents);

        let mut in_multiline_comment = false;

        for line in contents.lines() {
            let trimmed_line = line.trim();

            if in_multiline_comment {
                comments_count += 1;
                if trimmed_line.ends_with("*/")
                    || trimmed_line.ends_with("'''")
                    || trimmed_line.ends_with("\"\"\"")
                    || trimmed_line.ends_with("-->")
                {
                    in_multiline_comment = false;
                }
            } else if trimmed_line.starts_with("//") || // C, C++, Java, JavaScript
                      trimmed_line.starts_with("#") || // Python, Ruby, Shell, Perl
                      trimmed_line.starts_with("--") || // SQL
                      trimmed_line.starts_with("//!") || // Rust documentation
                      trimmed_line.starts_with("///") || // Rust
                      trimmed_line.starts_with(";")
            {
                // Lisp, Scheme
                comments_count += 1;
            } else if trimmed_line.starts_with("/*") {
                // C, C++, Java (start of multi-line comment)
                comments_count += 1;
                if !trimmed_line.ends_with("*/") {
                    // Only set flag if it doesn't end on the same line
                    in_multiline_comment = true;
                }
            } else if trimmed_line.starts_with("'''")
                || trimmed_line.starts_with("\"\"\"")
                || trimmed_line.starts_with("<!--")
            {
                // Python/HTML multi-line comments
                comments_count += 1;
                if !(trimmed_line.ends_with("'''")
                    || trimmed_line.ends_with("\"\"\"")
                    || trimmed_line.ends_with("-->"))
                {
                    in_multiline_comment = true;
                }
            } else if trimmed_line.is_empty() {
                blank_spaces_count += 1;
            } else {
                line_count += 1;
            }
        }

        FileInfo {
            comments_count,
            blank_spaces_count,
            lines_of_code: line_count,
            nums_of_files,
        }
    }
}

use crate::*;
use crate::console::*;

use std::fs::*;
use std::io::*;
use std::time::SystemTime;
use std::result::Result;

pub type FileResult<T> = Result<T, FileError>;

#[derive(Clone, PartialEq)]
pub enum FileError {
    CurrentDirectoryReadFailure {},
    DirectoryReadFailure { dir_path: String },
    FileOpenFailure { file_path: String },
    FileReadFailure { file_path: String },
    FileWriteFailure { file_path: String },
    EnvironmentVariableReadFailure { var_name: String },
    InvalidPath { path: String },
    MetadataReadFailure { path: String },
    PathNotDirectory { path: String },
    PathNotExists { path: String },
    PathNotFile { path: String },
}

impl ConsoleLogger for FileError {
    fn get_log(&self) -> ConsoleLog {
        match self {
            FileError::CurrentDirectoryReadFailure {} => log!(Error, "{^file.err.1069}", format!("?{{^console.spec_link}}: https://ches.gant.work/en/spec/console/file/error/1069/index.html")),
            FileError::DirectoryReadFailure { dir_path } => log!(Error, "{^file.err.5978}", format!("{{^file.dir_path}}: {}", dir_path), format!("?{{^console.spec_link}}: https://ches.gant.work/en/spec/console/file/error/5978/index.html")),
            FileError::FileOpenFailure { file_path } => log!(Error, "{^file.err.0117}", format!("{{^file.file_path}}: {}", file_path), format!("?{{^console.spec_link}}: https://ches.gant.work/en/spec/console/file/error/0117/index.html")),
            FileError::FileReadFailure { file_path } => log!(Error, "{^file.err.3995}", format!("{{^file.file_path}}: {}", file_path), format!("?{{^console.spec_link}}: https://ches.gant.work/en/spec/console/file/error/3995/index.html")),
            FileError::FileWriteFailure { file_path } => log!(Error, "{^file.err.}", format!("{{^file.file_path}}: {}", file_path), format!("?{{^console.spec_link}}: https://ches.gant.work/en/spec/console/file/error//index.html")),
            FileError::EnvironmentVariableReadFailure { var_name } => log!(Error, "{^file.err.9798}", format!("{{^file.env_var_name}}: {}", var_name), format!("{{^console.spec_link}}: https://ches.gant.work/en/spec/console/file/error/9798/index.html")),
            FileError::InvalidPath { path } => log!(Error, "{^file.err.2711}", format!("{{^file.file_path}}: {}", path), format!("?{{^console.spec_link}}: https://ches.gant.work/en/spec/console/file/error/2711/index.html")),
            FileError::MetadataReadFailure { path } => log!(Error, "metadata read failure", format!("{{^file.path}}: {}", path)),
            FileError::PathNotDirectory { path } => log!(Error, "{^file.err.0077}", format!("{{^file.file_path}}: {}", path), format!("?{{^console.spec_link}}: https://ches.gant.work/en/spec/console/file/error/0077/index.html")),
            FileError::PathNotExists { path } => log!(Error, "{^file.err.8531}", format!("{{^file.file_path}}: {}", path), format!("?{{^console.spec_link}}: https://ches.gant.work/en/spec/console/file/error/8531/index.html")),
            FileError::PathNotFile { path } => log!(Error, "{^file.err.2160}", format!("{{^file.file_path}}: {}", path), format!("?{{^console.spec_link}}: https://ches.gant.work/en/spec/console/file/error/2160/index.html")),
        }
    }
}

pub struct FileMan {}

impl FileMan {
    pub fn exists(path: &str) -> bool {
        return std::path::Path::new(path).exists();
    }

    pub fn get_abs_path(rel_path: &str) -> std::result::Result<std::boxed::Box<std::path::Path>, FileError> {
        let rel_path_obj = std::path::Path::new(rel_path);
        let curr_dir_path_obj = match std::env::current_dir() {
            Ok(v) => v,
            Err(_) => return Err(FileError::CurrentDirectoryReadFailure {}),
        };

        return Ok(std::boxed::Box::from(curr_dir_path_obj.join(rel_path_obj)));
    }

    pub fn get_last_modified(path: &str) -> Result<SystemTime, FileError> {
        return match metadata(path) {
            Ok(metadata) => {
                match metadata.modified() {
                    Ok(time) => Ok(time),
                    Err(_) => Err(FileError::MetadataReadFailure {
                        path: path.to_string(),
                    }),
                }
            },
            Err(_) => Err(FileError::MetadataReadFailure {
                path: path.to_string(),
            }),
        };
    }

    pub fn get_parent_dir_path(path: &str) -> std::result::Result<std::option::Option<std::boxed::Box<std::path::Path>>, FileError> {
        if !FileMan::exists(path) {
            return Err(FileError::PathNotExists { path: path.to_string() });
        }

        let parent_path = match std::path::Path::new(path).parent() {
            Some(v) => v,
            None => return Ok(None),
        };

        println!("{}", parent_path.to_str().unwrap());
        println!("{}", parent_path.to_str().unwrap());
        println!("{}", parent_path.to_str().unwrap());

        return Ok(Some(std::boxed::Box::from(parent_path)));
    }

    pub fn is_dir(path: &str) -> bool {
        return std::path::Path::new(path).is_dir();
    }

    pub fn is_same(path1: &str, path2: &str) -> std::result::Result<bool, FileError> {
        return match same_file::is_same_file(path1, path2) {
            Ok(v) => Ok(v),
            Err(_) => Err(FileError::InvalidPath { path: format!("{}; {}", path1, path2) }),
        };
    }

    pub fn join_path(orig_path: &str, rel_path: &str) -> std::result::Result<std::boxed::Box<std::path::Path>, FileError> {
        let orig_path_obj = std::path::Path::new(orig_path);
        let rel_path_obj = std::path::Path::new(rel_path);
        let joined_path_obj = orig_path_obj.join(rel_path_obj);

        return match joined_path_obj.canonicalize() {
            Ok(v) => Ok(std::boxed::Box::from(v)),
            Err(_) => Err(FileError::InvalidPath { path: joined_path_obj.to_str().unwrap().to_string() }),
        };
    }

    pub fn read_all(path: &str) -> std::result::Result<String, FileError> {
        if !FileMan::exists(&path) {
            return Err(FileError::PathNotExists { path: path.to_string() });
        }

        if FileMan::is_dir(&path) {
            return Err(FileError::PathNotFile { path: path.to_string() });
        }

        let content = match std::fs::read_to_string(path) {
            Ok(v) => v,
            Err(_) => return Err(FileError::FileReadFailure { file_path: path.to_string() }),
        };

        return Ok(content);
    }

    pub fn read_all_bytes(path: &str) -> std::result::Result<Vec<u8>, FileError> {
        if !FileMan::exists(&path) {
            return Err(FileError::PathNotExists { path: path.to_string() });
        }

        if FileMan::is_dir(&path) {
            return Err(FileError::PathNotFile { path: path.to_string() });
        }

        let mut reader = match std::fs::File::open(path) {
            Ok(v) => BufReader::new(v),
            Err(_) => return Err(FileError::FileOpenFailure { file_path: path.to_string() }),
        };

        let mut bytes = Vec::<u8>::new();
        let mut buf = [0; 4];

        loop {
            match reader.read(&mut buf) {
                Ok(v) => {
                    match v {
                        0 => break,
                        n => {
                            let buf = &buf[..n];
                            bytes.append(&mut buf.to_vec());
                        }
                    }
                },
                Err(_) => return Err(FileError::FileReadFailure { file_path: path.to_string() }),
            }
        }

        return Ok(bytes);
    }

    pub fn read_lines(path: &str) -> std::result::Result<Vec<String>, FileError> {
        if !FileMan::exists(&path) {
            return Err(FileError::PathNotExists { path: path.to_string() });
        }

        if FileMan::is_dir(&path) {
            return Err(FileError::PathNotFile { path: path.to_string() });
        }

        let reader = match std::fs::File::open(path) {
            Ok(v) => v,
            Err(_) => return Err(FileError::FileOpenFailure { file_path: path.to_string() }),
        };

        let mut lines = Vec::<String>::new();

        for each_line in std::io::BufReader::new(reader).lines() {
            lines.push(match each_line {
                Ok(v) => v,
                Err(_) => return Err(FileError::FileReadFailure { file_path: path.to_string() }),
            });
        }

        return Ok(lines);
    }

    pub fn rename_ext(path: &str, new_ext: &str) -> String {
        let split_path: Vec<&str> = path.split(".").collect();

        // 拡張子がついていない場合は新しく付け足す
        if split_path.len() < 2 {
            return path.to_string() + "." + new_ext;
        }

        let old_ext_raw: Vec<&str> = split_path[split_path.len() - 1..split_path.len()].to_vec();
        let old_ext = old_ext_raw.get(0).unwrap();

        return path[0..path.len() - old_ext.len()].to_string() + new_ext;
    }

    pub fn write_all_bytes(path: &str, bytes: &Vec<u8>) -> std::result::Result<(), FileError> {
        let mut file = match std::fs::File::create(path) {
            Err(_) => return Err(FileError::FileOpenFailure { file_path: path.to_string() }),
            Ok(v) => v,
        };

        match file.write_all(bytes) {
            Err(_) => return Err(FileError::FileWriteFailure { file_path: path.to_string() }),
            Ok(v) => v,
        };

        return Ok(());
    }
}

use crate::console;

use std::io::*;

#[derive(Debug)]
pub enum FileManError {
    CurrDirReadFailure(),
    DirReadFailure(String),
    FileNotOpenable(String),
    FileReadFailure(String),
    FileWriteFailure(String),
    EnvVarReadFailure(String),
    InvalidPath(String),
    PathNotDirectory(String),
    PathNotFile(String),
    PathNotExists(String),
}

impl FileManError {
    pub fn get_log_data(&self) -> console::ConsoleLogData {
        match self {
            FileManError::CurrDirReadFailure() => console::ConsoleLogData::new(console::ConsoleLogKind::Error, "{^file.err.1069}", vec![], vec![format!("{{^console.spec_link}}: https://ches.gant.work/en/spec/console/file/error/1069/index.html")]),
            FileManError::DirReadFailure(dir_path) => console::ConsoleLogData::new(console::ConsoleLogKind::Error, "{^file.err.5978}", vec![format!("{{^file.dir_path}}: {}", dir_path)], vec![format!("{{^console.spec_link}}: https://ches.gant.work/en/spec/console/file/error/5978/index.html")]),
            FileManError::FileNotOpenable(file_path) => console::ConsoleLogData::new(console::ConsoleLogKind::Error, "{^file.err.0117}", vec![format!("{{^file.file_path}}: {}", file_path)], vec![format!("{{^console.spec_link}}: https://ches.gant.work/en/spec/console/file/error/0117/index.html")]),
            FileManError::FileReadFailure(file_path) => console::ConsoleLogData::new(console::ConsoleLogKind::Error, "{^file.err.3995}", vec![format!("{{^file.file_path}}: {}", file_path)], vec![format!("{{^console.spec_link}}: https://ches.gant.work/en/spec/console/file/error/3995/index.html")]),
            FileManError::FileWriteFailure(file_path) => console::ConsoleLogData::new(console::ConsoleLogKind::Error, "{^file.err.}", vec![format!("{{^file.file_path}}: {}", file_path)], vec![format!("{{^console.spec_link}}: https://ches.gant.work/en/spec/console/file/error//index.html")]),
            FileManError::EnvVarReadFailure(env_name) => console::ConsoleLogData::new(console::ConsoleLogKind::Error, "{^file.err.9798}", vec![format!("{{^file.env_var_name}}: {}", env_name)], vec![format!("{{^console.spec_link}}: https://ches.gant.work/en/spec/console/file/error/9798/index.html")]),
            FileManError::InvalidPath(path) => console::ConsoleLogData::new(console::ConsoleLogKind::Error, "{^file.err.2711}", vec![format!("{{^file.file_path}}: {}", path)], vec![format!("{{^console.spec_link}}: https://ches.gant.work/en/spec/console/file/error/2711/index.html")]),
            FileManError::PathNotDirectory(path) => console::ConsoleLogData::new(console::ConsoleLogKind::Error, "{^file.err.0077}", vec![format!("{{^file.file_path}}: {}", path)], vec![format!("{{^console.spec_link}}: https://ches.gant.work/en/spec/console/file/error/0077/index.html")]),
            FileManError::PathNotExists(path) => console::ConsoleLogData::new(console::ConsoleLogKind::Error, "{^file.err.8531}", vec![format!("{{^file.file_path}}: {}", path)], vec![format!("{{^console.spec_link}}: https://ches.gant.work/en/spec/console/file/error/8531/index.html")]),
            FileManError::PathNotFile(path) => console::ConsoleLogData::new(console::ConsoleLogKind::Error, "{^file.err.2160}", vec![format!("{{^file.file_path}}: {}", path)], vec![format!("{{^console.spec_link}}: https://ches.gant.work/en/spec/console/file/error/2160/index.html")]),
        }
    }
}

impl std::error::Error for FileManError {}

impl std::fmt::Display for FileManError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "CommandError");
    }
}

pub struct FileMan {}

impl FileMan {
    pub fn exists(path: &str) -> bool {
        return std::path::Path::new(path).exists();
    }

    pub fn get_abs_path(rel_path: &str) -> std::result::Result<std::boxed::Box<std::path::Path>, FileManError> {
        let rel_path_obj = std::path::Path::new(rel_path);
        let curr_dir_path_obj = match std::env::current_dir() {
            Ok(v) => v,
            Err(_e) => return Err(FileManError::CurrDirReadFailure()),
        };

        return Ok(std::boxed::Box::from(curr_dir_path_obj.join(rel_path_obj)));
    }

    pub fn get_parent_dir_path(path: &str) -> std::result::Result<std::option::Option<std::boxed::Box<std::path::Path>>, FileManError> {
        if !FileMan::exists(path) {
            return Err(FileManError::PathNotExists(path.to_string()));
        }

        let parent_path = match std::path::Path::new(path).parent() {
            Some(v) => v,
            None => return Ok(None),
        };

        println!("{}", parent_path.to_str().unwrap().to_string());
        println!("{}", parent_path.to_str().unwrap().to_string());
        println!("{}", parent_path.to_str().unwrap().to_string());

        return Ok(Some(std::boxed::Box::from(parent_path)));
    }

    pub fn get_root_path() -> std::result::Result<String, FileManError> {
        let env_var_name = "CHES_HOME";

        return match std::env::var(env_var_name) {
            Ok(v) => Ok(v),
            Err(_e) => Err(FileManError::EnvVarReadFailure(env_var_name.to_string())),
        }
    }

    pub fn get_langpack_path(lang_name: &str) -> std::result::Result<String, FileManError> {
        return Ok(FileMan::get_root_path()? + "/lib/lang/" + lang_name + ".lang");
    }

    pub fn is_dir(path: &str) -> bool {
        return std::path::Path::new(path).is_dir();
    }

    pub fn is_same(path1: &str, path2: &str) -> std::result::Result<bool, FileManError> {
        return match same_file::is_same_file(path1, path2) {
            Ok(v) => Ok(v),
            Err(_e) => Err(FileManError::InvalidPath(format!("{}, {}", path1, path2))),
        };
    }

    pub fn join_path(orig_path: &str, rel_path: &str) -> std::result::Result<std::boxed::Box<std::path::Path>, FileManError> {
        let orig_path_obj = std::path::Path::new(orig_path);
        let rel_path_obj = std::path::Path::new(rel_path);
        let joined_path_obj = orig_path_obj.join(rel_path_obj);

        return match joined_path_obj.canonicalize() {
            Ok(v) => Ok(std::boxed::Box::from(v)),
            Err(_e) => Err(FileManError::InvalidPath(joined_path_obj.to_str().unwrap().to_string())),
        };
    }

    pub fn read_all(path: &str) -> std::result::Result<String, FileManError> {
        if !FileMan::exists(&path) {
            return Err(FileManError::PathNotExists(path.to_string()));
        }

        if FileMan::is_dir(&path) {
            return Err(FileManError::PathNotFile(path.to_string()));
        }

        let content = match std::fs::read_to_string(path) {
            Ok(v) => v,
            Err(_e) => return Err(FileManError::FileReadFailure(path.to_string())),
        };

        return Ok(content);
    }

    pub fn read_lines(path: &str) -> std::result::Result<Vec<String>, FileManError> {
        if !FileMan::exists(&path) {
            return Err(FileManError::PathNotExists(path.to_string()));
        }

        if FileMan::is_dir(&path) {
            return Err(FileManError::PathNotFile(path.to_string()));
        }

        let reader = match std::fs::File::open(path) {
            Ok(v) => v,
            Err(_e) => return Err(FileManError::FileNotOpenable(path.to_string())),
        };

        let mut lines = Vec::<String>::new();

        for each_line in std::io::BufReader::new(reader).lines() {
            lines.push(match each_line {
                Ok(v) => v,
                Err(_e) => return Err(FileManError::FileReadFailure(path.to_string())),
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

    pub fn write_all_bytes(path: &str, bytes: &Vec<u8>) -> std::result::Result<(), FileManError> {
        let mut file = match std::fs::File::create(path) {
            Err(_e) => return Err(FileManError::FileNotOpenable(path.to_string())),
            Ok(v) => v,
        };

        match file.write_all(bytes) {
            Err(_e) => return Err(FileManError::FileWriteFailure(path.to_string())),
            Ok(v) => v,
        };

        return Ok(());
    }
}

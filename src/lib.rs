pub mod console;
pub mod file;

use crate::console::*;

enum InternalLanguage {
    English,
    Japanese,
}

impl InternalLanguage {
    pub fn from(s: &str) -> Option<InternalLanguage> {
        let v = match s {
            "en" => InternalLanguage::English,
            "ja" => InternalLanguage::Japanese,
            _ => return None,
        };

        return Some(v);
    }
}

#[derive(Clone, PartialEq)]
enum InternalTranslator {
            // note: log titles
    ExpectedDirectoryPathNotFilePath,
    ExpectedFilePathNotDirectoryPath,
    FailedToGetCurrentDirectory,
    FailedToGetEnvironmentVariable,
    FailedToOpenDirectory,
    FailedToOpenFile,
    FailedToOpenFileOrDirectory,
    FailedToReadFile,
    FailedToWriteFile,
    LogLimitExceeded { log_limit: ConsoleLogLimit },
    PathDoesNotExist,
    // note: descriptions
    NameDescription { name: String },
    PathDescription { path: String },
}

impl ConsoleLogTranslator for InternalTranslator {
    fn translate(&self, lang_name: &str) -> String {
        let lang = match InternalLanguage::from(lang_name) {
            Some(v) => v,
            None => return "{unknown language}".to_string(),
        };

        return translate!{
            translator => self,
            lang => lang,
            // note: log titles
            InternalTranslator::ExpectedDirectoryPathNotFilePath => {
                InternalLanguage::English => "expected directory path not file path",
                InternalLanguage::Japanese => "ファイルパスでなくディレクトリパスが必要です",
            },
            InternalTranslator::ExpectedFilePathNotDirectoryPath => {
                InternalLanguage::English => "expected file path not directory path",
                InternalLanguage::Japanese => "ディレクトリパスでなくファイルパスが必要です",
            },
            InternalTranslator::FailedToGetCurrentDirectory => {
                InternalLanguage::English => "failed to get current directory",
                InternalLanguage::Japanese => "カレントディレクトリの取得に失敗しました",
            },
            InternalTranslator::FailedToGetEnvironmentVariable => {
                InternalLanguage::English => "failed to get environment variable",
                InternalLanguage::Japanese => "環境変数の取得に失敗しました",
            },
            InternalTranslator::FailedToOpenDirectory => {
                InternalLanguage::English => "failed to open directory",
                InternalLanguage::Japanese => "ディレクトリのオープンに失敗しました",
            },
            InternalTranslator::FailedToOpenFile => {
                InternalLanguage::English => "failed to open file",
                InternalLanguage::Japanese => "ファイルのオープンに失敗しました",
            },
            InternalTranslator::FailedToOpenFileOrDirectory => {
                InternalLanguage::English => "failed to open file or directory",
                InternalLanguage::Japanese => "ファイルもしくはディレクトリのオープンに失敗しました",
            },
            InternalTranslator::FailedToReadFile => {
                InternalLanguage::English => "failed to read file",
                InternalLanguage::Japanese => "ファイルの読み込みに失敗しました",
            },
            InternalTranslator::FailedToWriteFile => {
                InternalLanguage::English => "failed to write file",
                InternalLanguage::Japanese => "ファイルの書き込みに失敗しました",
            },
            InternalTranslator::LogLimitExceeded { log_limit } => {
                InternalLanguage::English => format!("log limit {} exceeded", log_limit),
                InternalLanguage::Japanese => format!("ログ制限 {} を超過しました", log_limit),
            },
            InternalTranslator::PathDoesNotExist => {
                InternalLanguage::English => "path does not exist",
                InternalLanguage::Japanese => "パスが存在しません",
            },
            // note: descriptions
            InternalTranslator::NameDescription { name } => {
                InternalLanguage::English => format!("name:\n{}", name),
                InternalLanguage::Japanese => format!("名前:\n{}", name),
            },
            InternalTranslator::PathDescription { path } => {
                InternalLanguage::English => format!("path:\t{}", path),
                InternalLanguage::Japanese => format!("パス:\t{}", path),
            },
        };
    }
}

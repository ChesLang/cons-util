use crate::*;

use std::fmt::{Display, Formatter};

pub type ConsoleResult<T> = Result<T, ()>;

#[macro_export]
macro_rules! log {
    ($kind:ident, $title:expr $(, $desc:expr)*) => {
        {
            ConsoleLog {
                kind: ConsoleLogKind::$kind,
                title: Box::new($title),
                descs: vec![
                    $(Box::new($desc),)*
                ]
            }
        }
    };
}

#[macro_export]
macro_rules! translate {
    (translator => $log:expr, lang => $lang:expr, $($log_key:pat => {$($lang_key:pat => $value:expr,)*},)*) => {
        match $log {
            $(
                $log_key => {
                    match $lang {
                        $($lang_key => $value.to_string(),)+
                    }
                },
            )+
        }
    };
}

pub trait ConsoleLogger: Clone + PartialEq {
    fn get_log(&self) -> ConsoleLog;
}

#[derive(Clone, PartialEq)]
pub enum TranslationResult {
    Success(String),
    UnknownLanguage
}

pub trait ConsoleLogTranslator: Send {
    fn translate(&self, lang_name: &str) -> TranslationResult;
}

#[derive(Clone, PartialEq)]
pub enum ConsoleLogKind {
    Error,
    Warning,
    Note,
}

impl ConsoleLogKind {
    fn get_log_color_num(&self) -> usize {
        return match self {
            ConsoleLogKind::Error => 31,
            ConsoleLogKind::Warning => 33,
            ConsoleLogKind::Note => 34,
        };
    }

    fn get_log_kind_name(&self) -> String {
        let s = match self {
            ConsoleLogKind::Error => "err",
            ConsoleLogKind::Warning => "warn",
            ConsoleLogKind::Note => "note",
        };

        return s.to_string();
    }
}

pub struct ConsoleLog {
    pub kind: ConsoleLogKind,
    pub title: Box<dyn ConsoleLogTranslator>,
    pub descs: Vec<Box<dyn ConsoleLogTranslator>>,
}

#[derive(Clone, PartialEq)]
pub enum ConsoleLogLimit {
    NoLimit,
    Limited(usize),
}

impl Display for ConsoleLogLimit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ConsoleLogLimit::NoLimit => "[no limit]".to_string(),
            ConsoleLogLimit::Limited(limit_count) => limit_count.to_string(),
        };

        return write!(f, "{}", s);
    }
}

pub struct Console {
    lang: String,
    log_list: Vec<ConsoleLog>,
    log_limit: ConsoleLogLimit,
    pub ignore_logs: bool,
}

impl Console {
    pub fn new(lang: String, log_limit: ConsoleLogLimit) -> Console {
        return Console {
            lang: lang,
            log_list: Vec::new(),
            log_limit: log_limit,
            ignore_logs: false,
        };
    }

    pub fn append_log(&mut self, log: ConsoleLog) {
        if !self.ignore_logs {
            self.log_list.push(log);
        }
    }

    pub fn clear(&mut self) {
        self.log_list.clear();
    }

    pub fn pop_log(&mut self) {
        if self.log_list.len() > 0 {
            self.log_list.pop();
        }
    }

    pub fn print_all(&self) {
        // note: ログ数制限のチェック
        let limit_num = match &self.log_limit {
            ConsoleLogLimit::NoLimit => -1i32,
            ConsoleLogLimit::Limited(v) => *v as i32,
        };

        let mut log_count = 0;

        for each_log in &self.log_list {
            if limit_num != -1 && log_count + 1 > limit_num as i32 {
                self.print(&log!(Note, InternalTranslator::LogLimitExceeded { log_limit: self.log_limit.clone() }));
                break;
            }

            self.print(each_log);
            log_count += 1;
        }
    }

    pub fn print(&self, log: &ConsoleLog) {
        let title_color = log.kind.get_log_color_num();
        let kind_name = log.kind.get_log_kind_name();

        let title = match log.title.translate(&self.lang) {
            TranslationResult::Success(v) => v,
            TranslationResult::UnknownLanguage => {
                Console::print_unknown_language_log();
                println!();
                return;
            },
        };

        Console::print_title(title_color, kind_name, title);

        for each_desc_result in &log.descs {
            let each_desc = match each_desc_result.translate(&self.lang) {
                TranslationResult::Success(v) => v,
                TranslationResult::UnknownLanguage => {
                    Console::print_unknown_language_log();
                    println!();
                    return;
                },
            };

            println!("{}", each_desc);
        }

        println!();
    }

    fn print_unknown_language_log() {
        let err_log_kind = ConsoleLogKind::Error;
        Console::print_title(err_log_kind.get_log_color_num(), err_log_kind.get_log_kind_name(), "unknown language".to_string());
    }

    fn print_title(color: usize, kind: String, title: String) {
        println!("\x1b[{}m[{}]\x1b[m {}", color, kind, title);
    }
}

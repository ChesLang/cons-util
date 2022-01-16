use std::fmt::{Display, Formatter};

use crate::file::*;
use crate::langpack::*;

pub type ConsoleResult<T> = Result<T, ()>;

#[macro_export]
macro_rules! log {
    ($kind:ident, $title:expr) => {
        {
            let kind = ConsoleLogKind::$kind;
            let title = $title.to_string();
            let descs = Vec::<ConsoleLogDescription>::new();
            ConsoleLog::new(kind, title, descs)
        }
    };

    ($kind:ident, $title:expr, $($desc:expr), *) => {
        {
            let kind = ConsoleLogKind::$kind;
            let title = $title.to_string();
            let descs = vec![$($desc.to_string(),)*];
            ConsoleLog::new(kind, title, ConsoleLogDescription::to_vec(descs))
        }
    };
}

pub trait ConsoleLogger {
    fn get_log(&self) -> ConsoleLog;
}

pub enum ConsoleLogKind {
    Error,
    Warning,
    Notice,
}

impl ConsoleLogKind {
    fn get_log_color_num(&self) -> usize {
        return match self {
            ConsoleLogKind::Error => 31,
            ConsoleLogKind::Warning => 33,
            ConsoleLogKind::Notice => 34,
        };
    }

    fn get_log_kind_name(&self) -> String {
        let s = match self {
            ConsoleLogKind::Error => "err",
            ConsoleLogKind::Warning => "warn",
            ConsoleLogKind::Notice => "note",
        };

        return s.to_string();
    }
}

pub enum ConsoleLogDescription {
    Normal(String),
    Optional(String),
}

impl ConsoleLogDescription {
    pub fn reverse_kind(&self) -> ConsoleLogDescription {
        match self {
            ConsoleLogDescription::Normal(msg) => ConsoleLogDescription::Optional(msg.clone()),
            ConsoleLogDescription::Optional(msg) => ConsoleLogDescription::Normal(msg.clone()),
        }
    }

    pub fn to_vec(descs: Vec<String>) -> Vec<ConsoleLogDescription> {
        return descs.iter().map(|s| {
            if !s.starts_with("?") {
                ConsoleLogDescription::Normal(s.clone())
            } else {
                ConsoleLogDescription::Optional(s.clone())
            }
        }).collect::<Vec<ConsoleLogDescription>>();
    }
}

pub struct ConsoleLog {
    kind: ConsoleLogKind,
    title: String,
    descs: Vec<ConsoleLogDescription>,
}

impl ConsoleLog {
    pub fn new(kind: ConsoleLogKind, title: String, descs: Vec<ConsoleLogDescription>) -> ConsoleLog {
        return ConsoleLog {
            kind: kind,
            title: title,
            descs: descs,
        };
    }
}

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
    langpack: Langpack,
    log_list: Vec<ConsoleLog>,
    log_limit: ConsoleLogLimit,
}

impl Console {
    // note: lang_file_path が None であれば言語パックを読み込まない
    pub fn load(lang_file_path: Option<String>, log_limit: ConsoleLogLimit) -> Result<Console, FileError> {
        let cons = Console {
            langpack: match lang_file_path {
                Some(v) => Langpack::load(&v)?,
                None => Langpack::get_empty(),
            },
            log_list: Vec::new(),
            log_limit: log_limit,
        };

        return Ok(cons);
    }

    pub fn append_log(&mut self, log: ConsoleLog) {
        self.log_list.push(log);
    }

    pub fn get_terminate_msg() -> String {
        return "Program was aborted with error.".to_string();
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
                self.print(&log!(Notice, "{^console.note.4768}", format!("{{^console.log_limit}}: {}", self.log_limit)));
                break;
            }

            self.print(each_log);
            log_count += 1;
        }
    }

    pub fn print(&self, log: &ConsoleLog) {
        let title_color = log.kind.get_log_color_num();
        let kind_name = log.kind.get_log_kind_name();

        println!("\x1b[{}m[{}]\x1b[m {}", title_color, kind_name, self.langpack.translate(&log.title));

        for each_desc in &log.descs {
            match each_desc {
                ConsoleLogDescription::Normal(msg) => println!("\t{}", self.langpack.translate(msg)),
                _ => (),
            }
        }

        println!();
    }
}

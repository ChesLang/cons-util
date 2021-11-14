use crate::fileman::*;
use crate::langpack::*;

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

pub struct Console {
    langpack: Langpack,
    log_limit: i32,
    log_count: i32,
}

impl Console {
    // note: rel_langpack_path が None であれば言語パックを読み込まない
    pub fn load(rel_langpack_path: Option<String>) -> Result<Console, FileManError> {
        let cons = Console {
            langpack: match rel_langpack_path {
                Some(v) => Langpack::load(v)?,
                None => Langpack::get_empty(),
            },
            log_limit: 20,
            log_count: 0,
        };

        return Ok(cons);
    }

    pub fn get_terminate_msg() -> String {
        return "Program was aborted with error.".to_string();
    }

    pub fn log(&mut self, log: ConsoleLog, show_details: bool) {
        if self.log_limit != -1 {
            if self.log_limit < self.log_count {
                return;
            }

            if self.log_limit <= self.log_count {
                let tmp_log_limit = self.log_limit;
                self.log_limit = -1;
                self.log(log!(Notice, "{^console.note.4768}", format!("{{^console.log_limit}}: {}", tmp_log_limit)), false);
                self.log_limit = tmp_log_limit;
                return;
            }
        }

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

        self.log_count += 1;

        if show_details {
            let mut reverse_descs = Vec::<ConsoleLogDescription>::new();

            for each_desc in &log.descs {
                reverse_descs.push(each_desc.reverse_kind());
            }

            self.log(ConsoleLog::new(ConsoleLogKind::Notice, "{^cmd.note.5720}".to_string(), reverse_descs), false);
        }
    }
}

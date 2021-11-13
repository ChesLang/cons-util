use crate::fileman;

/* ConsoleLogKind */

pub enum ConsoleLogKind {
    Error,
    Warning,
    Notice,
}

/* ConsoleLogData */

pub struct ConsoleLogData {
    kind: ConsoleLogKind,
    title: String,
    desc: Vec<String>,
    details: Vec<String>,
}

impl ConsoleLogData {
    pub fn new(kind: ConsoleLogKind, title: &str, desc: Vec<String>, details: Vec<String>) -> Self {
        return ConsoleLogData {
            kind: kind,
            title: title.to_string(),
            desc: desc,
            details: details,
        };
    }
}

/* Console */

pub struct Console {
    lang_pack_props: std::collections::HashMap<String, String>,

    pub log_limit: i32,
    log_count: i32,
}

impl Console {
    pub fn new() -> Self {
        return Console {
            lang_pack_props: std::collections::HashMap::new(),
            log_limit: 20,
            log_count: 0,
        };
    }

    fn get_log_color(kind: &ConsoleLogKind) -> &'static str {
        return match kind {
            ConsoleLogKind::Error => "31",
            ConsoleLogKind::Warning => "33",
            ConsoleLogKind::Notice => "34",
        };
    }

    fn get_log_kind_name(kind: &ConsoleLogKind) -> String {
        return match kind {
            ConsoleLogKind::Error => "err".to_string(),
            ConsoleLogKind::Warning => "warn".to_string(),
            ConsoleLogKind::Notice => "note".to_string(),
        };
    }

    pub fn get_terminate_msg() -> &'static str {
        return "Program was aborted with error.";
    }

    fn get_translated_text(&self, text: &str) -> String {
        let regex = regex::Regex::new(r"\{\^[a-zA-Z0-9\._-]+\}").unwrap();
        let matched_iter = regex.find_iter(text);

        let mut translated_text = text.to_string();

        for matched in matched_iter {
            let matched_str = matched.as_str();
            let prop_name = &matched_str[2..matched_str.len() - 1];

            if !self.lang_pack_props.contains_key(prop_name) {
                continue;
            }

            translated_text = translated_text.replace(matched_str, &self.lang_pack_props[prop_name]);
        }

        return translated_text;
    }

    pub fn load_langpack(&mut self, rel_path: &str) -> std::result::Result<(), fileman::FileManError> {
        self.lang_pack_props = std::collections::HashMap::new();

        let path = fileman::FileMan::get_langpack_path(rel_path)?;
        let lines = fileman::FileMan::read_lines(&path)?;

        for mut each_line in lines {
            if each_line.contains(' ') {
                let tokens: Vec<&str> = each_line.split(' ').collect();
                let prop_name = tokens[0];
                let prop_name_len = prop_name.len();

                if prop_name_len == 0 {
                    continue;
                }

                self.lang_pack_props.insert(prop_name.to_string(), each_line.split_off(prop_name_len + 1).to_string());
            }
        }

        return Ok(());
    }

    pub fn log(&mut self, data: ConsoleLogData, show_details: bool) {
        if self.log_limit != -1 {
            if self.log_limit < self.log_count {
                return;
            }

            if self.log_limit <= self.log_count {
                let tmp_log_limit = self.log_limit;
                self.log_limit = -1;
                self.log(ConsoleLogData::new(ConsoleLogKind::Notice, "{^console.note.4768}", vec![format!("{{^console.log_limit}}: {}", tmp_log_limit)], vec![]), false);
                self.log_limit = tmp_log_limit;
                return;
            }
        }

        let title_color = Console::get_log_color(&data.kind);
        let kind_name = Console::get_log_kind_name(&data.kind);

        println!("\x1b[{}m[{}]\x1b[m {}", title_color, self.get_translated_text(&kind_name), self.get_translated_text(&data.title));

        for desc_line in &data.desc {
            println!("\t{}", self.get_translated_text(&desc_line));
        }

        println!();

        self.log_count += 1;

        if show_details {
            self.log(ConsoleLogData::new(ConsoleLogKind::Notice, "{^cmd.note.5720}", data.details, vec![]), false);
        }
    }
}

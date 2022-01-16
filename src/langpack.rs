use std::collections::*;

use crate::file::*;

pub struct Langpack {
    props: HashMap<String, String>,
}

impl Langpack {
    pub fn get_empty() -> Langpack {
        return Langpack {
            props: HashMap::new(),
        }
    }

    pub fn load(lang_file_path: &str) -> Result<Langpack, FileError> {
        let mut props = HashMap::<String, String>::new();

        let lines = FileMan::read_lines(lang_file_path)?;

        for mut each_line in lines {
            if each_line.contains(' ') {
                let tokens = each_line.split(' ').collect::<Vec<&str>>();
                let prop_name = tokens[0];
                let prop_name_len = prop_name.len();

                if prop_name_len == 0 {
                    continue;
                }

                props.insert(prop_name.to_string(), each_line.split_off(prop_name_len + 1));
            }
        }

        let langpack = Langpack {
            props: props,
        };

        return Ok(langpack);
    }

    pub fn translate(&self, text: &String) -> String {
        let regex = regex::Regex::new(r"\{\^[a-zA-Z0-9\._-]+\}").unwrap();
        let matched_iter = regex.find_iter(text);
        let mut translated_text = text.clone();

        for matched in matched_iter {
            let matched_str = matched.as_str();
            let prop_name = &matched_str[2..matched_str.len() - 1];

            match self.props.get(prop_name) {
                Some(v) => translated_text = translated_text.replace(matched_str, v),
                None => (),
            }
        }

        return translated_text;
    }
}

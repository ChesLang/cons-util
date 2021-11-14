use std::collections::*;

use crate::fileman::*;

pub struct Langpack {
    props: HashMap<String, String>,
}

impl Langpack {
    pub fn load(rel_path: &String) -> Result<Langpack, FileManError> {
        let mut props = HashMap::new();

        let path = FileMan::get_langpack_path(rel_path)?;
        let lines = FileMan::read_lines(&path)?;

        for mut each_line in lines {
            if each_line.contains(' ') {
                let tokens: Vec<&str> = each_line.split(' ').collect();
                let prop_name = tokens[0];
                let prop_name_len = prop_name.len();

                if prop_name_len == 0 {
                    continue;
                }

                props.insert(prop_name.to_string(), each_line.split_off(prop_name_len + 1).to_string());
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
        let mut translated_text = String::new();

        for matched in matched_iter {
            let matched_str = matched.as_str();
            let prop_name = &matched_str[2..matched_str.len() - 1];

            match self.props.get(prop_name) {
                Some(v) => translated_text = translated_text.replace(matched_str, &self.props[prop_name]),
                None => (),
            }
        }

        return translated_text;
    }
}

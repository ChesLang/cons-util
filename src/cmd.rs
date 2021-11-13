use crate::console;

pub type CommandMap = std::collections::HashMap<String, fn(String, std::collections::HashMap::<String, Vec<String>>, &mut console::Console)>;

pub fn run_command(default_cmd_name: &str, cmd_procs: CommandMap) {
    let mut cons = console::Console::new();

    match cons.load_langpack("/Ches_1/rustnut/compiler/1.0.0/lib/lang/en-us.lang") {
        Ok(v) => v,
        Err(e) => {
            cons.log(e.get_log_data(), false);
            return;
        }
    }

    let cmd_line_args: Vec<String> = std::env::args().collect();

    let cmd = match Command::get_cmd_data(&cmd_line_args, default_cmd_name) {
        Ok(v) => v,
        Err(e) => {
            for _i in 0..1 {
                cons.log(e.get_log_data(), false);
            }

            return;
        },
    };

    let show_details = cmd.subcmd_options.contains_key("-det");

    let log_limit_key = "-lim";

    if cmd.subcmd_options.contains_key(log_limit_key) {
        let log_limit_vec = &cmd.subcmd_options[log_limit_key];

        if log_limit_vec.len() != 1 {
            cons.log(console::ConsoleLogData::new(console::ConsoleLogKind::Error, "{^cmd.err.7095}", vec![], vec![]), show_details);
            return;
        }

        if log_limit_vec[0] == "no" {
            cons.log_limit = -1;
        } else {
            let log_limit_value: i32 = match log_limit_vec[0].parse() {
                Ok(v) => v,
                Err(_e) => {
                    cons.log(console::ConsoleLogData::new(console::ConsoleLogKind::Error, "{^cmd.err.7095}", vec![format!("{{^cmd.option_value}}: {}", log_limit_vec[0])], vec![]), show_details);
                    return;
                },
            };

            cons.log_limit = log_limit_value;
        }
    }

    match cmd.run(cmd_procs, &mut cons) {
        Ok(()) => (),
        Err(e) => {
            cons.log(e.get_log_data(), show_details);
            return;
        },
    };
}

/* CommandError */

#[derive(Debug)]
pub enum CommandError {
    DuplicatedOptionName(String),
    InternalLoadFailure(String),
    NoMatchingSubcmdName(String),
    OptionValueBeforeOptionName(String),
}

impl CommandError {
    pub fn get_log_data(&self) -> console::ConsoleLogData {
        match self {
            CommandError::DuplicatedOptionName(option_name) => console::ConsoleLogData::new(console::ConsoleLogKind::Error, "{^cmd.err.1943}", vec![format!("{{^cmd.option_name}}: {}", option_name)], vec![format!("{{^console.spec_link}}: https://ches.gant.work/en/spec/console/command/error/1943/index.html")]),
            CommandError::InternalLoadFailure(cause) => console::ConsoleLogData::new(console::ConsoleLogKind::Error, "{^cmd.err.6389}", vec![format!("{{^cmd.cause}}: {}", cause)], vec![format!("{{^console.spec_link}}: https://ches.gant.work/en/spec/console/command/error/6389/index.html")]),
            CommandError::NoMatchingSubcmdName(subcmd_name) => console::ConsoleLogData::new(console::ConsoleLogKind::Error, "{^cmd.err.3485}", vec![format!("{{^cmd.subcmd_name}}: {}", subcmd_name)], vec![format!("{{^console.spec_link}}: https://ches.gant.work/en/spec/console/command/error/3485/index.html")]),
            CommandError::OptionValueBeforeOptionName(option_value) => console::ConsoleLogData::new(console::ConsoleLogKind::Error, "{^cmd.err.9534}", vec![format!("{{^cmd.option_value}}: {}", option_value)], vec![format!("{{^console.spec_link}}: https://ches.gant.work/en/spec/console/command/error/9534/index.html")]),
        }
    }
}

impl std::error::Error for CommandError {}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "CommandError");
    }
}

/* Command */

struct Command {
    pub cmd_line_args: Vec<String>,

    pub subcmd_name: String,
    pub subcmd_options: std::collections::HashMap::<String, Vec::<String>>,

    pub console: console::Console,
}

impl Command {
    pub fn new(cmd_line_args: Vec<String>, subcmd_name: String, subcmd_options: std::collections::HashMap::<String, Vec::<String>>) -> Self {
        return Command {
            cmd_line_args: cmd_line_args,
            subcmd_name: subcmd_name,
            subcmd_options: subcmd_options,
            console: console::Console::new(),
        };
    }

    pub fn get_cmd_data(cmd_line_args: &Vec<String>, default_subcmd_name: &str) -> std::result::Result<Command, CommandError> {
        let mut subcmd_name = default_subcmd_name;
        let mut subcmd_options = std::collections::HashMap::<String, Vec::<String>>::new();
        let mut tmp_option_name = "";

        if cmd_line_args.len() == 1 {
            return Ok(Command::new(cmd_line_args.clone(), default_subcmd_name.to_string(), subcmd_options));
        }

        let mut arg_count_begin = 1;

        if !Command::is_cmd_arg_option(cmd_line_args[1].to_string()) {
            subcmd_name = &cmd_line_args[1];
            arg_count_begin = 2;
        }

        for i in arg_count_begin..cmd_line_args.len() {
            if Command::is_cmd_arg_option(cmd_line_args[i].to_string()) {
                tmp_option_name = &cmd_line_args[i];

                if subcmd_options.contains_key(tmp_option_name) {
                    return Err(CommandError::DuplicatedOptionName(tmp_option_name.to_string()))?;
                }

                subcmd_options.insert(tmp_option_name.to_string(), Vec::<String>::new());
            } else {
                if tmp_option_name == "" {
                    return Err(CommandError::OptionValueBeforeOptionName(cmd_line_args[i].to_string()))?;
                }

                let options_opt = subcmd_options.get(tmp_option_name);
                let mut options;

                match options_opt {
                    Some(v) => {
                        options = v.clone();
                    },
                    None => {
                        return Err(CommandError::InternalLoadFailure("IndexOutOfRange".to_string()))?;
                    }
                }

                options.push(cmd_line_args[i].to_string());
                subcmd_options.insert(tmp_option_name.to_string(), options);
            }
        }

        return Ok(Command::new(cmd_line_args.to_vec(), subcmd_name.to_string(), subcmd_options));
    }

    pub fn is_cmd_arg_option(arg: String) -> bool {
        return arg.starts_with('-');
    }

    pub fn run<F: Fn(String, std::collections::HashMap::<String, Vec<String>>, &mut console::Console)>(&self, cmd_procs: std::collections::HashMap<String, F>, cons: &mut console::Console) -> std::result::Result<(), CommandError> {
        if !cmd_procs.contains_key(&self.subcmd_name) {
            return Err(CommandError::NoMatchingSubcmdName(self.subcmd_name.to_string()))?;
        }

        cmd_procs[&self.subcmd_name](self.subcmd_name.to_string(), self.subcmd_options.clone(), cons);
        return Ok(());
    }
}

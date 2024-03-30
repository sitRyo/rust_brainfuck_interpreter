// コマンドラインの中身を取り出す

use std::env;

use colored::Colorize;

#[derive(Debug)]
pub struct OptionCommand {
    pub is_debug: bool,
    pub is_help: bool,
}

fn args_options(args: String, option_command: &mut OptionCommand) {
    if args == "-h" || args == "--help" {
        show_help();
        option_command.is_help = true;
    } else if args == "-d" || args == "--debug" {
        option_command.is_debug = true;
    } else {
        show_unrecognized_option_descriptions(args);
    }
}

fn show_help() {
    println!("{}", "Options".green());
    println!(" * -h --help => show all options.");
    println!(" * -d --debug => use REPL mode.");
}

fn show_unrecognized_option_descriptions(args: String) {
    println!("{} is unrecognized option.", args);
    println!("please enter help args");
}

pub fn parse_command_line_args() -> OptionCommand {
    let args: Vec<String> = env::args().collect();
    let iter = args.iter();
    let mut option_command = OptionCommand {
        is_debug: false,
        is_help: false,
    };

    let mut is_skiped_exe_file_str = false;
    for args in iter {
        if !is_skiped_exe_file_str {
            is_skiped_exe_file_str = true;
            continue;
        }
        args_options(args.to_string(), &mut option_command);
    }

    return option_command;
}

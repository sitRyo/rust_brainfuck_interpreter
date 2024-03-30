mod command_line;
mod console;
mod debuger;
mod parser;
mod stack;
mod token;

use console::{Console, InputData};
use parser::{BFMemory, Parser};

fn construct_input_data() -> InputData {
    let input_data = InputData {
        code: String::new(),
    };

    return input_data;
}

fn main() {
    // コマンドラインオプション
    let command = command_line::parse_command_line_args();

    if command.is_help {
        return;
    }

    // コードをインプット
    let mut input_data = construct_input_data();
    input_data.input_code();
    // input_data.debug_show();

    // パーサーの作成
    let mut parser = BFMemory::new(input_data, command);
    parser.parse();
}

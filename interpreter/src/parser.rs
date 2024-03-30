use std::io;

use crate::console::InputData;
use crate::stack;
use crate::stack::Manipulation;
use crate::token;
use crate::{command_line, debuger};

// Brainfuck 実行中に管理するメモリ位置やコード本体
pub struct BFMemory {
    // 現在のカーソルの位置
    pub code_cursor: u32,
    input_data: InputData,

    runtime_memory: Vec<i32>,
    runtime_memory_cursor: u32,

    left_bracket_stack: stack::Stack<u32>,
    right_bracket_stack: stack::Stack<u32>,

    is_debug_mode: bool,
}

pub trait Parser {
    fn new(input_data: InputData, command_option: command_line::OptionCommand) -> Self;
    fn parse(&mut self);
}

impl Parser for BFMemory {
    fn new(input_data: InputData, command_option: command_line::OptionCommand) -> BFMemory {
        BFMemory {
            code_cursor: 0,
            input_data: input_data,
            runtime_memory: Vec::new(),
            runtime_memory_cursor: 0,
            left_bracket_stack: stack::Stack::<u32>::new(),
            right_bracket_stack: stack::Stack::<u32>::new(),
            is_debug_mode: command_option.is_debug,
        }
    }

    fn parse(&mut self) {
        let chars = self.input_data.code.chars();
        let code_length = chars.count() as u32;

        self.initialize_parser();

        while self.code_cursor < code_length {
            let letter = self.read_one_letter();
            self.analyze(letter);

            if self.is_debug_mode && self.code_cursor < code_length {
                debuger::show_debug_info(
                    self.code_cursor,
                    &self.input_data.code,
                    &self.runtime_memory,
                    self.runtime_memory_cursor,
                );
                debuger::wait_until_read_line();
            }
        }
    }
}

impl BFMemory {
    fn initialize_parser(&self) {
        if self.is_debug_mode {
            // 初期対応
            debuger::show_debug_info(
                self.code_cursor,
                &self.input_data.code,
                &self.runtime_memory,
                self.runtime_memory_cursor,
            );
        }
    }

    // code から1文字読む
    fn read_one_letter(&mut self) -> char {
        // unwrap するのはあんまりよくない
        let letter = self
            .input_data
            .code
            .chars()
            .nth(self.code_cursor as usize)
            .unwrap();
        self.code_cursor += 1;
        letter
    }

    // 特定の位置の文字を読み込む
    fn read_nth_letter(&self, cursor: usize) -> char {
        let letter = self.input_data.code.chars().nth(cursor).unwrap();
        letter
    }

    // 読んだ文字を解析する
    fn analyze(&mut self, letter: char) {
        self.check_runtime_memory();
        let token = token::letter_to_token(letter);
        match token {
            token::Token::Plus => self.proc_plus_token(),
            token::Token::Minus => self.proc_minus_token(),
            token::Token::Grater => self.proc_grater_token(),
            token::Token::Lesser => self.proc_lesser_token(),
            token::Token::Period => self.proc_period_token(),
            token::Token::Comma => self.proc_comma_token(),
            token::Token::LeftBracket => self.proc_left_bracket_token(),
            token::Token::RightBracket => self.proc_right_bracket_token(),
            token::Token::TERM => self.proc_term(),
        }
    }

    // 現在のメモリをインクリメントする
    fn proc_plus_token(&mut self) {
        self.runtime_memory[self.runtime_memory_cursor as usize] += 1;
    }

    // 現在のメモリをデクリメントする
    fn proc_minus_token(&mut self) {
        self.runtime_memory[self.runtime_memory_cursor as usize] -= 1;
    }

    // 命令ポインタを1つ右に移動させる
    fn proc_grater_token(&mut self) {
        self.runtime_memory_cursor += 1;
    }

    // 命令ポインタを1つ左に移動させる
    fn proc_lesser_token(&mut self) {
        self.runtime_memory_cursor -= 1;
    }

    // 現在のメモリの値を文字コードとみなし出力する
    fn proc_period_token(&self) {
        // ASCII として出力する
        let val = self.runtime_memory[self.runtime_memory_cursor as usize] as u32;
        let ch = std::char::from_u32(val).unwrap();
        print!("{}", ch);
    }

    // 入力を求め、入力された値を現在のメモリに代入する
    fn proc_comma_token(&mut self) {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line");
        // 改行だけだと上手く動かないはず
        let buf = String::from(buf.trim()).chars().nth(0).unwrap();
        let val = buf as u32;
        self.runtime_memory[self.runtime_memory_cursor as usize] = val as i32;
    }

    // 現在のメモリの値が0なら、対応する「]」にジャンプする
    fn proc_left_bracket_token(&mut self) {
        self.left_bracket_stack.push(self.code_cursor - 1);

        let value = self.get_current_cursor_memory_value();
        if value == 0 {
            let right_bracket_pos = if self.right_bracket_stack.is_empty() {
                self.search_match_right_bracket()
            } else {
                self.right_bracket_stack.pop()
            };
            self.code_cursor = right_bracket_pos;
        }
    }

    fn search_match_right_bracket(&self) -> u32 {
        let mut l_brackets = stack::Stack::<u32>::new();

        let bracket_idx = self.code_cursor;
        l_brackets.push(bracket_idx);

        let mut idx = bracket_idx + 1;
        while idx < self.input_data.code.len() as u32 {
            let letter = self.read_nth_letter(idx as usize);
            let token = token::letter_to_token(letter);
            match token {
                token::Token::LeftBracket => {
                    l_brackets.push(idx);
                }
                token::Token::RightBracket => {
                    l_brackets.pop();
                }
                _ => {}
            }

            if l_brackets.is_empty() {
                break;
            }

            idx += 1;
        }

        return idx;
    }

    // 現在のメモリの値が0でないなら、対応する「[」にジャンプする
    fn proc_right_bracket_token(&mut self) {
        self.right_bracket_stack.push(self.code_cursor - 1);

        let value = self.get_current_cursor_memory_value();
        if value == 0 {
            return;
        }

        // 対応する左カッコを探す
        let stack_len = self.right_bracket_stack.size();
        self.right_bracket_stack.clear();

        let mut idx = 0;
        let mut pos = 0;
        while idx < stack_len {
            idx += 1;
            pos = self.left_bracket_stack.pop();
        }
        self.code_cursor = pos;
    }

    // 不正なトークンが入力された
    // 文字の位置を指定してエラー出力出して終わらせる
    fn proc_term(&self) {
        panic!(
            "Error! Cannot be used as {} character detected.",
            self.runtime_memory_cursor - 1
        );
    }

    // メモリのサイズをチェックする。サイズが足りなかったら増やす
    fn check_runtime_memory(&mut self) {
        if self.runtime_memory_cursor >= self.runtime_memory.len() as u32 {
            self.runtime_memory.push(0);
        }
    }

    fn get_current_cursor_memory_value(&self) -> i32 {
        self.runtime_memory[self.runtime_memory_cursor as usize]
    }
}

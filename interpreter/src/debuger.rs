// メモリの状況を知りたい

use std::io;

pub fn wait_until_read_line() {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");
}

pub fn show_debug_info(
    code_cursor: u32,
    code: &String,
    runtime_memory: &Vec<i32>,
    runtime_memory_cursor: u32,
) {
    show_partition();
    show_code(code);
    show_code_cursor_position(code_cursor);
    show_runtime_memory(&runtime_memory, runtime_memory_cursor);
}

fn show_partition() {
    println!("---------------------------------------------------");
}

fn show_code(code: &String) {
    println!("{}", code);
}

fn show_code_cursor_position(cursor: u32) {
    let mut idx = 0;
    while idx < cursor {
        print!(" ");
        idx += 1;
    }
    println!("^");
}

fn show_runtime_memory(runtime_memory: &Vec<i32>, cursor: u32) {
    let len = runtime_memory.len();
    let mut idx = 0;
    while idx < len {
        print!("{}", runtime_memory[idx]);
        if idx != len - 1 {
            print!("|")
        }
        idx += 1;
    }
    println!("");

    println!("runtime memory cursor: {}", cursor);
}

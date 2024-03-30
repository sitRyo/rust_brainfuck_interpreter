use std::io;

pub struct InputData {
    pub code: String,
}

pub trait Console {
    fn input_code(&mut self);
    fn debug_show(&self);
}

impl Console for InputData {
    fn input_code(&mut self) {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line");
        buf = String::from(buf.trim());
        self.code = buf;
    }

    fn debug_show(&self) {
        println!("{}", self.code);
    }
}

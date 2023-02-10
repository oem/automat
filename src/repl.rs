use std::io::{stdin, stdout, Write};

use crate::scanner::Scanner;

pub fn run() {
    let mut stdout = stdout();

    loop {
        print!("  ");
        stdout.flush().expect("Unable to flush stdout");
        let mut buffer = String::new();
        let stdin = stdin();
        stdin
            .read_line(&mut buffer)
            .expect("Unable to read from stdin");
        let mut scanner = Scanner::new(buffer.trim().chars().collect());
        let scanned = scanner.scan();
        println!("{:?}", scanned);

        if buffer.len() == 0 {
            return;
        }
    }
}

use std::io::{stdin, stdout, Write};

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
        print!("{buffer}");

        if buffer.len() == 0 {
            return;
        }
    }
}

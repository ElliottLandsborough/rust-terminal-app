use rustyline::error::ReadlineError;
use rustyline::{Editor};

fn main() {
    // `()` can be used when no completer is required
    let mut rl = match Editor::<()>::new() {
        Ok(rl) => rl,
        Err(error) => panic!("Problem with readline: {:?}", error),
    };

    println!("Please input a github username below:");

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let user_name = line.as_str().trim();

                println!("Line: {}", user_name);
                break
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}

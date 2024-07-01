use std::fs;
use std::fs::File;
use std::io::{stdin, stdout, Write};

fn main(){
    loop {
        println!("1 - turn file to .cyce\n2 - turn .cyce file to .txt");
        print!(">");
        let _ = stdout().flush();

        let mut choice = String::new();
        stdin().read_line(&mut choice).expect("couldn't take user input");
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("enter name of your file");
                print!(">");
                let _ = stdout().flush();

                let mut filename = String::new();
                stdin().read_line(&mut filename).expect("couldn't take user input");
                let filename = filename.trim();

                match fs::read_to_string(filename) {
                    Ok(_) => {
                        let _ = File::create("cyphered.cyce");
                        let mut cyphered_contains = String::new();

                        match fs::read_to_string(filename) {
                            Ok(contains) => {
                                for i in contains.chars() {
                                    match i {
                                        'a' => cyphered_contains.push_str("g"),
                                        'A' => cyphered_contains.push_str("G"),
                                        'b' => cyphered_contains.push_str("q"),
                                        'B' => cyphered_contains.push_str("Q"),
                                        'c' => cyphered_contains.push_str("w"),
                                        'C' => cyphered_contains.push_str("W"),
                                        'd' => cyphered_contains.push_str("k"),
                                        'D' => cyphered_contains.push_str("K"),
                                        'e' => cyphered_contains.push_str("t"),
                                        'E' => cyphered_contains.push_str("T"),
                                        'f' => cyphered_contains.push_str("o"),
                                        'F' => cyphered_contains.push_str("O"),
                                        'g' => cyphered_contains.push_str("a"),
                                        'G' => cyphered_contains.push_str("A"),
                                        'h' => cyphered_contains.push_str("v"),
                                        'H' => cyphered_contains.push_str("V"),
                                        'i' => cyphered_contains.push_str("p"),
                                        'I' => cyphered_contains.push_str("P"),
                                        'j' => cyphered_contains.push_str("r"),
                                        'J' => cyphered_contains.push_str("R"),
                                        'k' => cyphered_contains.push_str("d"),
                                        'K' => cyphered_contains.push_str("D"),
                                        'l' => cyphered_contains.push_str("n"),
                                        'L' => cyphered_contains.push_str("N"),
                                        'm' => cyphered_contains.push_str("s"),
                                        'M' => cyphered_contains.push_str("S"),
                                        'n' => cyphered_contains.push_str("l"),
                                        'N' => cyphered_contains.push_str("L"),
                                        'o' => cyphered_contains.push_str("f"),
                                        'O' => cyphered_contains.push_str("F"),
                                        'p' => cyphered_contains.push_str("i"),
                                        'P' => cyphered_contains.push_str("I"),
                                        'q' => cyphered_contains.push_str("b"),
                                        'Q' => cyphered_contains.push_str("B"),
                                        'r' => cyphered_contains.push_str("j"),
                                        'R' => cyphered_contains.push_str("J"),
                                        's' => cyphered_contains.push_str("m"),
                                        'S' => cyphered_contains.push_str("M"),
                                        't' => cyphered_contains.push_str("e"),
                                        'T' => cyphered_contains.push_str("E"),
                                        'u' => cyphered_contains.push_str("y"),
                                        'U' => cyphered_contains.push_str("Y"),
                                        'v' => cyphered_contains.push_str("h"),
                                        'V' => cyphered_contains.push_str("H"),
                                        'w' => cyphered_contains.push_str("c"),
                                        'W' => cyphered_contains.push_str("C"),
                                        'x' => cyphered_contains.push_str("z"),
                                        'X' => cyphered_contains.push_str("Z"),
                                        'y' => cyphered_contains.push_str("u"),
                                        'Y' => cyphered_contains.push_str("U"),
                                        'z' => cyphered_contains.push_str("x"),
                                        'Z' => cyphered_contains.push_str("X"),
                                        _ => cyphered_contains.push_str(&i.to_string()),
                                    }
                                }
                                let _ = fs::write("cyphered.cyce", cyphered_contains);
                            }
                            Err(error) => println!("error {}", error),
                        }
                    },
                    Err(error) => println!("error: {}", error),
                }
            },
            "2" => {
                println!("enter file you want to decrypt");
                print!(">");
                let _ = stdout().flush();

                let mut filename = String::new();
                stdin().read_line(&mut filename).expect("couldn't take user input");
                let filename = filename.trim();

                match fs::read_to_string(filename) {
                    Ok(contents) => {
                        let _ = File::create("decrypted.txt");
                        let mut decrypted_message = String::new();
                        for i in contents.chars() {
                            match i {
                                'A' => decrypted_message.push_str("G"),
                                'a' => decrypted_message.push_str("g"),
                                'B' => decrypted_message.push_str("Q"),
                                'b' => decrypted_message.push_str("q"),
                                'c' => decrypted_message.push_str("w"),
                                'C' => decrypted_message.push_str("W"),
                                'd' => decrypted_message.push_str("k"),
                                'D' => decrypted_message.push_str("K"),
                                'e' => decrypted_message.push_str("t"),
                                'E' => decrypted_message.push_str("T"),
                                'f' => decrypted_message.push_str("o"),
                                'F' => decrypted_message.push_str("O"),
                                'g' => decrypted_message.push_str("a"),
                                'G' => decrypted_message.push_str("A"),
                                'h' => decrypted_message.push_str("v"),
                                'H' => decrypted_message.push_str("V"),
                                'i' => decrypted_message.push_str("p"),
                                'I' => decrypted_message.push_str("P"),
                                'j' => decrypted_message.push_str("r"),
                                'J' => decrypted_message.push_str("R"),
                                'k' => decrypted_message.push_str("d"),
                                'K' => decrypted_message.push_str("D"),
                                'l' => decrypted_message.push_str("n"),
                                'L' => decrypted_message.push_str("N"),
                                'm' => decrypted_message.push_str("s"),
                                'M' => decrypted_message.push_str("S"),
                                'n' => decrypted_message.push_str("l"),
                                'N' => decrypted_message.push_str("L"),
                                'o' => decrypted_message.push_str("f"),
                                'O' => decrypted_message.push_str("F"),
                                'p' => decrypted_message.push_str("i"),
                                'P' => decrypted_message.push_str("I"),
                                'q' => decrypted_message.push_str("b"),
                                'Q' => decrypted_message.push_str("B"),
                                'r' => decrypted_message.push_str("j"),
                                'R' => decrypted_message.push_str("J"),
                                's' => decrypted_message.push_str("m"),
                                'S' => decrypted_message.push_str("M"),
                                't' => decrypted_message.push_str("e"),
                                'T' => decrypted_message.push_str("E"),
                                'u' => decrypted_message.push_str("y"),
                                'U' => decrypted_message.push_str("Y"),
                                'v' => decrypted_message.push_str("h"),
                                'V' => decrypted_message.push_str("H"),
                                'w' => decrypted_message.push_str("c"),
                                'W' => decrypted_message.push_str("C"),
                                'x' => decrypted_message.push_str("z"),
                                'X' => decrypted_message.push_str("Z"),
                                'y' => decrypted_message.push_str("u"),
                                'Y' => decrypted_message.push_str("U"),
                                'z' => decrypted_message.push_str("x"),
                                'Z' => decrypted_message.push_str("X"),
                                _ => decrypted_message.push_str(&i.to_string()),
                            }
                        }
                        let _ = fs::write("decrypted.txt", decrypted_message);
                    },
                    Err(error) => println!("error: {}", error),
                }
            },
            _ => println!("invalid option!"),
        }
    }
}
use std::io::{self, Write, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::env;

const BUFFER_CAPACITY: usize = 1024 * 32;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut show_lines = false;
    let mut show_endings = false;

    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }
        else if arg == "-n" {
            show_lines = true;
        } else if arg == "-E" {
            show_endings = true;
        } else {
            cat(arg, show_lines, show_endings).unwrap();
        }
    }

    Ok(())
}

fn cat(filename : &String, show_lines: bool, show_endings: bool) -> io::Result<()> {
    let input = File::open(filename)?;
    let mut input = BufReader::new(input);
   
    let stdout = io::stdout();
    let mut handle = stdout.lock(); 

    let mut buf = [0; BUFFER_CAPACITY];
    let mut vec = Vec::new();
    let mut linenums: u32 = 1;

    loop {
        match input.read(&mut buf) {
            Ok(n) => {
                if n != 0 {
                    let mut last = 0;
                    for i in 0..buf.len() {
                        if buf[i] == b'\n' {
                            if show_lines {
                                vec.extend(&[b' '; 4]);
                                vec.extend(linenums.to_string().as_bytes());
                                vec.push(b' ');
                            }

                            vec.extend(&buf[last..i]);
                            if show_endings {
                                vec.push(b'$');
                            }
                            vec.push(buf[i]);
                            last = i+1;
                            linenums += 1;
                        } else if buf[i] == b'\0' {
                            vec.extend(&buf[last..i]);
                            last = i;
                        }
                    }

                    handle.write_all(&vec).unwrap();
                    buf = [0; BUFFER_CAPACITY];
                    vec.clear();
                }
                else {
                    handle.flush()?;
                    break;
                }
            }
            Err(error) => panic!(error),
        }
    }

    Ok(())
}

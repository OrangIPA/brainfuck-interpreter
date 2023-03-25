use std::{env, fs, process, io::stdin};

const OPENING_BRACKET_U8: u8 = '[' as u8;
const CLOSING_BRACKET_U8: u8 = ']' as u8;
const ADD_U8: u8 = '+' as u8;
const MIN_U8: u8 = '-' as u8;
const LEFT_U8: u8 = '<' as u8;
const RIGHT_U8: u8 = '>' as u8;
const DOT_U8: u8 = '.' as u8;
const COM_U8: u8 = ',' as u8;

struct Tape {
    cells: [u8; 10000],
    ptr: usize
}

impl Tape {
    fn left(&mut self) {
        match self.ptr {
            0  => panic!("out of bounds!"),
            _ => self.ptr -= 1,
        }
    }

    fn right(&mut self) {
        match self.ptr {
            10000 => panic!("out of bounds!"),
            _ => self.ptr += 1
        }
    }

    fn inc(&mut self, val: u8) {
        self.cells[self.ptr] += val;
    }

    fn dec(&mut self, val: u8) {
        self.cells[self.ptr] -= val;
    }

    fn lp(&mut self) {
        // Escape the loop if current cell have the value 0
        if self.cells[self.ptr] == 0 {
            self.inc(1)
        }

        // Track how much nested loop the 'local pointer' have been through
        let mut count = 0usize;

        // Iterate for each cell from current pointer to address zero
        for _ in self.ptr..0 {
            let local_ptr = self.ptr - count;
            let local_val = self.cells[local_ptr];
            match (count, local_val) {
                (0, CLOSING_BRACKET_U8) => {
                    self.ptr += 1;
                    break;
                },
                (0, OPENING_BRACKET_U8) => {
                    self.ptr = local_ptr;
                },
                (_, CLOSING_BRACKET_U8) => {
                    count += 1;
                },
                (_, OPENING_BRACKET_U8) => {
                    count -= 1;
                },
                (_, _)
                    if local_ptr == 0 && count != 1 => panic!("fix your brackets!"),
                (_, _) => (),

            }
        }
    }

    fn write(&mut self, val: u8) {
        self.cells[self.ptr] = val;
    }

    fn read(&self) -> u8 {
        self.cells[self.ptr]
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.get(1) {
        None => {
            println!("Usage: brainfuckinterpreter <input>");
            return;
        }
        Some(v) => v.to_owned(),
    };

    let instruction = match fs::read(filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1)
        }
    };

    let mut tape = Tape {
        cells: [0;10000],
        ptr: 0,
        
    };

    for c in instruction {
        match c {
            ADD_U8 => tape.inc(1),
            MIN_U8 => tape.dec(1),
            LEFT_U8 => tape.left(),
            RIGHT_U8 => tape.right(),
            DOT_U8 => print!("{}", tape.read()),
            COM_U8 => {
                let mut val: String = String::new();
                stdin().read_line(&mut val).unwrap();
                if val.len() == 1 {
                    tape.write(val.chars().collect::<Vec<char>>().get(0).unwrap().to_owned() as u8);
                }
            },
            CLOSING_BRACKET_U8 => tape.lp(),
            _ => ()
        }
    }
}

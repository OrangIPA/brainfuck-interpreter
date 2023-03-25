use std::{env, fs, io::stdin, process};

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
    ptr: usize,
    inst: Vec<char>,
    inst_ptr: usize,
}

impl Tape {
    fn left(&mut self) {
        match self.ptr {
            0 => panic!("out of bounds!"),
            _ => self.ptr -= 1,
        }
    }

    fn right(&mut self) {
        match self.ptr {
            10000 => panic!("out of bounds!"),
            _ => self.ptr += 1,
        }
    }

    fn inc(&mut self, val: u8) {
        self.cells[self.ptr] += val;
    }

    fn dec(&mut self, val: u8) {
        self.cells[self.ptr] -= val;
    }

    fn continue_loop(&mut self) {
        let mut count = 0;
        for i in (0..self.inst_ptr).rev() {
            let inst_char = self.inst.get(i).unwrap().to_owned() as u8;
            match (count, inst_char) {
                (0, OPENING_BRACKET_U8) => {
                    self.inst_ptr = i;
                    return;
                },
                (_, CLOSING_BRACKET_U8) => count += 1,
                (_, OPENING_BRACKET_U8) => count -= 1,
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

    let instruction_chars = instruction
        .clone()
        .into_iter()
        .map(|v| v as char)
        .collect::<Vec<char>>();

    let mut tape = Tape {
        cells: [0; 10000],
        ptr: 0,
        inst: instruction_chars,
        inst_ptr: 0,
    };

    while tape.inst_ptr < tape.inst.len() {
        let c = tape.inst.get(tape.inst_ptr).unwrap().to_owned() as u8;
        match c {
            ADD_U8 => tape.inc(1),
            MIN_U8 => tape.dec(1),
            LEFT_U8 => tape.left(),
            RIGHT_U8 => tape.right(),
            DOT_U8 => print!("{}", tape.read() as char),
            COM_U8 => {
                let mut val: String = String::new();
                stdin().read_line(&mut val).unwrap();
                if val.len() == 1 {
                    tape.write(
                        val.chars()
                            .collect::<Vec<char>>()
                            .get(0)
                            .unwrap()
                            .to_owned() as u8,
                    );
                }
            }
            CLOSING_BRACKET_U8 => {
                if tape.cells[tape.ptr] != 0 {
                    tape.continue_loop()
                }
            }
            _ => (),
        }
        tape.inst_ptr += 1;
    }
}

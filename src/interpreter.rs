use crate::tokenizer::Token;
use std::{
    collections::VecDeque,
    io::{self},
};

/// Defines what the Interpreter does with the current cell on an EOF
#[derive(Debug, Default)]
pub enum EOFConfig {
    #[default]
    Zero,
    MinusOne,
    Unchanged,
}

#[derive(Debug)]
pub struct Simulator {
    pub pointer: i128,
    pub right_dir: Vec<i128>,
    pub left_dir: Vec<i128>,

    pub program_counter: usize,
    pub tokens: Vec<Token>,

    pub io_buffer: VecDeque<char>,

    pub eof_config: EOFConfig,
}

impl Simulator {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            pointer: 0,
            right_dir: vec![0],
            left_dir: vec![0],
            program_counter: 0,
            tokens,
            io_buffer: VecDeque::new(),
            eof_config: EOFConfig::default(),
        }
    }

    pub fn simulate(&mut self) {
        while self.program_counter < self.tokens.len() {
            let instruction = self.next_instruction();
            match instruction {
                Some(Token::MoveRight) => self.move_right(),
                Some(Token::MoveLeft) => self.move_left(),
                Some(Token::Increment) => self.increment(),
                Some(Token::Decrement) => self.decrement(),
                Some(Token::Output) => self.output(),
                Some(Token::Input) => self.input(),
                Some(Token::LoopStart) => self.loop_start(),
                Some(Token::LoopEnd) => self.loop_end(),
                _ => {}
            }
        }
    }

    pub fn move_right(&mut self) {
        self.pointer += 1;
        if self.pointer >= self.right_dir.len() as i128 {
            let new_length = self.right_dir.len() * 2;
            self.right_dir.resize(new_length, 0);
        }
    }

    pub fn move_left(&mut self) {
        self.pointer -= 1;
        if -self.pointer > self.left_dir.len() as i128 {
            let new_length = self.left_dir.len() * 2;
            self.left_dir.resize(new_length, 0);
        }
    }

    pub fn increment(&mut self) {
        self.write_cell(self.read_cell() + 1);
    }

    pub fn decrement(&mut self) {
        self.write_cell(self.read_cell() - 1);
    }

    pub fn output(&mut self) {
        match char::from_u32(self.read_cell() as u32) {
            Some(v) => print!("{}", v),
            None => print!("�"),
        }
    }

    pub fn input(&mut self) {
        let mut buffer = String::new();
        if self.io_buffer.is_empty() && io::stdin().read_line(&mut buffer).is_ok() {
            buffer.chars().for_each(|e| self.io_buffer.push_back(e));
        }
        if let Some(e) = self.io_buffer.pop_front() {
            self.write_cell(e as i128);
        } else {
            match self.eof_config {
                EOFConfig::Zero => self.write_cell(0),
                EOFConfig::MinusOne => self.write_cell(-1),
                EOFConfig::Unchanged => {}
            }
        }
    }

    pub fn loop_start(&mut self) {
        if self.read_cell() != 0 {
            return;
        };
        let mut loop_start_counter = 0;
        while let Some(current_instruction) = self.next_instruction() {
            match current_instruction {
                Token::LoopStart => loop_start_counter += 1,
                Token::LoopEnd => {
                    if loop_start_counter <= 0 {
                        // found matching LoopEnd
                        break;
                    } else {
                        loop_start_counter -= 1;
                    }
                }
                _ => {}
            }
        }
    }

    pub fn loop_end(&mut self) {
        if self.read_cell() == 0 {
            return;
        };
        let mut loop_end_counter = 0;
        self.program_counter -= 2; // go back 2
        while let Some(current_instruction) = self.next_instruction_decrement() {
            match current_instruction {
                Token::LoopEnd => loop_end_counter += 1,
                Token::LoopStart => {
                    if loop_end_counter <= 0 {
                        // found matching LoopStart
                        self.program_counter += 1;
                        break;
                    } else {
                        loop_end_counter -= 1;
                    }
                }
                _ => {}
            }
        }
    }

    fn write_cell(&mut self, value: i128) {
        if self.pointer >= 0 {
            self.right_dir[self.pointer as usize] = value;
        } else {
            let idx = -(self.pointer + 1);
            self.left_dir[idx as usize] = value;
        }
    }

    fn read_cell(&self) -> i128 {
        if self.pointer >= 0 {
            self.right_dir[self.pointer as usize]
        } else {
            let idx = -(self.pointer + 1);
            self.left_dir[idx as usize]
        }
    }

    fn next_instruction(&mut self) -> Option<&Token> {
        let instruction = self.tokens.get(self.program_counter);
        self.program_counter += 1;
        instruction
    }

    fn next_instruction_decrement(&mut self) -> Option<&Token> {
        let instruction = self.tokens.get(self.program_counter);
        if self.program_counter > 0 {
            self.program_counter -= 1;
        }
        instruction
    }
}

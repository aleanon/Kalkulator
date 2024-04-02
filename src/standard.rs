use std::ops::{Add, Div, Mul, Sub};

use egui::TextBuffer;

const ADD: &str = " + ";
const SUBTRACT: &str = " - ";
const MULTIPLY: &str = " x ";
const DIVIDE: &str = " / ";
const NONE: &str = "";

#[derive(PartialEq, Debug)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    None,
}

#[derive(PartialEq)]
enum State {
    Clear,
    NewInput,
    Equalized,
    Calculated,
    Squared,
}

struct Cache {
    pub previous_result: String,
    pub operator: &'static str,
    pub operation_value: String,
    pub ending_sign: &'static str,
}

impl Cache {
    fn new() -> Self {
        Self {
            previous_result: String::new(),
            operator: NONE,
            operation_value: String::new(),
            ending_sign: "",
        }
    }

    fn clear(&mut self) {
        self.previous_result.clear();
        self.operator = NONE;
        self.operation_value.clear();
        self.ending_sign = "";
    }

    fn change_operator(&mut self, operation: &Operation) {
        match operation {
            Operation::Add => self.operator = ADD,
            Operation::Subtract => self.operator = SUBTRACT,
            Operation::Multiply => self.operator = MULTIPLY,
            Operation::Divide => self.operator = DIVIDE,
            Operation::None => self.operator = NONE,
        }
    }
}

#[derive(PartialEq)]
enum BufferState {
    Clear,
    ClearOnInput,
}

struct Buffer {
    pub state: BufferState,
    pub value: String,
    pub len: u8,
    pub contains_comma: bool,
}

impl Buffer {
    fn new() -> Self {
        let mut value = String::with_capacity(16);
        value.push('0');

        Self {
            state: BufferState::Clear,
            value,
            len: 1,
            contains_comma: false,
        }
    }

    fn clear(&mut self) {
        self.state = BufferState::Clear;
        self.value.replace_with("0");
        self.len = 1;
        self.contains_comma = false;
    }
}

pub struct Standard {
    state: State,

    buffer: Buffer,

    cache: Cache,

    operation: Operation,

    memory: Option<Vec<String>>,

    log: Option<Vec<String>>,
}

impl Standard {
    pub fn new() -> Self {
        Self {
            state: State::Clear,

            buffer: Buffer::new(),

            cache: Cache::new(),

            operation: Operation::None,

            memory: None,

            log: None,
        }
    }

    pub fn read_cache(&self) -> [&str; 4] {
        [
            &self.cache.previous_result,
            &self.cache.operator,
            &self.cache.operation_value,
            &self.cache.ending_sign,
        ]
    }

    pub fn read_buffer(&self) -> &str {
        &self.buffer.value
    }

    pub fn clear(&mut self) {
        self.cache.clear();
        self.buffer.clear();
        self.operation = Operation::None;
        self.state = State::Clear;
    }

    pub fn input_char(&mut self, char: char) {
        if self.buffer.state == BufferState::ClearOnInput {
            self.buffer.clear();
            self.state = State::NewInput
        }

        if self.buffer.len < 16 {
            match char {
                '0'..='9' => {
                    if self.buffer.value.ends_with('0') && self.buffer.len == 1 {
                        self.buffer.value.clear();
                        self.buffer.value.push(char);
                    } else {
                        self.buffer.value.push(char);
                        self.buffer.len += 1;
                    }
                }
                ',' => {
                    if !self.buffer.contains_comma {
                        self.buffer.value.push(char);
                        self.buffer.len += 1;
                        self.buffer.contains_comma = true;
                    }
                }
                _ => { /*Do nothing */ }
            }
        }
    }

    pub fn backspace(&mut self) {
        match self.state {
            State::Equalized => self.cache.clear(),
            _ => {
                if self.buffer.len > 1 {
                    self.buffer.value.pop();
                    self.buffer.len -= 1;
                } else {
                    self.buffer.value.replace_with("0")
                }
            }
        }
    }

    pub fn add(&mut self) {
        self.perform_operation(Operation::Add)
    }

    pub fn subtract(&mut self) {
        self.perform_operation(Operation::Subtract)
    }

    pub fn multiply(&mut self) {
        self.perform_operation(Operation::Multiply)
    }

    pub fn divide(&mut self) {
        self.perform_operation(Operation::Divide)
    }

    pub fn square(&mut self) {
        match self.state {
            State::NewInput => {
                let x = self
                    .buffer
                    .value
                    .replacen(",", ".", 1)
                    .parse::<f32>()
                    .unwrap_or(0.);

                let result = x * x;

                self.buffer
                    .value
                    .replace_with(&result.to_string().replacen(".", ",", 1));

                self.cache.operation_value.replace_with(&format!("sqr({})", x));
                self.state = State::Calculated;
            }
            State::Equalized => {
                let x = self
                    .buffer
                    .value
                    .replacen(",", ".", 1)
                    .parse::<f32>()
                    .unwrap_or(0.);

                let result = x * x;
                self.buffer
                    .value
                    .replace_with(&result.to_string().replacen(".", ",", 1));
                self.cache.clear();
                self.cache.operation_value.replace_with(&format!("sqr({})", x));
            }

            _ => {
                self.equal();
                self.square();
            }
        }
    }

    pub fn equal(&mut self) {
        match self.state {
            State::Clear => {
                self.cache.previous_result.push_str(&self.buffer.value);
                self.buffer.state = BufferState::ClearOnInput;
                self.state = State::Equalized;
            }
            State::NewInput | State::Calculated => {
                match self.calculate(&self.cache.previous_result, &self.buffer.value) {
                    Some(result) => {
                        self.cache.operation_value.push_str(&self.buffer.value);
                        self.buffer
                            .value
                            .replace_with(&result.to_string().replacen(".", ",", 1));
                    }
                    None => self.cache.previous_result.replace_with(&self.buffer.value),
                }

                self.buffer.state = BufferState::ClearOnInput;
                self.state = State::Equalized;
            }
            State::Equalized => {
                self.cache.previous_result.replace_with(&self.buffer.value);

                if let Some(result) =
                    self.calculate(&self.cache.previous_result, &self.cache.operation_value)
                {
                    self.buffer
                        .value
                        .replace_with(&result.to_string().replacen(".", ",", 1))
                }
            }
            State::Squared => {
                //todo
            }
        }
        self.cache.ending_sign = "=";
    }

    //************************************************ASSOCIATED FUNCTIONS***************************************************//
    fn perform_operation(&mut self, operation: Operation) {
        //
        match self.state {
            State::Clear => {
                self.cache.previous_result.push_str(&self.buffer.value);
                self.set_state_calculated();
                self.clear_buffer_on_next_input();
            }
            State::NewInput => {
                if let Some(result) =
                    self.calculate(&self.cache.previous_result, &self.buffer.value)
                {
                    self.buffer
                        .value
                        .replace_with(&result.to_string().replacen(".", ",", 1))
                }

                self.cache.operation_value.clear();
                self.cache.previous_result.replace_with(&self.buffer.value);
                self.set_state_calculated();
                self.clear_buffer_on_next_input();
            }
            State::Equalized => {
                self.cache.ending_sign = NONE;
                self.cache.operation_value.clear();
                self.cache.previous_result.replace_with(&self.buffer.value);
                self.set_state_calculated();
                self.clear_buffer_on_next_input();
            }
            State::Squared => match self.operation {
                Operation::None => self.clear_buffer_on_next_input(),
                _ => {
                    if let Some(result) =
                        self.calculate(&self.cache.previous_result, &self.buffer.value)
                    {
                        self.buffer
                            .value
                            .replace_with(&result.to_string().replacen(".", ",", 1))
                    }
                    self.cache.operation_value.replace_with(&self.buffer.value);
                    self.set_state_calculated();
                    self.clear_buffer_on_next_input();
                }
            },
            State::Calculated => {}
        }

        if self.operation != operation {
            self.cache.change_operator(&operation);
            self.operation = operation;
        }
    }

    fn calculate(&self, first_value: &str, second_value: &str) -> Option<f32> {
        let first_value = first_value
            .replacen(",", ".", 1)
            .parse::<f32>()
            .unwrap_or(0.);

        let second_value = second_value
            .replacen(",", ".", 1)
            .parse::<f32>()
            .unwrap_or(0.);

        match self.operation {
            Operation::Add => Some(first_value.add(second_value)),
            Operation::Subtract => Some(first_value.sub(second_value)),
            Operation::Multiply => Some(first_value.mul(second_value)),
            Operation::Divide => Some(first_value.div(second_value)),
            _ => None,
        }
    }

    fn clear_buffer_on_next_input(&mut self) {
        if self.buffer.state != BufferState::ClearOnInput {
            self.buffer.state = BufferState::ClearOnInput
        }
    }

    fn set_state_calculated(&mut self) {
        if self.state != State::Calculated {
            self.state = State::Calculated
        }
    }
}

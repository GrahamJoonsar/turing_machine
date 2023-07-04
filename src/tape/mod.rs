use colored::Colorize;

pub struct Tape {
    tape: Vec<bool>,
    index: usize
}

impl Tape {
    pub fn new() -> Tape {
        let mut new_tape = Tape {
            tape: Vec::new(),
            index: 0
        };
        new_tape.tape.push(false);
        new_tape
    }

    // Get the value of the cell that the head is looking at 
    pub fn get(&self) -> bool {
        self.tape[self.index]
    }

    pub fn set(&mut self, val: bool) {
        self.tape[self.index] = val;
    }

    // Moves the head right
    pub fn move_right(&mut self) {
        self.index += 1;
        if self.index == self.tape.len() {
            self.tape.push(false);
        }
    }

    // Moves the head left
    pub fn move_left(&mut self) {
        if self.index != 0 {
            self.index -= 1;
        } else {
            self.tape.push(false);
            for i in 0..(self.tape.len()-1) {
                let j = self.tape.len() - i - 2;
                self.tape[j+1] = self.tape[j];
            }
            self.tape[0] = false;
        }
    }

    // Displays the cells on the tape that have been seen, with the cell
    // that the pointer is on being displayed differently.
    pub fn display(&self) {
        for i in 0..self.tape.len() {
            if i != self.index {
                match self.tape[i] {
                    true  => print!("1"),
                    false => print!("0")
                }
            } else {
                if self.tape[i] {
                    print!("{}", "1".bright_blue().on_bright_white());
                } else {
                    print!("{}", "0".bright_blue().on_bright_white());
                }
            }
        }
        println!();
    }
}

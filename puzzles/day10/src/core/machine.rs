use crate::core::{Command, Program};

pub type Register = isize;

#[derive(Debug, Clone)]
pub struct Machine {
    program: Program,
    register: Register,
    executing: Option<Command>,
    cycles_remaining: usize,
}

impl Machine {
    pub fn new(mut program: Program) -> Self {
        program.reverse();

        let mut machine = Self {
            program,
            register: 1,
            executing: None,
            cycles_remaining: 0,
        };

        machine.next_command();

        machine
    }

    fn next_command(&mut self) {
        if let Some(command) = self.program.pop() {
            self.executing = Some(command);
            self.cycles_remaining = command.cycles();
        } else {
            self.executing = None;
            self.cycles_remaining = 0;
        }
    }

    fn tick(&mut self) -> Option<Register> {
        if let Some(command) = self.executing {
            self.cycles_remaining -= 1;

            let register = self.register;
            let finished = self.cycles_remaining == 0;

            if finished {
                match command {
                    Command::Noop => {}
                    Command::AddX(value) => {
                        self.register += value;
                    }
                }

                self.next_command();
            }

            Some(register)
        } else {
            None
        }
    }

    pub fn run(&mut self) -> impl Iterator<Item = Register> + '_ {
        std::iter::from_fn(|| self.tick())
    }
}

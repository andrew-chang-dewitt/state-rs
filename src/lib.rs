pub struct Machine {
    current: String,
}

impl Machine {
    pub fn new(start: &str) -> Self {
        Machine {
            current: String::from(start),
        }
    }

    pub fn current_state(&self) -> &str {
        &self.current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_machine_has_a_starting_state() {
        let starting_state = "starting";
        let machine = Machine::new(starting_state);
        let current_state = machine.current_state();

        assert_eq!(current_state, starting_state);
    }
}

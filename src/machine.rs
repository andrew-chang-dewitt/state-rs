//! Anything that can dispatch an action for a possible state change and store the current state is
//! a Machine. That requires implementing the Machine trait, as well as the Dispatch trait for each
//! available Action.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_knows_its_current_state() {}

    #[test]
    fn it_can_transitions_states_with_dispatch() {
        todo!()
    }
}

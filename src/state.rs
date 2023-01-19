pub trait Receive<Action>
where
    Self: Into<<Self as Receive<Action>>::NewState>,
    <Self as Receive<Action>>::NewState: From<Self>,
{
    type NewState;

    fn act(self, _: Action) -> Self::NewState {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_change_from_one_state_to_another() {
        // Create an Off state
        #[derive(Debug, PartialEq)]
        struct Off;
        // it can change on <-> off
        impl From<On> for Off {
            fn from(_: On) -> Self {
                Off
            }
        }
        // it can receive a Toggle action
        impl Receive<Toggle> for Off {
            type NewState = On;
        }

        // Create an on state
        #[derive(Debug, PartialEq)]
        struct On;
        // it can change off <-> on
        impl From<Off> for On {
            fn from(_: Off) -> Self {
                On
            }
        }
        // it can receive a Toggle action
        impl Receive<Toggle> for On {
            type NewState = Off;
        }

        // Create a Toggle type to be used as an action
        struct Toggle;

        // start the state as On
        let state = On;
        // it should change to Off when Toggled
        assert_eq!(state.act(Toggle), Off);
    }
}

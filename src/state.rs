/// A State is anything that can Receive an Action and transition to a new State. This is done by
/// implementing the Receive trait for a given action on a specific type, during which you specify
/// the new State type.
///
/// The default implementation relies on From/Into already being implemented for the new & old
/// State types.
///
/// ```
/// use state_rs::Receive;
///
/// // create any type to implement Receive for
/// #[derive(Debug, PartialEq)]
/// struct StartingState;
/// // a type for it to transition to
/// #[derive(Debug, PartialEq)]
/// struct EndingState;
/// // and an action type
/// struct AnAction;
///
/// // Use the default implementation by first implementing From
/// impl From<StartingState> for EndingState {
///     fn from(_: StartingState) -> Self {
///         EndingState
///     }
/// }
/// // then implement Receive
/// impl Receive<AnAction> for StartingState {
///     // all that needs implemented is to specify the new state type
///     type NewState = EndingState;
/// }
///
/// let start = StartingState;
/// assert_eq!(start.act(AnAction), EndingState);
/// ```
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

    #[test]
    fn it_can_have_multiple_actions_per_state() {
        // States
        #[derive(Debug, PartialEq)]
        struct Idle;
        #[derive(Debug, PartialEq, Clone)]
        struct Pending;
        #[derive(Debug, PartialEq)]
        struct Succeeded;
        #[derive(Debug, PartialEq)]
        struct Failed;
        // Actions
        struct Submit;
        struct Success;
        struct Failure;

        // Can go from Idle to Pending on Submit
        impl Receive<Submit> for Idle {
            type NewState = Pending;
        }
        impl From<Idle> for Pending {
            fn from(_: Idle) -> Self {
                Pending
            }
        }

        // Can go from Pending to Succeeded on Success
        impl Receive<Success> for Pending {
            type NewState = Succeeded;
        }
        impl From<Pending> for Succeeded {
            fn from(_: Pending) -> Self {
                Succeeded
            }
        }

        // Can also go from Pending to Failed on Failure
        impl Receive<Failure> for Pending {
            type NewState = Failed;
        }
        impl From<Pending> for Failed {
            fn from(_: Pending) -> Self {
                Failed
            }
        }

        // Can retry when Failed on Submit
        impl Receive<Submit> for Failed {
            type NewState = Pending;
        }
        impl From<Failed> for Pending {
            fn from(_: Failed) -> Self {
                Pending
            }
        }

        // start at Idle
        let idle = Idle;

        // Submitting should transition to pending
        let pending = idle.act(Submit);
        assert_eq!(pending, Pending);

        // Pending can fail
        assert_eq!(pending.clone().act(Success), Succeeded);
        // or succeed
        assert_eq!(pending.act(Failure), Failed);
    }
}

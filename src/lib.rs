//! A super, ridiculously over-simplified implementation of a state machine using one trait.
//!
//! # Basic Example
//!
//! ```
//! use state_rs::Receive;
//!
//! // first define your States
//! struct On;
//! struct Off;
//! // and your Actions
//! struct Toggle;
//!
//! // then implement receive for the state transitions
//! impl From<Off> for On {
//!     fn from(_: Off) -> Self {
//!         On
//!     }
//! }
//! impl From<On> for Off {
//!     fn from(_: On) -> Self {
//!         Off
//!     }
//! }
//! impl Receive<Toggle> for On {
//!     type NewState = Off;
//! }
//! impl Receive<Toggle> for Off {
//!     type NewState = On;
//! }
//!
//! // finally, wrap it all up in a new Struct
//! // struct Switch<On, Off>;
//! // implement Machine for it
//! // impl Machine for Switch<On, Off> {
//! // }
//! // impl Dispatch<Toggle> for Switch<On, Off> {}
//! ```
mod state;

// Re-export state traits
pub use state::Receive;

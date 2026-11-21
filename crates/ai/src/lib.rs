//! Policies answering engine decisions on behalf of AI investigators.
#![deny(missing_docs)]

mod choices;
mod fighting;
mod gear;
mod goal;
mod knob;
mod movement;
mod odds;
mod policy;
mod route;
mod rumor;
mod seated;
mod sliders;
mod value;

pub use goal::{Reason, intention};
pub use policy::Policy;
pub use seated::Seated;

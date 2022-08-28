pub mod compose;
pub mod generator;
use self::{compose::Compose, generator::Generator};

pub trait Driver: Generator + Compose {}
impl<T: Generator + Compose> Driver for T {}
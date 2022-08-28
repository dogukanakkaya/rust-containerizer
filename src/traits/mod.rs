pub mod compose;
pub mod driver;
use self::{compose::Compose, driver::Driver};

pub trait Generator: Driver + Compose {}
impl<T: Driver + Compose> Generator for T {}

pub trait Image: Compose {}
impl<T: Compose> Image for T {}
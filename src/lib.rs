#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]

mod child;
mod swaynag;
#[cfg(test)]
mod tests;

pub(crate) type CowString = std::borrow::Cow<'static, str>;
pub use crate::swaynag::Swaynag;
pub use child::Child;

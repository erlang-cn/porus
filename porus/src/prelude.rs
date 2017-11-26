pub use super::compat::prelude::*;

pub use std::cmp::Ordering::Less;
pub use std::cmp::Ordering::Equal;
pub use std::cmp::Ordering::Greater;

pub use super::io;
pub use super::list;

pub fn default<T: Default>() -> T {
    Default::default()
}

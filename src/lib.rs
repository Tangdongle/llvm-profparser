pub mod instrumentation_profile;
pub mod util;

pub use crate::instrumentation_profile::parse;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ProfileFormat {
    Binary,
    CompactBinary,
    ExtBinary,
    Text,
    Gcc,
}

//! Write-Ahead Log for crash recovery and durability.

mod log;

pub use log::{WalEntry, WriteAheadLog};

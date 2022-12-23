mod buffers;
mod config;

use crate::types::Result;

pub trait FlashOperations {
    fn read(&mut self, block: u32, offset: u32, buffer: &mut [u8]) -> Result<()>;
    fn prog(&mut self, block: u32, offset: u32, buffer: &[u8]) -> Result<()>;
    fn erase(&mut self, block: u32) -> Result<()>;
    fn sync(&mut self) -> Result<()>;
}

pub use buffers::Buffers;
pub use config::{Config, ConfigError};
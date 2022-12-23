use core::fmt::Display;

use crate::{FlashOperations, Buffers};
use crate::controller::LFS;

const NAME_MAX: u32 = 255;
const FILE_MAX: u32 = 2147483647;
const ATTR_MAX: u32 = 1022;

#[derive(Debug)]
pub enum ConfigError {
    MissingReadSize,
    MissingProgramSize,
    MissingBlockSize,
    MissingBlockCount,
    InvalidBlockSize,
    InvalidCacheSize,
    InvalidLookaheadSize,
    InvalidMetadataMax
}

impl ConfigError {
    pub fn message(&self) -> &'static str {
        match self {
            ConfigError::MissingReadSize => "You must provide a value for 'read_size'",
            ConfigError::MissingProgramSize => "You must provide a value for 'prog_size'",
            ConfigError::MissingBlockSize => "You must provide a value for 'block_size'",
            ConfigError::MissingBlockCount => "You must provide a value for 'block_count'",
            ConfigError::InvalidBlockSize => "Must be a multiple of the read and program sizes",
            ConfigError::InvalidCacheSize => "Must be a multiple of the read and program sizes, and a factor of the block size",
            ConfigError::InvalidLookaheadSize => "Must be a multiple of 8",
            ConfigError::InvalidMetadataMax => "Must be less than or equal to the block size"
        }
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.message())
    }
}
pub struct Config {
    read_size: u32,
    prog_size: u32,
    block_size: u32,
    block_count: u32,
    block_cycles: i32,
    name_max: u32,
    file_max: u32,
    attr_max: u32,
    metadata_max: u32
}

impl Config {
    pub fn new(read_size: u32, prog_size: u32, block_size: u32, block_count: u32) -> Config {
        Config {
            read_size,
            prog_size,
            block_size,
            block_count,
            ..Default::default()
        }
    }

    pub fn mount(&self, flash: impl FlashOperations, buffers: Buffers) -> Result<LFS, ConfigError> {
        unimplemented!()
    }

    pub fn block_cycles(&mut self, value: i32) -> &mut Self {
        self.block_cycles = value.max(-1);
        self
    }

    pub fn name_max(&mut self, value: u32) -> &mut Self {
        self.name_max = value;
        self
    }

    pub fn file_max(&mut self, value: u32) -> &mut Self {
        self.file_max = value;
        self
    }

    pub fn attr_max(&mut self, value: u32) -> &mut Self {
        self.attr_max = value;
        self
    }

    pub fn metadata_max(&mut self, value: u32) -> &mut Self {
        self.metadata_max = value.min(self.block_size);
        self
    }

    fn validate(&self) -> Option<ConfigError> {
        if self.read_size == 0 {
            return Some(ConfigError::MissingReadSize)
        }

        if self.prog_size == 0 {
            return Some(ConfigError::MissingProgramSize)
        }

        if self.block_size == 0 {
            return Some(ConfigError::MissingBlockSize)
        }

        if self.block_count == 0 {
            return Some(ConfigError::MissingBlockCount)
        }

        if self.block_size % self.read_size != 0 ||
            self.block_size % self.prog_size != 0
        {
            return Some(ConfigError::InvalidBlockSize)
        }

        if self.metadata_max > self.block_size {
            return Some(ConfigError::InvalidMetadataMax)
        }

        return None
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            read_size: 0,
            prog_size: 0,
            block_size: 0,
            block_count: 0,
            block_cycles: -1,
            name_max: NAME_MAX,
            file_max: FILE_MAX,
            attr_max: ATTR_MAX,
            metadata_max: 0
        }
    }
}
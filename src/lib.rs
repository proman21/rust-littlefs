#![no_std]

mod types;
mod config;
mod controller;

pub use config::{Config, Buffers, FlashOperations};
pub use types::{Result, Error};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

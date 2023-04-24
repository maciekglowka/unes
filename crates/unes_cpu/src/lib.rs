#![no_std]
mod cpu;
pub mod flags;
mod opcodes;
mod tests;
mod utils;

pub use cpu::CPU;
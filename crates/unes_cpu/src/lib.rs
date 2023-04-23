#![no_std]
mod cpu;
pub mod flags;
mod opcodes;
mod tests;

pub use cpu::CPU;
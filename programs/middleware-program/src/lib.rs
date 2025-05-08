#![deny(missing_docs)]

//! Middleware program for Solana (SPI)
//! This program is designed to be used as a middleware layer for Solana
//! It provides a set of instructions that can be used to interact with other programs
//! It is designed to be used in conjunction with the Solana program library

/// Standart Solana entrypoint
pub mod entrypoint;
/// Main enum for managing the instructions
pub mod instruction;
/// Module for processing instructions and routing them
/// to the associated processor.
pub mod processor;
/// State and Type definitions for SPI.
pub mod state;

pub use solana_program;

solana_program::declare_id!("AtPDcrqAHATvnaerngsa1FpVzjXnY9tLwh3vkTpFBPCP");

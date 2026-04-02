//! AHCI (Advanced Host Controller Interface) SATA driver for bare-metal Rust.
//!
//! This crate provides a `no_std` AHCI driver that can detect SATA drives
//! and perform sector-level read/write I/O. It uses only volatile MMIO --
//! no Linux kernel, no POSIX, no userspace abstractions.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use ahci_nostd::AhciController;
//!
//! // After PCI enumeration finds an AHCI controller (class 0x01, subclass 0x06),
//! // read BAR5 to get the ABAR (AHCI Base Address Register).
//! let abar: u64 = /* PCI BAR5 */ 0xFEB0_0000;
//! let mut controller = unsafe { AhciController::init(abar).unwrap() };
//! // Now use controller.disks() to access detected drives.
//! ```

#![no_std]

extern crate alloc;

pub mod hba;
pub mod port;
pub mod command;
pub mod identify;
pub mod driver;

pub use driver::{AhciController, AhciDisk, AhciError};

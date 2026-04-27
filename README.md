# ahci-nostd

[![Crates.io](https://img.shields.io/crates/v/ahci-nostd.svg)](https://crates.io/crates/ahci-nostd)
[![docs.rs](https://docs.rs/ahci-nostd/badge.svg)](https://docs.rs/ahci-nostd)
[![License](https://img.shields.io/crates/l/ahci-nostd.svg)](https://github.com/suhteevah/ahci-nostd)

A `#![no_std]` AHCI (Advanced Host Controller Interface) SATA driver written in pure Rust.

Designed for bare-metal operating systems and embedded environments that need direct
SATA disk access without a Linux kernel, POSIX layer, or any OS abstractions.

## Features

- **Full AHCI 1.3.1 register definitions** -- HBA global registers, per-port registers, all documented bit fields
- **HBA initialization** -- reset, AHCI mode enable, BIOS/OS handoff
- **Port management** -- device detection, COMRESET, command engine start/stop
- **DMA command construction** -- READ DMA EXT, WRITE DMA EXT, IDENTIFY DEVICE, FLUSH CACHE EXT
- **ATA IDENTIFY parsing** -- model, serial, firmware, sector counts (LBA28/LBA48), sector sizes, NCQ support
- **High-level disk API** -- `read_sectors`, `write_sectors`, `read_bytes`, `write_bytes` with automatic sector alignment
- **Zero dependencies on std** -- only requires `alloc` for heap allocation and `log` for diagnostics
- **Volatile MMIO throughout** -- all hardware register access uses `core::ptr::read_volatile`/`write_volatile`

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
ahci-nostd = "0.1"
```

### Initialize the controller

```rust,no_run
use ahci_nostd::AhciController;

// After PCI enumeration finds an AHCI controller (class 0x01, subclass 0x06),
// read BAR5 to get the ABAR (AHCI Base Address Register).
let abar: u64 = 0xFEB0_0000; // from PCI config space

// SAFETY: abar must point to a valid, identity-mapped AHCI ABAR region.
let controller = unsafe { AhciController::init(abar).unwrap() };

for disk in controller.disks() {
    log::info!(
        "Found: {} ({} GiB)",
        disk.identify.model,
        disk.identify.capacity_bytes() / (1024 * 1024 * 1024),
    );
}
```

### Read and write sectors

```rust,no_run
use ahci_nostd::hba::HbaRegs;

// disk: &AhciDisk, hba: &HbaRegs (from the controller)
// Read 8 sectors (4 KiB) starting at LBA 0
let mut buf = [0u8; 4096];
// disk.read_sectors(&hba, 0, 8, &mut buf).unwrap();

// Write them back
// disk.write_sectors(&hba, 0, 8, &buf).unwrap();

// Or use the byte-level API for unaligned access
// disk.read_bytes(&hba, 1000, &mut buf[..100]).unwrap();
```

## Requirements

- A global allocator (`#[global_allocator]`) must be set up before calling any functions that allocate (which is most of them)
- The AHCI ABAR MMIO region must be identity-mapped (physical address == virtual address)
- The `log` crate facade should have a logger installed for diagnostic output

## Modules

| Module | Description |
|--------|-------------|
| `hba` | HBA memory-mapped register definitions and volatile accessors |
| `port` | Port initialization, device detection, command engine control |
| `command` | FIS, Command Header, PRDT, and command builder functions |
| `identify` | ATA IDENTIFY DEVICE response parser |
| `driver` | High-level `AhciController` and `AhciDisk` types |

## References

- [AHCI 1.3.1 Specification](https://www.intel.com/content/www/us/en/io/serial-ata/serial-ata-ahci-spec-rev1-3-1.html)
- [ATA/ATAPI Command Set (ACS-3)](https://standards.incits.org/apps/group_public/project/details.php?project_id=2124)
- [OSDev Wiki: AHCI](https://wiki.osdev.org/AHCI)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

---

## Support This Project

If you find this project useful, consider buying me a coffee! Your support helps me keep building and sharing open-source tools.

[![Donate via PayPal](https://img.shields.io/badge/Donate-PayPal-blue.svg?logo=paypal)](https://www.paypal.me/baal_hosting)

**PayPal:** [baal_hosting@live.com](https://paypal.me/baal_hosting)

Every donation, no matter how small, is greatly appreciated and motivates continued development. Thank you!

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-02

### Added

- Initial release extracted from ClaudioOS bare-metal operating system
- HBA memory-mapped register definitions (AHCI 1.3.1 spec compliant)
- Port initialization with device detection and COMRESET support
- DMA command construction: READ DMA EXT, WRITE DMA EXT, IDENTIFY DEVICE, FLUSH CACHE EXT
- ATA IDENTIFY DEVICE response parser (serial, model, firmware, LBA28/LBA48, sector sizes, NCQ)
- High-level `AhciController` for automatic HBA init and drive discovery
- High-level `AhciDisk` with sector-level and byte-level read/write APIs
- Unaligned byte-level I/O with automatic read-modify-write for partial sectors
- All volatile MMIO access for correctness on real hardware

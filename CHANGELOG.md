# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.1] - 2026-01-30

### Fixed

- Corrected window icon handling in the GUI by using `egui::IconData` with embedded PNG assets.
- Removed incorrect usage of raw `.ico` bytes in `eframe::NativeOptions`.

### Improved

- Improved Windows compatibility and correctness of GUI initialization.
- Clarified separation between executable icon (Windows resource) and window icon (runtime).

### Internal

- Minor cleanup of GUI bootstrap code.

---

## [0.1.0] – 2026-01-13

### Added

- Initial RedKey project bootstrap.
- Cross-platform graphical user interface based on `egui` / `eframe`.
- Tab-based navigation layout with the following sections:
    - General Info
    - CPU
    - RAM
    - Network
    - Disks
- Core data model for system information aggregation.
- System information collectors implemented using `sysinfo`:
    - CPU details (brand, cores, frequency)
    - Memory usage
    - Network interfaces (MAC addresses, IPs)
    - Disk information (mount points, filesystem, space)
    - General system information (OS, hostname)
- Manual refresh mechanism for system data.
- Initial RedKey branding and application icon assets.

### Notes

- This is the first public release and serves as a foundational version.
- APIs, data structures, and UI layout are subject to change.

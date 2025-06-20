# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.14](https://github.com/rcore-os/pie-boot/compare/pie-boot-loader-aarch64-v0.1.13...pie-boot-loader-aarch64-v0.1.14) - 2025-06-20

### Added

- enhance Pte creation with cache handling and add NoCache variant to CacheKind

### Fixed

- change PTE cache kind to NoCache in new_boot_table function

## [0.1.13](https://github.com/rcore-os/pie-boot/compare/pie-boot-loader-aarch64-v0.1.12...pie-boot-loader-aarch64-v0.1.13) - 2025-06-20

### Added

- enhance boot information structure and debug console initialization

## [0.1.12](https://github.com/rcore-os/pie-boot/compare/pie-boot-loader-aarch64-v0.1.11...pie-boot-loader-aarch64-v0.1.12) - 2025-06-19

### Other

- boot_info
- More boot info ([#15](https://github.com/rcore-os/pie-boot/pull/15))

## [0.1.11](https://github.com/rcore-os/pie-boot/compare/pie-boot-loader-aarch64-v0.1.10...pie-boot-loader-aarch64-v0.1.11) - 2025-06-17

### Added

- enhance memory management with CacheKind and reserve area tracking

## [0.1.10](https://github.com/rcore-os/pie-boot/compare/pie-boot-loader-aarch64-v0.1.9...pie-boot-loader-aarch64-v0.1.10) - 2025-06-17

### Fixed

- pte add cache for codes

## [0.1.9](https://github.com/rcore-os/pie-boot/compare/pie-boot-loader-aarch64-v0.1.8...pie-boot-loader-aarch64-v0.1.9) - 2025-06-17

### Other

- remove unused macros and clean up console module; update BootArgs struct to include kernel image addresses

## [0.1.8](https://github.com/rcore-os/pie-boot/compare/pie-boot-loader-aarch64-v0.1.7...pie-boot-loader-aarch64-v0.1.8) - 2025-06-15

### Other

- 明确crate构建目标

## [0.1.7](https://github.com/rcore-os/pie-boot/compare/pie-boot-loader-aarch64-v0.1.6...pie-boot-loader-aarch64-v0.1.7) - 2025-06-14

### Other

- update Cargo.lock dependencies

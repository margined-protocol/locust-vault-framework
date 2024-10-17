# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.5] - 2024/10/17

### Fixed

- Added checks that withdrawal cannot send zero tokens

## [0.0.4] - 2024/10/07

### Fixed

- Enabled the float to actually be used by the contract preventing strategy contract withdrawing all funds

## [0.0.3] - 2024/10/02

### Fixed

- Fixed the vault asset calculation to include the withdrawn assets, this was causing an error preventing users from depositing if asset were withdrawn

## [0.0.2] - 2024/10/01

### Fixed

- Fixed the estimate vault assets query

## [0.0.1] - 2024/09/30

### Added

- Release script to generate artifacts and release

[0.0.1]: https://github.com/margined-protocol/locust-vaults/releases/tag/0.0.1

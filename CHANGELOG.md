# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2023-07-09

### Added

- `cmd_exe::quote` and `cmd_exe::split` to support quoting and splitting
  command line arguments for cmd.exe

- `powershell::quote` and `powershell::split` to support quoting and splitting
  command line arguments for PowerShell

### Changed

- `vc_2008::parse` renamed to `vc_2008::split` to match other functions

### Removed

- Top level `split` function is now removed as there are three different
  possibilities

## [0.1.0] - 2022-08-04

### Added

- Initial commit of the project implementing `winsplit::split` and
  exposing the module directly via `winsplit::vc_2008::parse` in case we end
  up adding support for other parsing logic like pre-2008

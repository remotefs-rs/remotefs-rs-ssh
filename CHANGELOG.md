# Changelog

- [Changelog](#changelog)
  - [0.2.1](#021)
  - [0.2.0](#020)
  - [0.1.6](#016)
  - [0.1.5](#015)
  - [0.1.3](#013)
  - [0.1.2](#012)
  - [0.1.1](#011)
  - [0.1.0](#010)

---

## 0.2.2

Released on 03/05/2024

- `SshOpts::method` now requires `KeyMethod` and `MethodType` to setup key method

## 0.2.1

Released on 06/07/2023

- If ssh configuration timeout is `0`, don't set connection timeout

## 0.2.0

Released on 09/05/2023

- `SshOpts::config_file` now requires `SshConfigParseRule` as argument to specify the rules to parse the configuration file

## 0.1.6

Released on 19/04/2023

- Fixed relative paths resolve on Windows

## 0.1.5

Released on 18/04/2023

- Fixed relative paths resolve on Windows

## 0.1.3

Released on 10/02/2023

- Fixed client using ssh2 config parameter `HostName` to resolve configuration parameters.
- Bump `ssh2-config` to `0.1.4`

## 0.1.2

Released on 30/08/2022

- SshKeyStorage trait MUST return `PathBuf` instead of `Path`

## 0.1.1

Released on 20/07/2022

- Added `ssh2-vendored` feature to build libssl statically

## 0.1.0

Released on 04/01/2022

- First release

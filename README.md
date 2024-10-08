# remotefs SSH

<p align="center">
  <a href="https://veeso.github.io/remotefs-ssh/blob/main/CHANGELOG.md" target="_blank">Changelog</a>
  ·
  <a href="#get-started">Get started</a>
  ·
  <a href="https://docs.rs/remotefs-ssh" target="_blank">Documentation</a>
</p>

<p align="center">~ Remotefs SSH client ~</p>

<p align="center">Developed by <a href="https://veeso.github.io/" target="_blank">@veeso</a></p>
<p align="center">Current version: 0.4.1 (07/10/2024)</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://github.com/remotefs-rs/remotefs-rs-ssh/stargazers"
    ><img
      src="https://img.shields.io/github/stars/remotefs-rs/remotefs-rs-ssh.svg"
      alt="Repo stars"
  /></a>
  <a href="https://crates.io/crates/remotefs-ssh"
    ><img
      src="https://img.shields.io/crates/d/remotefs-ssh.svg"
      alt="Downloads counter"
  /></a>
  <a href="https://crates.io/crates/remotefs-ssh"
    ><img
      src="https://img.shields.io/crates/v/remotefs-ssh.svg"
      alt="Latest version"
  /></a>
  <a href="https://ko-fi.com/veeso">
    <img
      src="https://img.shields.io/badge/donate-ko--fi-red"
      alt="Ko-fi"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/remotefs-rs/remotefs-rs-ssh/actions"
    ><img
      src="https://github.com/remotefs-rs/remotefs-rs-ssh/workflows/Linux/badge.svg"
      alt="Linux CI"
  /></a>
  <a href="https://github.com/remotefs-rs/remotefs-rs-ssh/actions"
    ><img
      src="https://github.com/remotefs-rs/remotefs-rs-ssh/workflows/MacOS/badge.svg"
      alt="MacOS CI"
  /></a>
  <a href="https://github.com/remotefs-rs/remotefs-rs-ssh/actions"
    ><img
      src="https://github.com/remotefs-rs/remotefs-rs-ssh/workflows/Windows/badge.svg"
      alt="Windows CI"
  /></a>
  <a href="https://coveralls.io/github/remotefs-rs/remotefs-rs-ssh"
    ><img
      src="https://coveralls.io/repos/github/remotefs-rs/remotefs-rs-ssh/badge.svg"
      alt="Coveralls"
  /></a>
  <a href="https://docs.rs/remotefs-ssh"
    ><img
      src="https://docs.rs/remotefs-ssh/badge.svg"
      alt="Docs"
  /></a>
</p>

---

## About remotefs-ssh ☁️

remotefs-ssh is a client implementation for [remotefs](https://github.com/remotefs-rs/remotefs-rs), providing support for the SFTP/SCP protocol.

---

## Get started 🚀

First of all, add `remotefs-ssh` to your project dependencies:

```toml
remotefs = "0.3"
remotefs-ssh = "^0.4"
```

these features are supported:

- `find`: enable `find()` method on client (*enabled by default*)
- `no-log`: disable logging. By default, this library will log via the `log` crate.
- `ssh2-vendored`: build with static libssl

---

### Client compatibility table ✔️

The following table states the compatibility for the client client and the remote file system trait method.

Note: `connect()`, `disconnect()` and `is_connected()` **MUST** always be supported, and are so omitted in the table.

| Client/Method  | Scp | Sftp |
|----------------|-----|------|
| append_file    | No  | Yes  |
| append         | No  | Yes  |
| change_dir     | Yes | Yes  |
| copy           | Yes | Yes  |
| create_dir     | Yes | Yes  |
| create_file    | Yes | Yes  |
| create         | Yes | Yes  |
| exec           | Yes | Yes  |
| exists         | Yes | Yes  |
| list_dir       | Yes | Yes  |
| mov            | Yes | Yes  |
| open_file      | Yes | Yes  |
| open           | Yes | Yes  |
| pwd            | Yes | Yes  |
| remove_dir_all | Yes | Yes  |
| remove_dir     | Yes | Yes  |
| remove_file    | Yes | Yes  |
| setstat        | Yes | Yes  |
| stat           | Yes | Yes  |
| symlink        | Yes | Yes  |

---

## Support the developer ☕

If you like remotefs-ssh and you're grateful for the work I've done, please consider a little donation 🥳

You can make a donation with one of these platforms:

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)
[![bitcoin](https://img.shields.io/badge/Bitcoin-ff9416?style=for-the-badge&logo=bitcoin&logoColor=white)](https://btc.com/bc1qvlmykjn7htz0vuprmjrlkwtv9m9pan6kylsr8w)

---

## Contributing and issues 🤝🏻

Contributions, bug reports, new features, and questions are welcome! 😉
If you have any questions or concerns, or you want to suggest a new feature, or you want just want to improve remotefs, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog ⏳

View remotefs' changelog [HERE](CHANGELOG.md)

---

## Powered by 💪

remotefs-ssh is powered by these aweseome projects:

- [ssh2-config](https://github.com/veeso/ssh2-config)
- [ssh2-rs](https://github.com/alexcrichton/ssh2-rs)

---

## License 📃

remotefs-ssh is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)

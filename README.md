# Rust-template

<p align="center">~  ~</p>
<p align="center">
  <a href="#get-started-">Get started</a>
  ·
  <a href="https://crates.io/crates/rust-template" target="_blank">Crates.io</a>
</p>
<p align="center">Developed by <a href="https://veeso.dev/" target="_blank">@veeso</a></p>
<p align="center">Current version: 0.1.0 (26/06/2023)</p>

<p align="center">
  <a href="https://opensource.org/license/mit/"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://github.com/veeso-dev/rust-template/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso-dev/rust-template.svg"
      alt="Repo stars"
  /></a>
  <a href="https://crates.io/crates/rust-template"
    ><img
      src="https://img.shields.io/crates/d/rust-template.svg"
      alt="Downloads counter"
  /></a>
  <a href="https://crates.io/crates/rust-template"
    ><img
      src="https://img.shields.io/crates/v/rust-template.svg"
      alt="Latest version"
  /></a>
  <a href="https://ko-fi.com/veeso">
    <img
      src="https://img.shields.io/badge/donate-ko--fi-red"
      alt="Ko-fi"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/veeso-dev/rust-template/actions"
    ><img
      src="https://github.com/veeso-dev/rust-template/workflows/build-test/badge.svg"
      alt="Linux CI"
  /></a>
</p>

---

- [Rust-template](#rust-template)
  - [About rust-template](#about-rust-template)
  - [Get started](#get-started)
    - [Setup env](#setup-env)
    - [Run with Cargo make](#run-with-cargo-make)
  - [rust-template API](#rust-template-api)
    - [Check](#check)
  - [Contributing and issues](#contributing-and-issues)
  - [Changelog](#changelog)
  - [License](#license)

---

## About rust-template

rust-template is a Rust web service which comes integrated with ClamAV. The service provides an API endpoint to scan files with ClamAV.

---

## Get started

### Setup env

```sh
cp .env.test .env
vim .env
```

```env
WEB_PORT=3001
```

### Run with Cargo make

```sh
cargo make -p production run
```

## rust-template API

### Check

Check web service status:

```txt
GET /check
```

Response: Empty (200)

---

## Contributing and issues

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve pavao, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog

View rust-template's changelog [HERE](CHANGELOG.md)

---

## License

rust-template is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)

<h1>TLDRx</h1>

<p>
  <a href="https://github.com/tldrx/tldrx/actions/workflows/ci.yml/badge.svg"><img alt="GitHub CI" src="https://github.com/tldrx/tldrx/actions/workflows/ci.yml/badge.svg"></a>
  <a href="https://img.shields.io/crates/l/tldrx"><img alt="License" src="https://img.shields.io/crates/l/tldrx"></a>
  <a href="https://img.shields.io/crates/v/tldrx"><img alt="License" src="https://img.shields.io/crates/v/tldrx"></a>
</p>

<p>English | <a href="README.zh.md">简体中文</a></p>

<p>
  <img src="docs/screenshot.png" alt="screenshot" width="582">
</p>


## Features

- [x] Support private tldr pages repo. 
- [x] Support editing private tldr pages.
- [x] Show pages with platform info attached.
- [x] Adheres to [tldr-pages client specification](https://github.com/tldr-pages/tldr/blob/main/CLIENT-SPECIFICATION.md).[^1]
- [x] Offline caching official tldr pages repo.
- [x] Configurable official tldr pages archive download link.
- [x] Honor `HTTP_PROXY` and `HTTPS_PROXY` system proxies (handy for regulation area).
- [x] Advanced configuration: color style, platform, editor...
- [x] Support [new tldr pages syntax](https://github.com/tldr-pages/tldr/pull/958).[^2]


## Usages

Show pages for `git commit`:

    tldrx git commit

Update local cache(Required for the first time before showing official pages):

    tldrx --update

Edit or create private page for `git commit`:

    tldrx -e git commit

For more:

    tldrx --help


## Installation

Assume you have rust cargo installed:

    cargo install tldrx


## License

This project is dual-licensed under [MIT](LICENSE-MIT) license and [MulanPSL-2.0](LICENSE-MulanPSL) license.
You can freely choose one or the other that suits you.



[^1]: 1. Use `tldrx` instead of `tldr`.
      2. `-l` `--list` options for listing all the pages not supported yet.

[^2]: The new syntax for tldr-pages is an experimental RFC.
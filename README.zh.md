<h1>TLDRx</h1>

<p>
  <a href="https://github.com/tldrx/tldrx/actions/workflows/ci.yml/badge.svg"><img alt="GitHub CI" src="https://github.com/tldrx/tldrx/actions/workflows/ci.yml/badge.svg"></a>
  <a href="https://img.shields.io/crates/l/tldrx"><img alt="License" src="https://img.shields.io/crates/l/tldrx"></a>
  <a href="https://img.shields.io/crates/v/tldrx"><img alt="License" src="https://img.shields.io/crates/v/tldrx"></a>
</p>

<p><a href="README.md">English</a> | 简体中文</p>

<p>
  <img src="docs/screenshot.png" alt="screenshot" width="582">
</p>


## 说明

> #### tldr-pages:
> Simplified and community-driven man pages.

> #### tldr:
> TL;DR stands for "Too Long; Didn't Read".

`tldr-pages` 此处称之为「简读页」。简读页可以看作是简化版的命令手册页（man pages）。
通常一些命令或工具为了完整性会将其所有用法及介绍编入手册页，从而导致篇幅长且难以阅读使用，
而简读页则只择其常用用法并以示例形式编写，使其有简短且易使用的特征。


## 特点

- [x] 支持配置查阅私有简读页数据。
- [x] 支持编辑私有简读页。
- [x] 查阅简读页时附有目标平台信息。
- [x] 遵循[简读页客户端规范](https://github.com/tldr-pages/tldr/blob/main/CLIENT-SPECIFICATION.md)。[^1]
- [x] 离线存储官方简读页数据。
- [x] 官方简读页数据下载链接可配置化。
- [x] 遵循 `HTTP_PROXY` 和 `HTTPS_PROXY` 网络代理配置（管控地区尤为有用）。
- [x] 自定义配置：颜色样式、平台、编辑器等
- [x] 支持[新简读页语法](https://github.com/tldr-pages/tldr/pull/958)。[^2]


## 用法

查阅 `git commit` 命令简读页：

    tldrx git commit

更新本地缓存简读页数据（首次查阅官方简读页数据须先执行该命令）：

    tldrx --update

编辑或新建 `git commit` 命令私有简读页：

    tldrx -e git commit

更多用法：

    tldrx --help


## 安装

假定你己安装好 rust cargo：

    cargo install tldrx


## 许可证

本项目遵循 [MIT](LICENSE-MIT) 和 [MulanPSL-2.0](LICENSE-MulanPSL) 双许可。您可自由选择两款许可中任意一款适合您的来遵循。



[^1]: 1. 使用 `tldrx` 替代 `tldr`。
      2. `-l` `--list` 选项查看所有简读页暂未实现。

[^2]: 简读页新语法目前仍为一个实验性的 RFC 提议。
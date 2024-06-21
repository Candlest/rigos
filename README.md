<div align="center">
  <img src="./README/rigos_logo.png" alt="" width=320>
  <p><strong>Rigos: Rigos is a generator of sites, written in Rust</strong></p>

</div>

[中文](./README/README_CN.md)

# Overview
Rigos is a lightweight and high-speed single-file static site generator, written in Rust.
If you want to use Rigos, please understand the following information:
Advantages of Rigos:
- Lightweight single-file, avoiding cumbersome installation processes and environment variable configurations
- High performance, website rendering speed is extremely fast
Disadvantages of Rigos:
- Currently lacks flexibility, with poor customizability

# Installation
There is currently no packaging, installation can be done from the source code.

That is, obtain the source code from the release or directly `git clone`, and compile with `cargo build --release`. After compilation, add the binary file to the environment variable to use from the command line. Here is an example for Linux:

```bash
git clone https://github.com/Candlest/rigos.git
cd ./rigos
cargo build --release
sudo cp ./target/release/rigos /usr/bin/rigos
```

For Windows systems, you can directly copy `target/release/rigos.exe` to `C:\Windows`

# Usage
You can get the help list through `rigos help`.

Render the `/pub` directory: `rigos render`

Preview from the `/pub` directory: `rigos preview`

Lazy package: `rigos rap`

> If you have used `hexo` before, `rigos` is easy to get started with, because the `rigos` command is inspired by `hexo`.

Deploy to remote (not implemented yet): `rigos deploy`

# Related
- [ONEPAGE](https://github.com/hanpei/onepage) has greatly inspired me, especially in the part of rendering templates.
- [Hexo](https://github.com/hexojs/hexo), as a static blog generator with both expandability and ease of use, has cultivated my user habits and inspired me in command line design.
- I have referred to a lot of information on the Internet, and I will explain the main parts in my [development records](https://www.zhihu.com/column/c_1664617254036639745).

# Contributors
[<a href="https://github.com/Candlest/rigos/graphs/contributors"><img src="https://contrib.rocks/image?repo=Candlest/rigos" alt="Contributors" /></a>](https://github.com/Candlest)

Any contribution is welcome!

# License
This project is open source under the **MIT License**.
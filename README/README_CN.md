<div align="center">
  <img src="./rigos_logo.png" alt="" width=320>
  <p><strong>Rigos: Rigos is a generator of sites, written in Rust</strong></p>

</div>

# 概述
Rigos是一个轻量级高速度的单文件静态站点生成器，使用Rust编写。
如果你想使用Rigos，请了解以下信息：
Rigos的优点：
- 轻量级单文件，避免繁琐的安装过程和环境变量配置
- 高性能，网站渲染速度极快
Rigos的缺点：
- 目前欠缺灵活性，可自定义性欠佳

# 安装

目前暂时没有打包，可以从源代码安装。

即从release或者直接`git clone`获取源码，使用`cargo build --release`进行编译。编译完成后，将二进制文件加入环境变量，即可从命令行中使用。下面是一个适用于 linux 的例子：

```bash
git clone https://github.com/Candlest/rigos.git
cd ./rigos
cargo build --release
sudo cp ./target/release/rigos /usr/bin/rigos
```

对于 windows 系统，可以直接将 `target/release/rigos.exe` 复制到 `C:\Windows`

# 使用

可以通过`rigos help`获取帮助列表

渲染`/pub`目录：`rigos render`

从`/pub`目录预览：`rigos preview`

懒人包：`rigos rap`

> 如果你曾经使用过`hexo`，那么`rigos`是很容易上手的，因为`rigos`命令的灵感从`hexo`而来。

部署至远程（暂未实现）：`rigos deploy`

# 相关

- [ONEPAGE](https://github.com/hanpei/onepage)极大地给予我灵感，尤其是渲染模板的部分。
- [Hexo](https://github.com/hexojs/hexo)作为一个兼具拓展性和易用性的静态博客生成器，培养了我的用户习惯，给予我在命令行设计上的灵感。
- 参考了很多互联网上的资料，在我的[开发记录](https://www.zhihu.com/column/c_1664617254036639745)中会说明其中的主要部分。

# 贡献者

[<a href="https://github.com/Candlest/rigos/graphs/contributors"><img src="https://contrib.rocks/image?repo=Candlest/rigos" alt="Contributors" /></a>](https://github.com/Candlest)

欢迎任何的贡献！

# 许可证

本项目使用 **MIT License** 开源。
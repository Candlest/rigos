# README

> ⚠️ This project is still in the early development stage and will undergo refactoring. It is not recommended to use before version 1.0! ! !

- [README](#readme)
  - [About rigos](#about-rigos)
  - [demo DEMO](#demo-demo)
  - [Install](#install)
  - [use](#use)
    - [rigos command](#rigos-command)
    - [Publishing the web page](#publishing-the-web-page)
  - [Development Plan](#development-plan)
  - [TOML Configs](#toml-configs)
    - [config.toml](#configtoml)
    - [TOML Front Matter](#toml-front-matter)
  - [relevant](#relevant)
  - [Contributors](#contributors)
  - [license](#license)


the Chinese version: [中文](./README/README_CN.md)

## About rigos

rigos is a generator of sites, writen in Rust, which implements the following functions:

- Convert `markdown` to `html` via [pulldown cmark](https://github.com/raphlinus/pulldown-cmark)
- Use [toml-rs](https://github.com/toml-rs/toml) to implement article properties similar to `YAML Front Matter`
- Implement `html` template rendering of [Django template language](https://docs.djangoproject.com/en/3.1/topics/templates/) through [tera](https://github.com/Keats/tera)
- Currently using [warp](https://github.com/seanmonstar/warp) to preview the site (consider rewriting)

[View our development plan] (#development plan).

## demo DEMO

My blog: https://candlest.github.io/

![demo1](./README/demo_png.png)
![demo_classless_css](./README/demo_classless_css.png)

## Install

Currently not packaged, it can be installed from source code.

> in the future ;-)

That is, get the source code from release or directly `git clone`, and use `cargo build --release` to compile. After the compilation is complete, add the binary file to the environment variable and use it from the command line. Here is an example for `linux`:

```bash
git clone https://github.com/Candlest/rigos.git
cd ./rigos
cargo build --release
sudo cp ./target/release/rigos /usr/bin/rigos
```

## use

### rigos command

The help list can be obtained through `rigos help` or `rigos`

Build the `/public` directory: `rigos build`

Clean up the `/public` directory: `rigos clear`

Run from the `/public` directory: `rigos run`

Lazy package: `clear`, `build`, `run` in one go: `rigos cbr`

> If you have used `hexo`, then `rigos` is very easy to use, because the `rigos` command is inspired by `hexo`.

### Publishing the web page

The `/public` directory is the root directory of the generated website, just upload it.

## Development Plan

> Since Candlest is preparing for the Chinese college entrance examination in 2024, the development progress is likely to be delayed.

`TOML Front Matter` related:

- [ ] `TOML Front Matter` default handling
- [x] More `TOML Front Matter`
- [ ] Generate default `TOML Front Matter` via command line arguments
- [x] Sort articles by `TOML Front Matter`
- [x] index articles by `tags` and `category`

Plugins and Extensibility:

- [x] Use `config.toml` to manage webpage related parameters
- [ ] Build a plugin management system and use the plugin management system to add `gitalk`, `highlight.js`, etc.
- [ ] Build a theme management system and create default themes

Code related:

- [ ] Seriously write error handling instead of `unwarp()`.
- [ ] Organize projects and learn the project format of [crates.io](https://crates.io)

Library related:

- [ ] Write a server implementation that can meet the needs of the blog, or find a library that is both lightweight and high-performance
- [x] Use the clap library to process command line arguments

Documentation related:

- [x] Generate list of contributors
- [ ] Disassemble README

## TOML Configs

### config.toml

at the root directory of your blog:

``` rust

    pub page_templates: Vec<String>,
    pub public_dir: String,
    pub source_dir: String,
    pub static_dir: String,
    pub page_dir: String,
    pub template_dir: String,
    pub theme: String,

```

### TOML Front Matter

```rust
pub struct HProperty {
    pub title: String,
    pub datetime: Datetime,
    pub tags: Vec<String>,
    pub category: String,
    pub url_name: String,
}
```

We can set the properties of the `markdown` document through `TOML Front Matter` and access them through the `tera` templating language.

## relevant

- [ONEPAGE](https://github.com/hanpei/onepage) gave me great inspiration, especially the part of rendering templates.
- [Hexo](https://github.com/hexojs/hexo), as a static blog generator with both scalability and ease of use, cultivated my user habits and gave me inspiration for command line design.
- I refer to a lot of information on the Internet, and I will explain the main parts in my [Development Record](https://www.zhihu.com/column/c_1664617254036639745).

## Contributors

<a herf="https://github.com/Candlest/rigos/graphs/contributors"><img src="https://contrib.rocks/image?repo=Candlest/rigos" alt="Contributors" /></a>

Contributions of any kind are welcome!

## license

This project uses **MIT License** open source.

# README

the Chinese version: [中文](./README/README_CN.md)

## About rublog

rublog` /rʌblɑg/` is a static blog generator written in `Rust`, which implements the following functions:

- Convert `markdown` to `html` via [pulldown cmark](https://github.com/raphlinus/pulldown-cmark)
- Use [toml-rs](https://github.com/toml-rs/toml) to implement article attributes similar to `YAML Front Matter`
- Implemented `html` template rendering similar to [Django template language](https://docs.djangoproject.com/en/3.1/topics/templates/) through [tera](https://github.com/Keats/tera)
- Implemented a very simple local HTTP server for preview sites

What is planned:

- [ ] Implement a `rublog_config.toml` file within a project
- [ ] Implement `rublog push` to push `/public` to `Github Page`
- [ ] More `Front Matter`
- [ ] Implement `rublog init $PROJECT`

## demo DEMO

My blog: https://candlest.github.io/

![demo](./README/demo_png.png)

## Install

Currently not packaged, it can be installed from source code.

> will come later ;-)

That is, get the source code from release or directly `git clone`, and use `cargo build --release` to compile. After compiling, add the binary file to the environment variable and use it from the command line.

## Usage

### Initialize from rublog-template

We can initialize our project from [rublog-template](https://github.com/Candlest/rublog-template), please see its [README](https://github.com/Candlest/rublog-template/blob/main/README.md) for details, so I won’t repeat it here.

```bash
git clone https://github.com/Candlest/rublog-template.git
```

### rublog command

The help list can be obtained through `rublog help` or `rublog`

Build the `/public` directory: `rublog build`

Clean up the `/public` directory: `rublog clear`

Run from the `/public` directory: `rublog run`

Lazy bag: `clear`, `build`, `run` all in one go: `rublog cbr`

> If you have used `hexo`, then `rublog` is very easy to use, because the `rublog` command is inspired by `hexo`.

### Publishing the web page

The `/public` directory is the root directory of the generated website, just upload it.

> This design is also inspired by `hexo`.

## relevant

- [ONEPAGE](https://github.com/hanpei/onepage) gave me great inspiration, especially the part of rendering templates.
-[Hexo](https://github.com/hexojs/hexo), as a static blog generator with both scalability and ease of use, cultivated my user habits and gave me inspiration for command line design.
- I have referred to a lot of information on the Internet, and I will explain the main parts in my [Development Record](https://www.zhihu.com/column/c_1664617254036639745).

## Contributors

[@Candlest](https://github.com/Candlest)

Any contributions are welcome!

## license

This project is open sourced using **MIT License**.

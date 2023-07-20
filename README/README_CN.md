# README_CN

## 关于rublog

rublog` /rʌblɑg/ `是一个使用`Rust`编写的静态博客生成器，实现了以下功能：

- 通过[pulldown cmark](https://github.com/raphlinus/pulldown-cmark)将`markdown`转换为`html`
- 使用[toml-rs](https://github.com/toml-rs/toml)实现了类似`YAML Front Matter`的文章属性
- 通过[tera](https://github.com/Keats/tera)实现类[Django template language](https://docs.djangoproject.com/en/3.1/topics/templates/)的`html`模板渲染
- 实现了一个非常简陋的本地HTTP服务器以供预览网站

计划实现的：

- [ ] 实现一个项目内的`rublog_config.toml`文件
- [ ] 实现`rublog push`将`/public`推送至`Github Page`
- [ ] 暴露更多的`Front Matter`
- [ ] 实现`rublog init $PROJECT`

## 演示DEMO

我的博客：https://candlest.github.io/

![demo](./demo_png.png)

## 安装

目前暂时没有打包，可以从源代码安装。

> 以后会有 ;-)

即从release或者直接`git clone`获取源码，使用`cargo build --release`进行编译。编译完成后，将二进制文件加入环境变量，即可从命令行中使用。

## 使用

### 从rublog-template初始化

我们可以从[rublog-template](https://github.com/Candlest/rublog-template)初始化我们的项目，详情请看其[README](https://github.com/Candlest/rublog-template/blob/main/README.md)，这里不再赘述。

```bash
git clone https://github.com/Candlest/rublog-template.git
```

### rublog的命令

可以通过`rublog help`或者`rublog`获取帮助列表

构建`/public`目录：`rublog build`

清理`/public`目录：`rublog clear`

从`/public`目录运行：`rublog run`

懒人包：`clear`, `build`, `run`一气呵成：`rublog cbr`

> 如果你曾经使用过`hexo`，那么`rublog`是很容易上手的，因为`rublog`命令的灵感从`hexo`而来。

### 发布网页

`/public`目录即为被生成网站的根目录，上传即可。

> 这个设计的灵感也是来自`hexo`。

## 相关

- [ONEPAGE](https://github.com/hanpei/onepage)极大地给予我灵感，尤其是渲染模板的部分。
- [Hexo](https://github.com/hexojs/hexo)作为一个兼具拓展性和易用性的静态博客生成器，培养了我的用户习惯，给予我在命令行设计上的灵感。
- 参考了很多互联网上的资料，在我的[开发记录](https://www.zhihu.com/column/c_1664617254036639745)中会说明其中的主要部分。

## 贡献者

[@Candlest](https://github.com/Candlest)

欢迎任何的贡献！

## 许可证

本项目使用**MIT License**开源。

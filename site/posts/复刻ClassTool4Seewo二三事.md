title="复刻ClassTool4Seewo二三事"
filename="redo"
date="2023-06-20T19:19:25"
tags=["C#", "Win API"]
category="ClassBoard开发日志"
%%%%%%

# 复刻ClassTool4Seewo二三事

  好久好久以前，在Github发现了这个项目：[ClassTool](https://github.com/IcariaWorks/ClassTools)

> ClassTool
>
> 一个用在班级电脑上面的动态壁纸系统，显示高考倒计时、作业、课程表、值日生等

  作为电教，上周在班级电脑上使用了一下。好巧不巧，**我校希沃大概是17左右引进的，羸弱的性能、Win7的系统、冰点还原和校园白名单机制，使得ClassTool不能在这台机上很好地运行**，尤其是不能作为桌面背景层。

  也是夜郎自大，不曾想过自己的编程能力也和这台机一样羸弱，和同学夸下海口“我高考假七天就能复刻一个出来”，自6月5日晚以来“键”耕不掇，尝试了MFC、C#Winform、Electron+Vue、Rust+egui技术栈，无一可以找到在**那台Win7的、有冰点还原的、基本不能联网的、性能羸弱的一体机**上实现的良好方案。

  但是不完美的方案倒是有的\_(:з」∠)\_，而且是我初中便实现过的（MFC+WinForm），或者是我现在有望实现的（Rust + egui），原作ClassTool也是迭代版本多次几年才有现在的样子，我也无需那么着急。

### 吐槽Electron

  此外，我真的很想吐槽Node.JS或者吐槽整个前端系列的东西.....

  我依稀记得，使用Node.JS + Electron + Vue创建项目的时候，仅仅是一个Hello World项目体积就去到了5GB，编译后也有500MB+。大头都在依赖上面，大概是因为我不怎么懂HTML/JS/CSS一类的东西，所以才不懂得缩小。（毕竟ClassTool的Release版本大小只有230MB）

  相对于自带Chromium运行时的程序来说，虽然230MB已经算很小，但是这是和Rust方案编译后100MB以下（Debug模式）的大小不可同日而语的。此外，Rust的编译速度竟然也比Node.JS快（也有Electron + Vue交火的原因）

  没有踩一捧一的意思o(╥﹏╥)o

### 动态壁纸原理

TODO

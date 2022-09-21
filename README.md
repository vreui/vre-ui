# vre-ui
<https://github.com/vreui/vre-ui>

[![CI](https://github.com/vreui/vre-ui/actions/workflows/ci.yml/badge.svg)](https://github.com/vreui/vre-ui/actions)

威惹: 框架 (应用代码, wasm)

+ 威惹文档 <https://github.com/vreui/vre-doc>


## cargo features

+ `wr` (默认启用)

  使用 WebRender 渲染 (OpenGL / OpenGL ES).

  平台: wayland (GNU/Linux), Android, Windows

+ `bc`

  使用浏览器 CSS+HTML 渲染 (布局).

  平台: web

+ `np` (默认启用)

  非组件代理模式 (普通模式).

+ `proxy`

  组件代理模式.

+ `wayland` (默认启用)

  启用 wayland (GNU/Linux) 平台支持代码.

  如果不启用此功能, 则不会包含此平台的支持代码,
  编译生成的 wasm 代码也不支持在此平台运行.

+ `android`

  启用 Android 平台支持代码.
  同上.

+ `windows`

  启用 windows 平台支持代码.
  同上.

+ `web`

  启用 web 平台支持代码.
  同上.


## LICENSE

`Mozilla Public License 2.0`

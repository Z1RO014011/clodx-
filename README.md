# Clodx

一个本地优先的 Codex 用量状态栏应用。Clodx 会在桌面上显示当前套餐的剩余百分比、额度刷新时间，以及可用的重置次数。

> 当前为早期版本。界面采用蓝白终端风格，适合在写代码时快速查看额度。

## 功能

- 显示 **5 小时额度** 的剩余百分比和刷新时间
- 显示 **周额度** 的剩余百分比和刷新时间
- 显示当前可用的 **额度重置次数**
- 悬浮状态窗显示紧凑信息：`5H / WK`；点击可打开完整详情
- 原生托盘菜单：显示/隐藏、立即刷新、退出
- 支持 macOS 和 Windows

## 使用方式

1. 在电脑上安装并登录 Codex Desktop。
2. 下载并打开对应系统的 Clodx 安装包。
3. Clodx 会读取本机已有的 Codex 登录状态，自动刷新并显示额度。

数据无法读取时，界面会提示“不可用”，不会伪造或猜测额度。

## 隐私与安全

Clodx 仅在内存中使用本机的 Codex 登录状态来查询额度。它不会保存或上传访问令牌、账户 ID、提示词、对话历史或原始额度响应；也不会自动使用重置次数或修改任何账户设置。

> 额度数据来自非公开的内部接口，OpenAI 若调整接口，应用可能暂时无法获取数据。

## 安装包

请在 [Releases](https://github.com/Z1RO014011/clodx-/releases) 页面下载最新版。

| 系统 | 文件 | 说明 |
| --- | --- | --- |
| macOS | `.dmg` | Intel Mac；Apple Silicon 可通过 Rosetta 运行 |
| Windows | `.msi` 或 `.exe` | 分别为 MSI 与 NSIS 安装器 |

当前安装包未进行商业代码签名：macOS 可能要求在“隐私与安全性”中确认打开，Windows 可能显示 SmartScreen 提示。

## 本地开发

要求：Node.js 20+、Rust stable，以及 [Tauri v2 环境依赖](https://v2.tauri.app/start/prerequisites/)。

```sh
npm install
npm run dev -- --host 127.0.0.1
# 在另一个终端中运行
npm run tauri dev
```

## 验证

```sh
npm test
npm run build
cd src-tauri && cargo test && cargo check
```

## 构建发布包

推送形如 `v0.1.0` 的版本标签，或在 Actions 中手动执行 **Build installers**。工作流会生成：

- `clodx-macos-x64`：macOS DMG
- `clodx-windows`：Windows MSI 和 NSIS 安装器

## 许可证

当前仓库尚未指定开源许可证。在引入外部贡献或公开复用前，请补充许可证文件。

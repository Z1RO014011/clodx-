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

## 本地构建安装包

和常见 Electron Builder 项目一样，Clodx 在本机打包，并将产物统一输出到 `release/` 目录。无需 GitHub Actions。

在 Mac 上构建当前架构的安装包（Apple Silicon Mac 会生成 arm64，Intel Mac 会生成 x64）：

```sh
npm install
npm run package:mac
```

该命令生成：

- `release/Clodx-macos-<架构>.app.zip`
- `release/Clodx-macos-<架构>.dmg`

在 Windows 电脑上构建：

```powershell
npm install
npm run package:win
```

该命令生成 `release/` 下的 MSI 与 NSIS EXE 安装包。

> 目前没有商业签名。将未签名的 macOS 安装包发给其他人时，首次打开可能需要在“系统设置 → 隐私与安全性”中选择“仍要打开”；Windows 也可能显示 SmartScreen 提示。

## 许可证

当前仓库尚未指定开源许可证。在引入外部贡献或公开复用前，请补充许可证文件。

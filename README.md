# SetProxy

A lightweight Windows system proxy management application built with Tauri + Vue 3.

一款轻量级的 Windows 系统代理管理工具，基于 Tauri + Vue 3 构建。

[English](#features) | [中文](#功能特点)

---

## Features | 功能特点

###proxy Proxy Management | 代理配置管理

- Add, edit, delete and activate proxy profiles with one click
- 添加、编辑、删除和一键激活代理配置
- Support for multiple proxy configurations
- 支持多个代理配置并存

### 🔧 System Proxy Integration | 系统代理集成

- Directly modifies Windows Registry to set system proxy
- 直接修改 Windows 注册表设置系统代理
- No manual configuration required
- 无需手动配置

### 📦 System Tray | 系统托盘

- Runs in background with system tray icon
- 后台运行，最小化到系统托盘
- Tray menu: Show window | Restore proxy | Quit
- 托盘菜单：显示窗口 | 恢复代理 | 退出

### 📟 Real-time Logs | 实时日志

- Built-in xterm.js terminal for viewing application logs
- 内置 xterm.js 终端查看应用日志
- File-based log storage for debugging
- 文件日志存储，便于问题排查

### 🎨 Custom UI | 自定义界面

- Frameless window with custom titlebar (minimize / maximize / close)
- 无边框窗口，自定义标题栏（最小化/最大化/关闭）
- Beautiful gradient title bar with modern design
- 渐变色标题栏，现代设计风格

### ⚙️ Bypass Rules | 绕过规则

- Configurable proxy bypass list for local addresses
- 可配置代理绕过列表，适用于本地地址
- Default bypass rules cover common private IP ranges
- 默认绕过规则覆盖常用私有 IP 网段

---

## Tech Stack | 技术栈

| Layer | 技术层 | Technology |
|-------|--------|------------|
| Frontend | 前端 | Vue 3 + Composition API |
| UI Library | UI 组件 | Element Plus |
| Terminal | 终端 | xterm.js + xterm-addon-fit |
| Backend | 后端 | Rust |
| Framework | 框架 | Tauri v2 |
| Registry | 注册表 | winreg (Rust) |

---

## Screenshots | 截图

![主界面](http://img.innet.cloud/i/2026/04/23/69e9e75ee80cd.jpg)

![代理配置](http://img.innet.cloud/i/2026/04/23/69e9e760c154f.jpg)

---

## Installation | 安装

### Pre-built Releases | 预编译版本

Download from [Releases](https://github.com/debug-deng/SetProxy/releases):

- `SetProxy_x.x.x_x64-setup.exe` - NSIS installer (recommended)
- `SetProxy_x.x.x_x64.msi` - MSI installer

### Build from Source | 从源码构建

**Requirements | 环境要求：**
- Node.js 18+
- Rust 1.70+
- Windows SDK (for Tauri)

**Build Steps | 构建步骤：**

```bash
# Clone the repository
git clone https://github.com/debug-deng/SetProxy.git
cd SetProxy

# Install frontend dependencies
npm install

# Build the application
npm run tauri build
```

Built executable will be in `src-tauri/target/release/` | 构建完成后的可执行文件位于 `src-tauri/target/release/`

---

## Usage | 使用说明

1. Launch SetProxy | 启动 SetProxy
2. Click "Add" to create a new proxy configuration (name, IP, port) | 点击「添加」创建新的代理配置（名称、IP、端口）
3. Click "Activate" on any configuration to enable it | 点击任意配置的「激活」按钮启用该代理
4. Click "Restore" to disable proxy and restore system defaults | 点击「恢复」按钮禁用代理，恢复系统默认
5. Close the window to minimize to system tray | 关闭窗口会最小化到系统托盘
6. Right-click the tray icon for more options | 右键点击托盘图标查看更多选项

---

## Project Structure | 项目结构

```
SetProxy/
├── src/                        # Vue frontend | Vue 前端源码
│   ├── App.vue                # Main application component | 主应用组件
│   ├── main.js                # Frontend entry point | 前端入口
│   └── assets/                # Static assets | 静态资源
├── src-tauri/                  # Tauri backend (Rust) | Tauri 后端 (Rust)
│   ├── src/
│   │   └── lib.rs             # Main logic | 主要逻辑
│   ├── Cargo.toml             # Rust dependencies | Rust 依赖
│   ├── tauri.conf.json        # Tauri configuration | Tauri 配置
│   └── capabilities/          # App capabilities | 应用权限
├── public/                    # Public static files | 公共静态文件
├── index.html                 # HTML entry | HTML 入口
├── package.json               # Node dependencies | Node 依赖
├── vite.config.js             # Vite configuration | Vite 配置
└── README.md                  # This file | 说明文档
```

---

## Data Storage | 数据存储

| Type | 类型 | Location | 路径 |
|------|------|----------|------|
| Proxy Configs | 代理配置 | `%APPDATA%/SetProxy/proxy_configs.json` | `%APPDATA%/SetProxy/proxy_configs.json` |
| Application Logs | 应用日志 | `%LOCALAPPDATA%/SetProxy/logs/` | `%LOCALAPPDATA%/SetProxy/logs/` |

---

## Contributing | 贡献

Contributions are welcome! Please feel free to submit Issues and Pull Requests.
欢迎贡献！请随时提交 Issues 和 Pull Requests。

---

## Acknowledgments | 致谢

- [Tauri](https://tauri.app/) - Build smaller, faster, and more secure desktop apps
- [Vue.js](https://vuejs.org/) - The progressive JavaScript framework
- [Element Plus](https://element-plus.org/) - A Vue 3 based component library
- [xterm.js](https://xtermjs.org/) - Terminal emulator for web

---

## License | 许可证

**MIT License** - Fully open source, free for commercial use, personal use, modification, and distribution.

**MIT 许可证** - 完全开源，可用于商业用途、个人使用、修改和分发。

```
MIT License

Copyright (c) 2024 SetProxy Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
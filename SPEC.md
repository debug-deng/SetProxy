# SetProxy - Windows 系统代理管理工具

## 1. 项目概述

- **项目名称**: SetProxy
- **项目类型**: Windows 桌面工具 (Tauri + Rust + Vue)
- **核心功能**: 管理多个代理配置，一键切换 Windows 系统代理
- **目标用户**: 需要频繁切换代理的 Windows 用户

## 2. 功能列表

### 2.1 代理配置管理
- 添加代理配置（名称、IP、端口）
- 删除代理配置
- 保存配置到本地 JSON 文件（持久化）

### 2.2 代理切换
- 激活选中的代理配置
- 一键恢复系统代理（关闭自定义代理）
- 激活时自动刷新系统设置（即时生效）

### 2.3 启动状态回显
- 程序启动时读取注册表，显示当前系统实际代理状态

## 3. 技术方案

### 3.1 技术栈
- **前端**: Vue 3 (通过 create-tauri-app 初始化)
- **后端**: Rust + Tauri 1.x
- **关键依赖**:
  - `winreg` - 操作 Windows 注册表
  - `windows-sys` - 调用 `InternetSetOptionW` 刷新系统设置
  - `serde` / `serde_json` - 数据序列化

### 3.2 注册表操作
- 路径: `HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Internet Settings`
- 修改键值: `ProxyEnable` (u32, 0/1), `ProxyServer` (String, `ip:port`)

### 3.3 数据持久化
- 使用 JSON 文件存储在 AppData 目录下
- 文件名: `proxy_configs.json`

## 4. 验收标准

- [ ] 程序可正常启动，窗口显示代理配置列表
- [ ] 可添加新的代理配置
- [ ] 可删除代理配置
- [ ] 点击激活按钮后，系统代理被正确设置
- [ ] 点击恢复按钮后，系统代理被关闭
- [ ] 关闭程序后重新打开，配置数据保持不变
- [ ] 程序启动时正确显示当前系统代理状态
use serde::{Deserialize, Serialize};
use winreg::enums::*;
use winreg::RegKey;

// ============== 数据模型 ==============

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProxyConfig {
    pub id: String,
    pub name: String,
    pub ip: String,
    pub port: u32,
    #[serde(default)]
    pub active: bool,
}

#[derive(Serialize)]
pub struct CommandResult {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct SystemProxyStatus {
    pub enabled: bool,
    pub ip: String,
    pub port: u32,
}

// ============== 代理管理器 ==============

pub struct ProxyManager;

impl ProxyManager {
    const REG_PATH: &'static str = "Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings";

    // 激活指定代理
    pub fn apply_proxy(ip: &str, port: u32) -> Result<(), String> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let key = hkcu.open_subkey_with_flags(Self::REG_PATH, KEY_SET_VALUE)
            .map_err(|e| format!("打开注册表失败: {}", e))?;

        let proxy_server = format!("{}:{}", ip, port);

        key.set_value("ProxyEnable", &1u32)
            .map_err(|e| format!("设置 ProxyEnable 失败: {}", e))?;
        key.set_value("ProxyServer", &proxy_server)
            .map_err(|e| format!("设置 ProxyServer 失败: {}", e))?;

        Ok(())
    }

    // 恢复系统代理（关闭）
    pub fn disable_proxy() -> Result<(), String> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let key = hkcu.open_subkey_with_flags(Self::REG_PATH, KEY_SET_VALUE)
            .map_err(|e| format!("打开注册表失败: {}", e))?;

        key.set_value("ProxyEnable", &0u32)
            .map_err(|e| format!("设置 ProxyEnable 失败: {}", e))?;

        Ok(())
    }

    // 获取当前系统代理状态
    pub fn get_status() -> Result<SystemProxyStatus, String> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let key = hkcu.open_subkey(Self::REG_PATH)
            .map_err(|e| format!("打开注册表失败: {}", e))?;

        let proxy_enable: u32 = key.get_value("ProxyEnable")
            .unwrap_or(0);
        let proxy_server: String = key.get_value("ProxyServer").unwrap_or_default();

        let (ip, port) = if proxy_enable == 1 && !proxy_server.is_empty() {
            let parts: Vec<&str> = proxy_server.split(':').collect();
            let ip = parts.get(0).unwrap_or(&"").to_string();
            let port = parts.get(1).unwrap_or(&"0").parse().unwrap_or(0);
            (ip, port)
        } else {
            (String::new(), 0)
        };

        Ok(SystemProxyStatus {
            enabled: proxy_enable == 1,
            ip,
            port,
        })
    }
}

// ============== 数据持久化 ==============

fn get_config_path() -> Result<std::path::PathBuf, String> {
    let app_data = dirs::config_dir()
        .ok_or("无法获取配置目录".to_string())?;
    let config_dir = app_data.join("SetProxy");
    std::fs::create_dir_all(&config_dir)
        .map_err(|e| format!("创建配置目录失败: {}", e))?;
    Ok(config_dir.join("proxy_configs.json"))
}

fn load_configs() -> Vec<ProxyConfig> {
    let path = match get_config_path() {
        Ok(p) => p,
        Err(_) => return Vec::new(),
    };

    match std::fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

fn save_configs(configs: &[ProxyConfig]) -> Result<(), String> {
    let path = get_config_path()?;
    let content = serde_json::to_string_pretty(configs)
        .map_err(|e| format!("序列化配置失败: {}", e))?;
    std::fs::write(&path, content)
        .map_err(|e| format!("写入配置文件失败: {}", e))?;
    Ok(())
}

// ============== Tauri Commands ==============

#[tauri::command]
fn enable_proxy(ip: String, port: u32) -> CommandResult {
    match ProxyManager::apply_proxy(&ip, port) {
        Ok(_) => CommandResult {
            success: true,
            message: "代理已激活".to_string(),
        },
        Err(e) => CommandResult {
            success: false,
            message: e,
        },
    }
}

#[tauri::command]
fn disable_proxy() -> CommandResult {
    match ProxyManager::disable_proxy() {
        Ok(_) => CommandResult {
            success: true,
            message: "系统代理已恢复".to_string(),
        },
        Err(e) => CommandResult {
            success: false,
            message: e,
        },
    }
}

#[tauri::command]
fn get_system_proxy_status() -> Result<SystemProxyStatus, String> {
    ProxyManager::get_status()
}

#[tauri::command]
fn get_configs() -> Vec<ProxyConfig> {
    load_configs()
}

#[tauri::command]
fn save_config(config: ProxyConfig) -> CommandResult {
    let mut configs = load_configs();

    if let Some(existing) = configs.iter_mut().find(|c| c.id == config.id) {
        *existing = config;
    } else {
        configs.push(config);
    }

    match save_configs(&configs) {
        Ok(_) => CommandResult {
            success: true,
            message: "配置已保存".to_string(),
        },
        Err(e) => CommandResult {
            success: false,
            message: e,
        },
    }
}

#[tauri::command]
fn delete_config(id: String) -> CommandResult {
    let mut configs = load_configs();
    configs.retain(|c| c.id != id);

    match save_configs(&configs) {
        Ok(_) => CommandResult {
            success: true,
            message: "配置已删除".to_string(),
        },
        Err(e) => CommandResult {
            success: false,
            message: e,
        },
    }
}

// ============== 入口点 ==============

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            enable_proxy,
            disable_proxy,
            get_system_proxy_status,
            get_configs,
            save_config,
            delete_config,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
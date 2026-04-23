<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const configs = ref([]);
const currentStatus = ref({ enabled: false, ip: "", port: 0 });

const newConfig = ref({
  name: "",
  ip: "",
  port: "",
});

// 加载配置列表
async function loadConfigs() {
  try {
    configs.value = await invoke("get_configs");
  } catch (e) {
    console.error("加载配置失败:", e);
  }
}

// 获取系统代理状态
async function getSystemStatus() {
  try {
    currentStatus.value = await invoke("get_system_proxy_status");
  } catch (e) {
    console.error("获取系统状态失败:", e);
  }
}

// 激活配置
async function activateConfig(config) {
  try {
    const res = await invoke("enable_proxy", {
      ip: config.ip,
      port: parseInt(config.port),
    });
    if (res.success) {
      configs.value.forEach((c) => (c.active = c.id === config.id));
      await getSystemStatus();
      alert("代理已激活");
    } else {
      alert("激活失败: " + res.message);
    }
  } catch (e) {
    alert("激活失败: " + e);
  }
}

// 恢复系统代理
async function restoreSystem() {
  try {
    const res = await invoke("disable_proxy");
    if (res.success) {
      configs.value.forEach((c) => (c.active = false));
      await getSystemStatus();
      alert("系统代理已恢复");
    } else {
      alert("恢复失败: " + res.message);
    }
  } catch (e) {
    alert("恢复失败: " + e);
  }
}

// 添加配置
async function addConfig() {
  if (!newConfig.value.name || !newConfig.value.ip || !newConfig.value.port) {
    alert("请填写完整信息");
    return;
  }

  const config = {
    id: Date.now().toString(),
    name: newConfig.value.name,
    ip: newConfig.value.ip,
    port: parseInt(newConfig.value.port),
    active: false,
  };

  try {
    const res = await invoke("save_config", { config });
    if (res.success) {
      await loadConfigs();
      newConfig.value = { name: "", ip: "", port: "" };
    } else {
      alert("保存失败: " + res.message);
    }
  } catch (e) {
    alert("保存失败: " + e);
  }
}

// 删除配置
async function deleteConfig(id) {
  if (!confirm("确定要删除此配置吗？")) return;

  try {
    const res = await invoke("delete_config", { id });
    if (res.success) {
      await loadConfigs();
      await getSystemStatus();
    } else {
      alert("删除失败: " + res.message);
    }
  } catch (e) {
    alert("删除失败: " + e);
  }
}

// 启动时加载
onMounted(async () => {
  await loadConfigs();
  await getSystemStatus();
});
</script>

<template>
  <div class="container">
    <!-- 顶部状态栏 -->
    <div class="status-bar">
      <span v-if="currentStatus.enabled" class="status-active">
        当前代理: {{ currentStatus.ip }}:{{ currentStatus.port }}
      </span>
      <span v-else class="status-inactive">系统代理未启用</span>
      <button class="btn-restore" @click="restoreSystem">恢复系统代理</button>
    </div>

    <!-- 添加配置表单 -->
    <div class="form-area">
      <h3>添加代理配置</h3>
      <div class="form-row">
        <input v-model="newConfig.name" placeholder="配置名称" />
        <input v-model="newConfig.ip" placeholder="代理 IP" />
        <input v-model="newConfig.port" placeholder="端口" type="number" />
        <button @click="addConfig">保存</button>
      </div>
    </div>

    <!-- 配置列表 -->
    <div class="list-area">
      <h3>代理配置列表</h3>
      <div v-if="configs.length === 0" class="empty">暂无配置</div>
      <div v-for="config in configs" :key="config.id" class="config-card">
        <div class="config-info">
          <span class="config-name">{{ config.name }}</span>
          <span class="config-address">{{ config.ip }}:{{ config.port }}</span>
        </div>
        <div class="config-actions">
          <button
            class="btn-activate"
            :class="{ active: config.active }"
            @click="activateConfig(config)"
          >
            {{ config.active ? "已激活" : "激活" }}
          </button>
          <button class="btn-delete" @click="deleteConfig(config.id)">
            删除
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
  background-color: #f5f5f5;
  color: #333;
}

.container {
  padding: 20px;
  max-width: 600px;
  margin: 0 auto;
}

/* 状态栏 */
.status-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 15px;
  background: #fff;
  border-radius: 8px;
  margin-bottom: 20px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.status-active {
  color: #4caf50;
  font-weight: 500;
}

.status-inactive {
  color: #999;
}

.btn-restore {
  background: #666;
  color: #fff;
  border: none;
  padding: 8px 16px;
  border-radius: 4px;
  cursor: pointer;
}

.btn-restore:hover {
  background: #555;
}

/* 表单区域 */
.form-area {
  background: #fff;
  padding: 20px;
  border-radius: 8px;
  margin-bottom: 20px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.form-area h3 {
  margin-bottom: 15px;
  font-size: 16px;
}

.form-row {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}

.form-row input {
  flex: 1;
  min-width: 100px;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.form-row button {
  background: #2196f3;
  color: #fff;
  border: none;
  padding: 10px 20px;
  border-radius: 4px;
  cursor: pointer;
}

.form-row button:hover {
  background: #1976d2;
}

/* 列表区域 */
.list-area {
  background: #fff;
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.list-area h3 {
  margin-bottom: 15px;
  font-size: 16px;
}

.empty {
  text-align: center;
  color: #999;
  padding: 30px;
}

.config-card {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 15px;
  border: 1px solid #eee;
  border-radius: 6px;
  margin-bottom: 10px;
}

.config-card:hover {
  border-color: #ddd;
}

.config-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.config-name {
  font-weight: 500;
  color: #333;
}

.config-address {
  font-size: 13px;
  color: #666;
}

.config-actions {
  display: flex;
  gap: 8px;
}

.btn-activate {
  background: #4caf50;
  color: #fff;
  border: none;
  padding: 8px 16px;
  border-radius: 4px;
  cursor: pointer;
}

.btn-activate.active {
  background: #388e3c;
}

.btn-activate:not(.active):hover {
  background: #45a049;
}

.btn-delete {
  background: #f44336;
  color: #fff;
  border: none;
  padding: 8px 16px;
  border-radius: 4px;
  cursor: pointer;
}

.btn-delete:hover {
  background: #d32f2f;
}
</style>
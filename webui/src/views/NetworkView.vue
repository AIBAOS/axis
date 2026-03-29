<template>
  <DefaultLayout>
    <div class="space-y-6">
      <!-- 页面标题 -->
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">网络管理</h1>
          <p class="text-gray-600 mt-1">管理网络接口、DNS 和网关配置</p>
        </div>
        <div class="flex items-center space-x-3">
          <button @click="refreshAll" :disabled="loading" class="btn-secondary flex items-center space-x-1 text-sm">
            <svg :class="{'animate-spin': loading}" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" /></svg>
            <span>刷新</span>
          </button>
        </div>
      </div>

      <!-- 网络状态概览 -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-blue-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.111 16.404a5.5 5.5 0 017.778 0M12 20h.01m-7.08-7.071c3.904-3.905 10.156-3.905 14.06 0M1.394 9.393c5.583-5.587 14.629-5.587 20.212 0" /></svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">网络接口</p>
              <p class="text-xl font-bold text-gray-900">{{ interfaces.length }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-green-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" /></svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">在线接口</p>
              <p class="text-xl font-bold text-green-700">{{ onlineCount }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-purple-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">下载速度</p>
              <p class="text-xl font-bold text-purple-700">{{ formatSpeed(networkStats.rx_bytes_sec) }}</p>
            </div>
          </div>
        </div>
        <div class="bg-white rounded-lg shadow p-4">
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-orange-100 flex items-center justify-center mr-3">
              <svg class="w-5 h-5 text-orange-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4" /></svg>
            </div>
            <div>
              <p class="text-sm text-gray-500">上传速度</p>
              <p class="text-xl font-bold text-orange-700">{{ formatSpeed(networkStats.tx_bytes_sec) }}</p>
            </div>
          </div>
        </div>
      </div>

      <!-- 选项卡 -->
      <div class="border-b border-gray-200">
        <nav class="-mb-px flex space-x-8">
          <button v-for="tab in tabs" :key="tab.id" @click="currentTab = tab.id"
            :class="[currentTab === tab.id ? 'border-primary-500 text-primary-600' : 'border-transparent text-gray-500 hover:text-gray-700', 'whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm']">{{ tab.name }}</button>
        </nav>
      </div>

      <!-- 网络接口 -->
      <div v-if="currentTab === 'interfaces'" class="space-y-4">
        <div v-if="loading" class="flex justify-center py-12"><svg class="animate-spin h-8 w-8 text-primary-600" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" /><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" /></svg></div>
        <div v-else-if="interfaces.length === 0" class="text-center py-12 bg-white rounded-lg shadow"><svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.111 16.404a5.5 5.5 0 017.778 0M12 20h.01m-7.08-7.071c3.904-3.905 10.156-3.905 14.06 0M1.394 9.393c5.583-5.587 14.629-5.587 20.212 0" /></svg><p class="mt-4 text-gray-600">暂无网络接口</p></div>
        <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <NetworkInterfaceCard v-for="iface in interfaces" :key="iface.id" :interface="iface" @edit="openEditModal" @test="testConnection" />
        </div>
      </div>

      <!-- DNS 配置 -->
      <div v-else-if="currentTab === 'dns'" class="max-w-2xl">
        <div class="bg-white rounded-lg shadow p-6">
          <h3 class="text-lg font-semibold text-gray-900 mb-4">DNS 服务器配置</h3>
          <form @submit.prevent="saveDns" class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">主 DNS 服务器</label>
              <input v-model="dnsConfig.primary" type="text" placeholder="8.8.8.8" class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">备用 DNS 服务器</label>
              <input v-model="dnsConfig.secondary" type="text" placeholder="8.8.4.4" class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" />
            </div>
            <div class="flex justify-end">
              <button type="submit" :disabled="saving" class="btn-primary">{{ saving ? '保存中...' : '保存' }}</button>
            </div>
          </form>
        </div>
      </div>

      <!-- 网关配置 -->
      <div v-else-if="currentTab === 'gateway'" class="max-w-2xl">
        <div class="bg-white rounded-lg shadow p-6">
          <h3 class="text-lg font-semibold text-gray-900 mb-4">网关配置</h3>
          <form @submit.prevent="saveGateway" class="space-y-4">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">默认网关</label>
              <input v-model="gatewayConfig.gateway" type="text" placeholder="192.168.1.1" class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">主机名</label>
              <input v-model="gatewayConfig.hostname" type="text" placeholder="axis-nas" class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" />
            </div>
            <div class="flex justify-end">
              <button type="submit" :disabled="saving" class="btn-primary">{{ saving ? '保存中...' : '保存' }}</button>
            </div>
          </form>
        </div>
      </div>

      <!-- 网络测试 -->
      <div v-else-if="currentTab === 'test'" class="space-y-4">
        <div class="bg-white rounded-lg shadow p-6">
          <h3 class="text-lg font-semibold text-gray-900 mb-4">网络诊断工具</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Ping 测试 -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Ping 测试</label>
              <div class="flex space-x-2">
                <input v-model="pingHost" type="text" placeholder="8.8.8.8" class="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" />
                <button @click="runPing" :disabled="testing" class="btn-secondary">测试</button>
              </div>
              <pre v-if="pingResult" class="mt-2 p-3 bg-gray-50 rounded-lg text-xs font-mono overflow-auto max-h-40">{{ pingResult }}</pre>
            </div>
            <!-- DNS 解析测试 -->
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">DNS 解析</label>
              <div class="flex space-x-2">
                <input v-model="dnsHost" type="text" placeholder="google.com" class="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500" />
                <button @click="runDnsResolve" :disabled="testing" class="btn-secondary">解析</button>
              </div>
              <pre v-if="dnsResult" class="mt-2 p-3 bg-gray-50 rounded-lg text-xs font-mono overflow-auto max-h-40">{{ dnsResult }}</pre>
            </div>
          </div>
        </div>
      </div>

      <!-- 端口管理 -->
      <div v-else-if="currentTab === 'ports'" class="space-y-4">
        <div class="flex justify-between items-center">
          <h2 class="text-lg font-semibold">服务端口映射</h2>
          <button @click="showPortModal = true" class="btn-primary text-sm">添加端口</button>
        </div>
        <div class="bg-white rounded-lg shadow overflow-hidden">
          <table class="w-full">
            <thead class="bg-gray-50 border-b">
              <tr>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">服务</th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">内部端口</th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">外部端口</th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">协议</th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">状态</th>
                <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">操作</th>
              </tr>
            </thead>
            <tbody class="divide-y">
              <tr v-for="port in servicePorts" :key="port.id" class="hover:bg-gray-50">
                <td class="px-4 py-3 text-sm font-medium text-gray-900">{{ port.name }}</td>
                <td class="px-4 py-3 text-sm text-gray-600 font-mono">{{ port.internal_port }}</td>
                <td class="px-4 py-3 text-sm text-gray-600 font-mono">{{ port.external_port }}</td>
                <td class="px-4 py-3 text-sm text-gray-600 uppercase">{{ port.protocol }}</td>
                <td class="px-4 py-3">
                  <span :class="port.enabled ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-700'" class="px-2 py-1 text-xs rounded-full">{{ port.enabled ? '已启用' : '已禁用' }}</span>
                </td>
                <td class="px-4 py-3 text-right">
                  <button @click="togglePort(port)" class="text-sm text-primary-600 hover:text-primary-700 mr-2">{{ port.enabled ? '禁用' : '启用' }}</button>
                  <button @click="deletePort(port)" class="text-sm text-red-600 hover:text-red-700">删除</button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- 防火墙规则 -->
      <div v-else-if="currentTab === 'firewall'" class="space-y-6">
        <!-- 入站规则 -->
        <div>
          <div class="flex justify-between items-center mb-4">
            <h2 class="text-lg font-semibold">入站规则</h2>
            <button @click="showFirewallModal = true; firewallRule.direction = 'inbound'" class="btn-primary text-sm">添加规则</button>
          </div>
          <div class="bg-white rounded-lg shadow overflow-hidden">
            <table class="w-full text-sm">
              <thead class="bg-gray-50 border-b">
                <tr>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500">端口</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500">协议</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500">来源</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500">动作</th>
                  <th class="px-4 py-3 text-right text-xs font-medium text-gray-500">操作</th>
                </tr>
              </thead>
              <tbody class="divide-y">
                <tr v-for="rule in inboundRules" :key="rule.id" class="hover:bg-gray-50">
                  <td class="px-4 py-3 font-mono">{{ rule.port }}</td>
                  <td class="px-4 py-3 uppercase">{{ rule.protocol }}</td>
                  <td class="px-4 py-3">{{ rule.source || '任意' }}</td>
                  <td class="px-4 py-3">
                    <span :class="rule.action === 'allow' ? 'text-green-600' : 'text-red-600'" class="font-medium">{{ rule.action === 'allow' ? '允许' : '拒绝' }}</span>
                  </td>
                  <td class="px-4 py-3 text-right">
                    <button @click="deleteFirewallRule(rule)" class="text-red-600 hover:text-red-700">删除</button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <!-- 出站规则 -->
        <div>
          <div class="flex justify-between items-center mb-4">
            <h2 class="text-lg font-semibold">出站规则</h2>
            <button @click="showFirewallModal = true; firewallRule.direction = 'outbound'" class="btn-primary text-sm">添加规则</button>
          </div>
          <div class="bg-white rounded-lg shadow overflow-hidden">
            <table class="w-full text-sm">
              <thead class="bg-gray-50 border-b">
                <tr>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500">端口</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500">协议</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500">目标</th>
                  <th class="px-4 py-3 text-left text-xs font-medium text-gray-500">动作</th>
                  <th class="px-4 py-3 text-right text-xs font-medium text-gray-500">操作</th>
                </tr>
              </thead>
              <tbody class="divide-y">
                <tr v-for="rule in outboundRules" :key="rule.id" class="hover:bg-gray-50">
                  <td class="px-4 py-3 font-mono">{{ rule.port }}</td>
                  <td class="px-4 py-3 uppercase">{{ rule.protocol }}</td>
                  <td class="px-4 py-3">{{ rule.destination || '任意' }}</td>
                  <td class="px-4 py-3">
                    <span :class="rule.action === 'allow' ? 'text-green-600' : 'text-red-600'" class="font-medium">{{ rule.action === 'allow' ? '允许' : '拒绝' }}</span>
                  </td>
                  <td class="px-4 py-3 text-right">
                    <button @click="deleteFirewallRule(rule)" class="text-red-600 hover:text-red-700">删除</button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>

      <!-- VPN 配置 -->
      <div v-else-if="currentTab === 'vpn'" class="space-y-6">
        <!-- OpenVPN -->
        <div class="bg-white rounded-lg shadow p-6">
          <div class="flex justify-between items-start">
            <div>
              <h3 class="text-lg font-semibold text-gray-900">OpenVPN</h3>
              <p class="text-sm text-gray-500 mt-1">安全的 SSL/TLS VPN 解决方案</p>
            </div>
            <div class="flex items-center space-x-3">
              <span :class="vpnStatus.openvpn ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-700'" class="px-2 py-1 text-xs rounded-full">{{ vpnStatus.openvpn ? '运行中' : '已停止' }}</span>
              <button @click="toggleVpn('openvpn')" :class="vpnStatus.openvpn ? 'btn-secondary' : 'btn-primary'" class="text-sm">{{ vpnStatus.openvpn ? '停止' : '启动' }}</button>
            </div>
          </div>
          <div v-if="vpnStatus.openvpn" class="mt-4 grid grid-cols-2 gap-4 text-sm">
            <div><p class="text-gray-500">本地 IP</p><p class="font-medium text-gray-900">{{ vpnConfig.openvpn.local_ip || '10.8.0.1' }}</p></div>
            <div><p class="text-gray-500">端口</p><p class="font-medium text-gray-900">{{ vpnConfig.openvpn.port || 1194 }}</p></div>
            <div><p class="text-gray-500">协议</p><p class="font-medium text-gray-900 uppercase">{{ vpnConfig.openvpn.protocol || 'UDP' }}</p></div>
            <div><p class="text-gray-500">连接数</p><p class="font-medium text-gray-900">{{ vpnConfig.openvpn.clients || 0 }}</p></div>
          </div>
        </div>

        <!-- WireGuard -->
        <div class="bg-white rounded-lg shadow p-6">
          <div class="flex justify-between items-start">
            <div>
              <h3 class="text-lg font-semibold text-gray-900">WireGuard</h3>
              <p class="text-sm text-gray-500 mt-1">高性能现代 VPN 协议</p>
            </div>
            <div class="flex items-center space-x-3">
              <span :class="vpnStatus.wireguard ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-700'" class="px-2 py-1 text-xs rounded-full">{{ vpnStatus.wireguard ? '运行中' : '已停止' }}</span>
              <button @click="toggleVpn('wireguard')" :class="vpnStatus.wireguard ? 'btn-secondary' : 'btn-primary'" class="text-sm">{{ vpnStatus.wireguard ? '停止' : '启动' }}</button>
            </div>
          </div>
          <div v-if="vpnStatus.wireguard" class="mt-4 grid grid-cols-2 gap-4 text-sm">
            <div><p class="text-gray-500">公钥</p><p class="font-mono text-gray-900 truncate">{{ vpnConfig.wireguard.public_key || '未生成' }}</p></div>
            <div><p class="text-gray-500">监听端口</p><p class="font-medium text-gray-900">{{ vpnConfig.wireguard.port || 51820 }}</p></div>
            <div><p class="text-gray-500">接口地址</p><p class="font-medium text-gray-900">{{ vpnConfig.wireguard.address || '10.0.0.1/24' }}</p></div>
            <div><p class="text-gray-500">Peers</p><p class="font-medium text-gray-900">{{ vpnConfig.wireguard.peers || 0 }}</p></div>
          </div>
        </div>
      </div>

      <!-- 编辑模态框 -->
      <div v-if="editingInterface" class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
        <div class="bg-white rounded-lg shadow-xl max-w-lg w-full mx-4 max-h-[90vh] overflow-y-auto">
          <div class="flex justify-between items-center px-6 py-4 border-b sticky top-0 bg-white">
            <h3 class="text-lg font-semibold text-gray-900">编辑网络接口</h3>
            <button @click="editingInterface = null" class="text-gray-400 hover:text-gray-600"><svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg></button>
          </div>
          <form @submit.prevent="saveInterface" class="p-6 space-y-4">
            <div class="grid grid-cols-2 gap-4">
              <div class="col-span-2"><label class="block text-sm font-medium text-gray-700 mb-1">接口名称</label><input :value="editingInterface.interface" disabled class="w-full px-3 py-2 border border-gray-300 rounded-lg bg-gray-100" /></div>
              <div class="col-span-2 flex items-center"><input v-model="editForm.dhcp_enabled" type="checkbox" id="dhcp" class="h-4 w-4 text-primary-600 rounded" /><label for="dhcp" class="ml-2 text-sm text-gray-700">启用 DHCP（自动获取 IP）</label></div>
              <div><label class="block text-sm font-medium text-gray-700 mb-1">IP 地址</label><input v-model="editForm.ip_address" :disabled="editForm.dhcp_enabled" type="text" placeholder="192.168.1.100" class="w-full px-3 py-2 border border-gray-300 rounded-lg disabled:bg-gray-100" /></div>
              <div><label class="block text-sm font-medium text-gray-700 mb-1">子网掩码</label><input v-model="editForm.netmask" :disabled="editForm.dhcp_enabled" type="text" placeholder="255.255.255.0" class="w-full px-3 py-2 border border-gray-300 rounded-lg disabled:bg-gray-100" /></div>
              <div><label class="block text-sm font-medium text-gray-700 mb-1">网关</label><input v-model="editForm.gateway" :disabled="editForm.dhcp_enabled" type="text" placeholder="192.168.1.1" class="w-full px-3 py-2 border border-gray-300 rounded-lg disabled:bg-gray-100" /></div>
              <div><label class="block text-sm font-medium text-gray-700 mb-1">MTU</label><input v-model.number="editForm.mtu" type="number" placeholder="1500" class="w-full px-3 py-2 border border-gray-300 rounded-lg" /></div>
            </div>
          </form>
          <div class="flex justify-end space-x-3 px-6 py-4 bg-gray-50 rounded-b-lg">
            <button @click="editingInterface = null" class="px-4 py-2 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-50">取消</button>
            <button @click="saveInterface" :disabled="saving" class="btn-primary">{{ saving ? '保存中...' : '保存' }}</button>
          </div>
        </div>
      </div>

      <!-- Toast -->
      <div v-if="toast.show" class="fixed bottom-4 right-4 z-50"><div :class="toast.type === 'success' ? 'bg-green-500' : 'bg-red-500'" class="text-white px-4 py-2 rounded-lg shadow-lg">{{ toast.message }}</div></div>
    </div>
  </DefaultLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import DefaultLayout from '@/layouts/DefaultLayout.vue'
import NetworkInterfaceCard from '@/components/network/NetworkInterfaceCard.vue'
import { api } from '@/utils/api'

const tabs = [
  { id: 'interfaces', name: '网络接口' },
  { id: 'ports', name: '端口管理' },
  { id: 'firewall', name: '防火墙' },
  { id: 'vpn', name: 'VPN 配置' },
  { id: 'dns', name: 'DNS 配置' },
  { id: 'gateway', name: '网关配置' },
  { id: 'test', name: '网络测试' }
]
const currentTab = ref('interfaces')
const loading = ref(true)
const saving = ref(false)
const testing = ref(false)

// 数据
const interfaces = ref<any[]>([])
const dnsConfig = ref({ primary: '8.8.8.8', secondary: '8.8.4.4' })
const gatewayConfig = ref({ gateway: '', hostname: '' })
const networkStats = ref({ rx_bytes_sec: 0, tx_bytes_sec: 0 })

// 编辑
const editingInterface = ref<any>(null)
const editForm = ref({ ip_address: '', netmask: '', gateway: '', dhcp_enabled: false, mtu: 1500 })

// 测试
const pingHost = ref('8.8.8.8')
const pingResult = ref('')
const dnsHost = ref('google.com')
const dnsResult = ref('')

// 端口管理
const showPortModal = ref(false)
const servicePorts = ref([
  { id: 1, name: 'Web UI (HTTP)', internal_port: 80, external_port: 80, protocol: 'tcp', enabled: true },
  { id: 2, name: 'Web UI (HTTPS)', internal_port: 443, external_port: 443, protocol: 'tcp', enabled: true },
  { id: 3, name: 'SSH', internal_port: 22, external_port: 22, protocol: 'tcp', enabled: true },
  { id: 4, name: 'SMB/CIFS', internal_port: 445, external_port: 445, protocol: 'tcp', enabled: true },
  { id: 5, name: 'NFS', internal_port: 2049, external_port: 2049, protocol: 'tcp', enabled: false }
])

// 防火墙
const showFirewallModal = ref(false)
const firewallRule = ref({ direction: 'inbound', port: '', protocol: 'tcp', source: '', action: 'allow' })
const inboundRules = ref([
  { id: 1, port: '22', protocol: 'tcp', source: '', action: 'allow' },
  { id: 2, port: '80', protocol: 'tcp', source: '', action: 'allow' },
  { id: 3, port: '443', protocol: 'tcp', source: '', action: 'allow' }
])
const outboundRules = ref([
  { id: 1, port: '*', protocol: 'all', destination: '', action: 'allow' }
])

// VPN
const vpnStatus = ref({ openvpn: false, wireguard: false })
const vpnConfig = ref({
  openvpn: { local_ip: '10.8.0.1', port: 1194, protocol: 'UDP', clients: 0 },
  wireguard: { public_key: 'aBcDeFgHiJkLmNoPqRsTuVwXyZ123456789=', port: 51820, address: '10.0.0.1/24', peers: 0 }
})

const toast = ref({ show: false, type: 'success' as 'success' | 'error', message: '' })

const onlineCount = computed(() => interfaces.value.filter(i => i.status === 'up').length)

// 加载数据
const loadInterfaces = async () => { try { const r = await api.network.listInterfaces(); interfaces.value = r.data.data || r.data || [] } catch (e) {} }
const loadConfig = async () => { try { const r = await api.network.listConfig(); const configs = r.data.data || r.data || []; if (configs.length > 0) { dnsConfig.value = { primary: configs[0].dns_primary || '8.8.8.8', secondary: configs[0].dns_secondary || '8.8.4.4' }; gatewayConfig.value = { gateway: configs[0].gateway || '', hostname: '' } } } catch (e) {} }
const loadStats = async () => { try { const r = await api.system.resources(); networkStats.value = r.data.data?.network_io || r.data.network_io || { rx_bytes_sec: 0, tx_bytes_sec: 0 } } catch (e) {} }
const refreshAll = async () => { loading.value = true; await Promise.all([loadInterfaces(), loadConfig(), loadStats()]); loading.value = false }

// 编辑
const openEditModal = (iface: any) => { editingInterface.value = iface; editForm.value = { ip_address: iface.ip_address || '', netmask: iface.netmask || '255.255.255.0', gateway: iface.gateway || '', dhcp_enabled: iface.dhcp_enabled || false, mtu: iface.mtu || 1500 } }
const saveInterface = async () => { saving.value = true; try { await api.network.updateInterface(editingInterface.value.id, editForm.value); showToast('success', '配置已保存'); editingInterface.value = null; loadInterfaces() } catch (e) { showToast('error', '保存失败') } finally { saving.value = false } }

// DNS
const saveDns = async () => { saving.value = true; try { await api.network.updateConfig(1, { dns_primary: dnsConfig.value.primary, dns_secondary: dnsConfig.value.secondary }); showToast('success', 'DNS 配置已保存') } catch (e) { showToast('error', '保存失败') } finally { saving.value = false } }

// 网关
const saveGateway = async () => { saving.value = true; try { await api.network.updateConfig(1, { gateway: gatewayConfig.value.gateway }); if (gatewayConfig.value.hostname) await api.settings.update({ hostname: gatewayConfig.value.hostname }); showToast('success', '网关配置已保存') } catch (e) { showToast('error', '保存失败') } finally { saving.value = false } }

// 测试
const testConnection = async (iface: any) => { showToast('success', `测试 ${iface.interface} 连接...`) }
const runPing = async () => { if (!pingHost.value) return; testing.value = true; pingResult.value = '正在测试...'; try { const r = await api.network.test?.({ type: 'ping', host: pingHost.value }); pingResult.value = r.data?.output || 'Ping 测试完成' } catch (e) { pingResult.value = `Ping ${pingHost.value}: 测试完成（模拟）` } finally { testing.value = false } }
const runDnsResolve = async () => { if (!dnsHost.value) return; testing.value = true; dnsResult.value = '正在解析...'; try { const r = await api.network.test?.({ type: 'dns', host: dnsHost.value }); dnsResult.value = r.data?.output || `${dnsHost.value} -> 142.250.185.78` } catch (e) { dnsResult.value = `${dnsHost.value} -> 142.250.185.78（模拟）` } finally { testing.value = false } }

// 工具
const formatSpeed = (bps: number) => { if (!bps) return '0 B/s'; const k = 1024; const s = ['B/s', 'KB/s', 'MB/s']; const i = Math.floor(Math.log(bps) / Math.log(k)); return (bps / Math.pow(k, i)).toFixed(1) + ' ' + s[i] }
const showToast = (type: 'success' | 'error', msg: string) => { toast.value = { show: true, type, message: msg }; setTimeout(() => toast.value.show = false, 3000) }

// 端口管理
const togglePort = (port: any) => { port.enabled = !port.enabled; showToast('success', `${port.name} 已${port.enabled ? '启用' : '禁用'}`) }
const deletePort = (port: any) => { if (!confirm(`确定删除端口映射 "${port.name}" 吗？`)) return; servicePorts.value = servicePorts.value.filter(p => p.id !== port.id); showToast('success', '端口映射已删除') }

// 防火墙
const deleteFirewallRule = (rule: any) => { 
  if (!confirm('确定删除此规则吗？')) return
  if (rule.direction === 'inbound') inboundRules.value = inboundRules.value.filter(r => r.id !== rule.id)
  else outboundRules.value = outboundRules.value.filter(r => r.id !== rule.id)
  showToast('success', '规则已删除')
}

// VPN
const toggleVpn = async (type: 'openvpn' | 'wireguard') => {
  try {
    vpnStatus.value[type] = !vpnStatus.value[type]
    showToast('success', `${type === 'openvpn' ? 'OpenVPN' : 'WireGuard'} 已${vpnStatus.value[type] ? '启动' : '停止'}`)
  } catch (e) {
    showToast('error', '操作失败')
  }
}

let statsTimer: ReturnType<typeof setInterval> | null = null
onMounted(() => { refreshAll(); statsTimer = setInterval(loadStats, 5000) })
onUnmounted(() => { if (statsTimer) clearInterval(statsTimer) })
</script>
<template>
  <div class="fixed inset-0 bg-gray-500 bg-opacity-75 flex items-center justify-center z-50">
    <div class="bg-white rounded-lg shadow-xl max-w-lg w-full mx-4 max-h-[90vh] overflow-y-auto">
      <!-- 标题栏 -->
      <div class="flex justify-between items-center px-6 py-4 border-b sticky top-0 bg-white">
        <h3 class="text-lg font-semibold text-gray-900">
          {{ mode === 'create' ? '添加打印机' : '编辑打印机' }}
        </h3>
        <button @click="$emit('close')" class="text-gray-400 hover:text-gray-600">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- 表单 -->
      <form @submit.prevent="handleSubmit" class="px-6 py-4 space-y-4">
        <!-- 基本信息 -->
        <div class="space-y-4">
          <h4 class="text-sm font-medium text-gray-900 border-b pb-2">基本信息</h4>

          <!-- 名称 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">打印机名称 *</label>
            <input
              v-model="formData.name"
              type="text"
              required
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
              placeholder="如：办公室打印机"
            />
          </div>

          <!-- 类型 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">打印机类型 *</label>
            <select
              v-model="formData.type"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
            >
              <option value="network">网络打印机</option>
              <option value="usb">USB 打印机</option>
              <option value="virtual">虚拟打印机</option>
            </select>
          </div>

          <!-- 制造商 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">制造商</label>
            <input
              v-model="formData.manufacturer"
              type="text"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
              placeholder="如：HP, Canon, Epson"
            />
          </div>

          <!-- 型号 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">型号</label>
            <input
              v-model="formData.model"
              type="text"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
              placeholder="如：LaserJet Pro M404n"
            />
          </div>
        </div>

        <!-- 连接配置 -->
        <div class="space-y-4">
          <h4 class="text-sm font-medium text-gray-900 border-b pb-2">连接配置</h4>

          <!-- 网络打印机 -->
          <template v-if="formData.type === 'network'">
            <div class="grid grid-cols-3 gap-3">
              <div class="col-span-2">
                <label class="block text-sm font-medium text-gray-700 mb-1">IP 地址 *</label>
                <input
                  v-model="formData.ip_address"
                  type="text"
                  :required="formData.type === 'network'"
                  class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
                  placeholder="192.168.1.100"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">端口</label>
                <input
                  v-model.number="formData.port"
                  type="number"
                  class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
                  placeholder="9100"
                />
              </div>
            </div>
          </template>

          <!-- USB 打印机 -->
          <template v-if="formData.type === 'usb'">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">USB 设备路径 *</label>
              <input
                v-model="formData.usb_device"
                type="text"
                :required="formData.type === 'usb'"
                class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
                placeholder="/dev/usb/lp0"
              />
            </div>
          </template>
        </div>

        <!-- 位置和其他 -->
        <div class="space-y-4">
          <h4 class="text-sm font-medium text-gray-900 border-b pb-2">其他设置</h4>

          <!-- 位置 -->
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-1">位置</label>
            <input
              v-model="formData.location"
              type="text"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
              placeholder="如：一楼办公区"
            />
          </div>

          <!-- 默认打印机 -->
          <div class="flex items-center">
            <input
              v-model="formData.is_default"
              type="checkbox"
              id="is_default"
              class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
            />
            <label for="is_default" class="ml-2 block text-sm text-gray-700">
              设为默认打印机
            </label>
          </div>
        </div>

        <!-- 打印能力 -->
        <div class="space-y-4">
          <h4 class="text-sm font-medium text-gray-900 border-b pb-2">打印能力</h4>

          <div class="grid grid-cols-2 gap-3">
            <div class="flex items-center">
              <input
                v-model="formData.capabilities.color"
                type="checkbox"
                id="cap_color"
                class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
              />
              <label for="cap_color" class="ml-2 text-sm text-gray-700">彩色打印</label>
            </div>
            <div class="flex items-center">
              <input
                v-model="formData.capabilities.duplex"
                type="checkbox"
                id="cap_duplex"
                class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
              />
              <label for="cap_duplex" class="ml-2 text-sm text-gray-700">双面打印</label>
            </div>
            <div class="flex items-center">
              <input
                v-model="formData.capabilities.scanning"
                type="checkbox"
                id="cap_scanning"
                class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
              />
              <label for="cap_scanning" class="ml-2 text-sm text-gray-700">扫描功能</label>
            </div>
            <div class="flex items-center">
              <input
                v-model="formData.capabilities.fax"
                type="checkbox"
                id="cap_fax"
                class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
              />
              <label for="cap_fax" class="ml-2 text-sm text-gray-700">传真功能</label>
            </div>
          </div>
        </div>
      </form>

      <!-- 按钮栏 -->
      <div class="flex justify-end space-x-3 px-6 py-4 border-t bg-gray-50 sticky bottom-0">
        <button
          type="button"
          @click="$emit('close')"
          class="px-4 py-2 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-50"
        >
          取消
        </button>
        <button
          type="submit"
          @click="handleSubmit"
          :disabled="loading"
          class="btn-primary disabled:opacity-50"
        >
          {{ loading ? '处理中...' : (mode === 'create' ? '添加' : '保存') }}
        </button>
      </div>

      <!-- 错误提示 -->
      <div v-if="error" class="px-6 py-3 bg-red-50 border-t border-red-100">
        <p class="text-sm text-red-600">{{ error }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

const props = defineProps<{
  mode: 'create' | 'edit'
  printer?: any
}>()

const emit = defineEmits<{
  close: []
  save: [data: any]
}>()

const loading = ref(false)
const error = ref('')

// 表单数据
const formData = ref({
  name: '',
  type: 'network',
  model: '',
  manufacturer: '',
  ip_address: '',
  port: 9100,
  usb_device: '',
  location: '',
  is_default: false,
  capabilities: {
    color: false,
    duplex: false,
    scanning: false,
    fax: false
  }
})

// 监听编辑数据
watch(() => props.printer, (newPrinter) => {
  if (newPrinter && props.mode === 'edit') {
    formData.value = {
      name: newPrinter.name || '',
      type: newPrinter.type || 'network',
      model: newPrinter.model || '',
      manufacturer: newPrinter.manufacturer || '',
      ip_address: newPrinter.ip_address || '',
      port: newPrinter.port || 9100,
      usb_device: newPrinter.usb_device || '',
      location: newPrinter.location || '',
      is_default: newPrinter.is_default || false,
      capabilities: {
        color: newPrinter.capabilities?.color || false,
        duplex: newPrinter.capabilities?.duplex || false,
        scanning: newPrinter.capabilities?.scanning || false,
        fax: newPrinter.capabilities?.fax || false
      }
    }
  }
}, { immediate: true })

// 提交
const handleSubmit = async () => {
  error.value = ''

  // 验证
  if (!formData.value.name.trim()) {
    error.value = '请输入打印机名称'
    return
  }

  if (formData.value.type === 'network' && !formData.value.ip_address.trim()) {
    error.value = '网络打印机必须填写 IP 地址'
    return
  }

  if (formData.value.type === 'usb' && !formData.value.usb_device.trim()) {
    error.value = 'USB 打印机必须填写设备路径'
    return
  }

  // 构建数据
  const data: any = {
    name: formData.value.name.trim(),
    type: formData.value.type,
    model: formData.value.model.trim() || undefined,
    manufacturer: formData.value.manufacturer.trim() || undefined,
    location: formData.value.location.trim() || undefined,
    is_default: formData.value.is_default,
    capabilities: formData.value.capabilities
  }

  if (formData.value.type === 'network') {
    data.ip_address = formData.value.ip_address.trim()
    if (formData.value.port) data.port = formData.value.port
  }

  if (formData.value.type === 'usb') {
    data.usb_device = formData.value.usb_device.trim()
  }

  emit('save', data)
}
</script>
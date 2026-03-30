/**
 * 表单验证工具函数
 * 与后端 Bug #45-#47 修复保持一致
 */

export interface ValidationResult {
  valid: boolean
  error?: string
}

/**
 * 验证用户名
 * 规则：3-50 字符，只允许字母、数字、下划线、连字符
 */
export function validateUsername(username: string): ValidationResult {
  if (!username || username.length === 0) {
    return { valid: false, error: '用户名不能为空' }
  }
  
  if (username.length < 3) {
    return { valid: false, error: '用户名至少需要 3 个字符' }
  }
  
  if (username.length > 50) {
    return { valid: false, error: '用户名不能超过 50 个字符' }
  }
  
  // 只允许字母、数字、下划线、连字符
  const pattern = /^[a-zA-Z0-9_-]+$/
  if (!pattern.test(username)) {
    return { valid: false, error: '用户名只能包含字母、数字、下划线和连字符' }
  }
  
  return { valid: true }
}

/**
 * 验证密码强度
 * 规则：8-128 字符，至少包含大写、小写、数字
 */
export function validatePassword(password: string): ValidationResult {
  if (!password || password.length === 0) {
    return { valid: false, error: '密码不能为空' }
  }
  
  if (password.length < 8) {
    return { valid: false, error: '密码至少需要 8 个字符' }
  }
  
  if (password.length > 128) {
    return { valid: false, error: '密码不能超过 128 个字符' }
  }
  
  const hasUppercase = /[A-Z]/.test(password)
  const hasLowercase = /[a-z]/.test(password)
  const hasDigit = /[0-9]/.test(password)
  
  if (!hasUppercase) {
    return { valid: false, error: '密码需要包含至少一个大写字母' }
  }
  
  if (!hasLowercase) {
    return { valid: false, error: '密码需要包含至少一个小写字母' }
  }
  
  if (!hasDigit) {
    return { valid: false, error: '密码需要包含至少一个数字' }
  }
  
  return { valid: true }
}

/**
 * 验证邮箱格式
 * 规则：标准邮箱格式，最大 254 字符
 */
export function validateEmail(email: string): ValidationResult {
  if (!email || email.length === 0) {
    return { valid: false, error: '邮箱不能为空' }
  }
  
  if (email.length > 254) {
    return { valid: false, error: '邮箱长度不能超过 254 个字符' }
  }
  
  const pattern = /^[a-zA-Z0-9]([a-zA-Z0-9._%+-]*[a-zA-Z0-9])?@[a-zA-Z0-9]([a-zA-Z0-9.-]*[a-zA-Z0-9])?\.[a-zA-Z]{2,}$/
  if (!pattern.test(email)) {
    return { valid: false, error: '邮箱格式不正确' }
  }
  
  return { valid: true }
}

/**
 * 验证存储池名称
 * 规则：1-100 字符，只允许字母、数字、空格、下划线、连字符
 * 禁止路径遍历字符 (.. / \)
 */
export function validatePoolName(name: string): ValidationResult {
  if (!name || name.length === 0) {
    return { valid: false, error: '名称不能为空' }
  }
  
  if (name.length > 100) {
    return { valid: false, error: '名称不能超过 100 个字符' }
  }
  
  // 禁止路径遍历字符
  if (name.includes('..') || name.includes('/') || name.includes('\\')) {
    return { valid: false, error: '名称包含非法字符' }
  }
  
  // 只允许字母、数字、空格、下划线、连字符
  const pattern = /^[a-zA-Z0-9_\- ]+$/
  if (!pattern.test(name)) {
    return { valid: false, error: '名称只能包含字母、数字、空格、下划线和连字符' }
  }
  
  return { valid: true }
}

/**
 * 验证卷名称
 * 规则：1-64 字符，只允许字母、数字、下划线、连字符
 */
export function validateVolumeName(name: string): ValidationResult {
  if (!name || name.length === 0) {
    return { valid: false, error: '名称不能为空' }
  }
  
  if (name.length > 64) {
    return { valid: false, error: '名称不能超过 64 个字符' }
  }
  
  // 只允许字母、数字、下划线、连字符
  const pattern = /^[a-zA-Z0-9_-]+$/
  if (!pattern.test(name)) {
    return { valid: false, error: '名称只能包含字母、数字、下划线和连字符' }
  }
  
  return { valid: true }
}

/**
 * 验证文件名
 * 规则：不为空，禁止路径分隔符、路径遍历、控制字符
 */
export function validateFilename(filename: string): ValidationResult {
  if (!filename || filename.length === 0) {
    return { valid: false, error: '文件名不能为空' }
  }
  
  if (filename.length > 255) {
    return { valid: false, error: '文件名不能超过 255 个字符' }
  }
  
  // 禁止路径分隔符
  if (filename.includes('/') || filename.includes('\\')) {
    return { valid: false, error: '文件名不能包含路径分隔符' }
  }
  
  // 禁止路径遍历
  if (filename.includes('..')) {
    return { valid: false, error: '文件名不能包含 ".."' }
  }
  
  // 禁止控制字符
  if (/[\x00-\x1f]/.test(filename)) {
    return { valid: false, error: '文件名不能包含控制字符' }
  }
  
  return { valid: true }
}

/**
 * 验证共享名称
 * 规则：1-64 字符，只允许字母、数字、下划线、连字符、点
 */
export function validateShareName(name: string): ValidationResult {
  if (!name || name.length === 0) {
    return { valid: false, error: '名称不能为空' }
  }
  
  if (name.length > 64) {
    return { valid: false, error: '名称不能超过 64 个字符' }
  }
  
  const pattern = /^[a-zA-Z0-9_.-]+$/
  if (!pattern.test(name)) {
    return { valid: false, error: '名称只能包含字母、数字、下划线、连字符和点' }
  }
  
  return { valid: true }
}

/**
 * 实时验证辅助函数
 * 在输入时调用，返回错误信息或空字符串
 */
export function getFieldError(value: string, validator: (v: string) => ValidationResult): string {
  const result = validator(value)
  return result.valid ? '' : (result.error || '')
}
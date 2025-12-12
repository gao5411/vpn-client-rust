import { createI18n } from 'vue-i18n'

const messages = {
  en: {
    title: 'Internal Network VPN',
    userId: 'User ID',
    password: 'Password',
    enterUserId: 'Enter User ID',
    enterPassword: 'Enter Password',
    connect: 'Connect VPN',
    connecting: 'Connecting...',
    status: 'Status',
    disconnected: 'Disconnected',
    connected: 'Connected',
    failed: 'Failed',
    activityLog: 'Activity Log',
    errors: {
      missingCredentials: 'Error: Please enter User ID and Password',
      connectionFailed: 'Connection Failed'
    },
    logs: {
      starting: 'Starting VPN connection...',
      checkingKeys: 'Checking WireGuard keys...',
      keysGenerated: 'Keys generated. Public Key: {key}...',
      authenticating: 'Authenticating...',
      authSuccess: 'Authentication successful. Token obtained.',
      initiating: 'Initiating connection sequence...',
      success: 'Success: {msg}'
    }
  },
  zh: {
    title: '内网 VPN 客户端',
    userId: '用户 ID',
    password: '密码',
    enterUserId: '请输入用户 ID',
    enterPassword: '请输入密码',
    connect: '连接 VPN',
    connecting: '正在连接...',
    status: '当前状态',
    disconnected: '未连接',
    connected: '已连接',
    failed: '连接失败',
    activityLog: '活动日志',
    errors: {
      missingCredentials: '错误：请输入用户 ID 和密码',
      connectionFailed: '连接失败'
    },
    logs: {
      starting: '正在启动 VPN 连接...',
      checkingKeys: '正在检查 WireGuard 密钥...',
      keysGenerated: '密钥已生成。公钥：{key}...',
      authenticating: '正在认证...',
      authSuccess: '认证成功。已获取令牌。',
      initiating: '正在初始化连接序列...',
      success: '成功：{msg}'
    }
  }
}

const i18n = createI18n({
  legacy: false, // use Composition API
  locale: 'zh', // default locale
  fallbackLocale: 'en',
  messages
})

export default i18n

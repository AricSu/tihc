import { createI18n } from 'vue-i18n'
// @ts-expect-error: TypeScript JSON import
import zh from './locales/zh.json'
// @ts-expect-error: TypeScript JSON import
import en from './locales/en.json'

const i18n = createI18n({
  legacy: false,
  locale: 'zh',
  fallbackLocale: 'en',
  messages: {
    zh,
    en
  }
})

export default i18n

import { defineStore } from 'pinia'

export const useDdlPrecheckStore = defineStore('ddlPrecheck', {
  state: () => ({
    createDatabase: '',
    createTable: '',
    alterTable: '',
    collationEnabled: true
  }),
  actions: {
    setForm(data) {
      Object.assign(this, data)
    },
    clearForm() {
      this.createDatabase = ''
      this.createTable = ''
      this.alterTable = ''
      this.collationEnabled = true
    }
  },
  persist: true
})

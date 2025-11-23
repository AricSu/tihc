/* global defineBackground, browser */

export default defineBackground(() => {
  // use allowed console methods (warn/error) to satisfy lint rules
  console.warn('Hello background!', { id: browser?.runtime?.id })
  browser.action.onClicked.addListener(() => {
    browser.sidePanel.setPanelBehavior({ openPanelOnActionClick: true }).catch(error => console.error(error))
  })
})

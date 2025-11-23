/* global defineContentScript */

export default defineContentScript({
  matches: ['*://*.google.com/*'],
  main() {
    // use allowed console methods (warn/error)
    console.warn('Hello content.')
  },
})

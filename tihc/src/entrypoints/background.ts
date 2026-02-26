import { ExtensionServiceWorkerMLCEngineHandler } from "@mlc-ai/web-llm";



export default defineBackground(() => {
  console.log('Hello background!', { id: browser.runtime.id });
  browser.action.onClicked.addListener(() => {
    browser.sidePanel.setPanelBehavior({ openPanelOnActionClick: true }).catch(error => console.error(error));
  });

  // Listen for connections from content scripts or other extension parts
  browser.runtime.onConnect.addListener((port) => {
    console.log('Setting up MLC Engine Handler in service worker');
    let handler = new ExtensionServiceWorkerMLCEngineHandler(port);
    port.onMessage.addListener(handler.onmessage.bind(handler));
  });
});

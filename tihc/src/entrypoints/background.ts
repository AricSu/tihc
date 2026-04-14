import { registerWebSearchBackgroundBridge } from "@/lib/websearch/background-bridge";

function enableSidePanelOnActionClick() {
  browser.action.onClicked.addListener(() => {
    browser.sidePanel
      .setPanelBehavior({ openPanelOnActionClick: true })
      .catch((error) => console.error(error));
  });
}

export default defineBackground(() => {
  registerWebSearchBackgroundBridge();
  enableSidePanelOnActionClick();
});

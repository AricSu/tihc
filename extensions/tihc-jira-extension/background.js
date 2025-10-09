chrome.action.onClicked.addListener((tab) => {
  chrome.scripting.executeScript({
    target: { tabId: tab.id },
    func: toggleSidebar
  });
});

function toggleSidebar() {
  const sidebar = document.getElementById("jira-copilot-sidebar");
  if (sidebar) {
    sidebar.classList.toggle("open");
  } else {
    // 如果还没注入 content.js，可以在这里直接创建
    console.warn("Sidebar not found. Content script may not be loaded.");
  }
}

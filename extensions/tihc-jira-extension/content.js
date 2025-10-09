(function () {
  if (document.getElementById("jira-copilot-sidebar")) return;

  const sidebar = document.createElement("div");
  sidebar.id = "jira-copilot-sidebar";
  sidebar.innerHTML = `
    <div class="header">
      Jira Copilot
      <button id="close-btn">×</button>
    </div>
    <div class="messages"></div>
    <textarea id="input" placeholder="Ask something..."></textarea>
    <button id="send-btn">Send</button>
  `;
  document.body.appendChild(sidebar);

  const messages = sidebar.querySelector(".messages");
  const input = sidebar.querySelector("#input");
  const sendBtn = sidebar.querySelector("#send-btn");
  const closeBtn = sidebar.querySelector("#close-btn");

  // 关闭按钮
  closeBtn.onclick = () => {
    sidebar.classList.remove("open");
  };

  // 发送消息
  sendBtn.onclick = async () => {
    const text = input.value.trim();
    if (!text) return;
    messages.innerHTML += `<div class="msg user">${text}</div>`;
    input.value = "";

    const res = await fetch("https://api.openai.com/v1/chat/completions", {
      method: "POST",
      headers: {
        "Authorization": "Bearer YOUR_API_KEY",
        "Content-Type": "application/json"
      },
      body: JSON.stringify({
        model: "gpt-4o-mini",
        messages: [{ role: "user", content: text }]
      })
    });
    const data = await res.json();
    const reply = data.choices?.[0]?.message?.content || "Error";
    messages.innerHTML += `<div class="msg bot">${reply}</div>`;
    messages.scrollTop = messages.scrollHeight;
  };
})();

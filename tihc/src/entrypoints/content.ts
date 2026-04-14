import { registerWebSearchContentBridge } from "@/lib/websearch/content-bridge";

export default defineContentScript({
  matches: ['http://*/*', 'https://*/*'],
  main() {
    registerWebSearchContentBridge();
  },
});

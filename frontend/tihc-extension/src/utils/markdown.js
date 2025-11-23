import MarkdownIt from 'markdown-it'

// 配置 markdown-it 实例
const md = new MarkdownIt({
  html: true, // 允许 HTML 标签
  linkify: true, // 自动转换 URL 为链接
  typographer: true, // 启用一些语言中性的替换 + 引号美化
  breaks: true, // 转换换行符为 <br>
})

// 自定义渲染规则以改善代码块样式
md.renderer.rules.code_block = (tokens, idx) => {
  const token = tokens[idx]
  return `<pre class="code-block"><code>${md.utils.escapeHtml(token.content)}</code></pre>`
}

md.renderer.rules.code_inline = (tokens, idx) => {
  const token = tokens[idx]
  return `<code class="inline-code">${md.utils.escapeHtml(token.content)}</code>`
}

// 自定义表格渲染
md.renderer.rules.table_open = () => '<div class="table-wrapper"><table class="markdown-table">'
md.renderer.rules.table_close = () => '</table></div>'

/**
 * 渲染 Markdown 文本为 HTML
 * @param {string} text - 要渲染的 Markdown 文本
 * @returns {string} 渲染后的 HTML
 */
export function renderMarkdown(text) {
  if (!text)
    return ''

  try {
    return md.render(text)
  }
  catch (error) {
    console.error('Markdown rendering error:', error)
    return md.utils.escapeHtml(text)
  }
}

/**
 * 渲染内联 Markdown（不包含块级元素）
 * @param {string} text - 要渲染的文本
 * @returns {string} 渲染后的 HTML
 */
export function renderInlineMarkdown(text) {
  if (!text)
    return ''
  
  try {
    return md.renderInline(text)
  }
  catch (error) {
    console.error('Inline markdown rendering error:', error)
    return md.utils.escapeHtml(text)
  }
}

/**
 * 流式渲染 Markdown 文本，适合实时更新的场景
 * @param {string} text - 可能不完整的 Markdown 文本
 * @returns {string} 渲染后的 HTML
 */
export function renderStreamingMarkdown(text) {
  if (!text)
    return ''

  try {
    // 处理不完整的代码块
    let processedText = text
    
    // 如果存在未闭合的代码块，临时闭合它以便渲染
    const codeBlockMatches = text.match(/```[\s\S]*$/g)
    if (codeBlockMatches && !text.endsWith('```')) {
      processedText = `${text}\n\`\`\``
    }
    
    // 处理不完整的行内代码
    const inlineCodeCount = (text.match(/`/g) || []).length
    if (inlineCodeCount % 2 === 1) {
      processedText = `${text}\``
    }
    
    // 处理不完整的粗体标记
    const boldMatches = text.match(/\*\*/g) || []
    if (boldMatches.length % 2 === 1) {
      processedText = `${text}**`
    }
    
    // 处理不完整的斜体标记
    const italicMatches = text.match(/(?<!\*)\*(?!\*)/g) || []
    if (italicMatches.length % 2 === 1) {
      processedText = `${text}*`
    }
    
    // 渲染处理后的文本
    const rendered = md.render(processedText)
    
    // 如果我们添加了临时的闭合标记，需要在渲染结果中标记为"正在输入"
    if (processedText !== text) {
      return rendered.replace(/<\/([^>]+)>$/, '</​$1><span class="typing-indicator">▌</span>')
    }
    
    return rendered
  }
  catch (error) {
    console.error('Streaming markdown rendering error:', error)
    // 回退到安全的HTML转义
    return `<p>${md.utils.escapeHtml(text)}<span class="typing-indicator">▌</span></p>`
  }
}

/**
 * 检查文本是否包含 Markdown 格式
 * @param {string} text - 要检查的文本
 * @returns {boolean} 是否包含 Markdown 格式
 */
export function hasMarkdownFormat(text) {
  if (!text)
    return false

  // 简单的 Markdown 格式检测
  const markdownPatterns = [
    /^#{1,6}\s/, // 标题
    /\*\*.*\*\*/, // 粗体
    /\*.*\*/, // 斜体
    /`.*`/, // 行内代码
    /```[\s\S]*```/, // 代码块
    /^\s*[-*+]\s/, // 无序列表
    /^\s*\d+\.\s/, // 有序列表
    /\[.*\]\(.*\)/, // 链接
    /^\s*\|.*\|/, // 表格
  ]

  return markdownPatterns.some(pattern => pattern.test(text))
}

/**
 * 智能检测是否应该使用流式 Markdown 渲染
 * @param {string} text - 要检查的文本
 * @returns {boolean} 是否应该使用流式渲染
 */
export function shouldUseStreamingRender(text) {
  if (!text)
    return false

  // 如果包含任何 Markdown 标记，就使用流式渲染
  return hasMarkdownFormat(text)
    || text.includes('#') // 可能是标题
    || text.includes('*') // 可能是粗体/斜体/列表
    || text.includes('`') // 可能是代码
    || text.includes('[') // 可能是链接
    || text.includes('|') // 可能是表格
}

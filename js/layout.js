// Window resize and layout handler / 窗口大小调整和布局处理器
// Positions canvases responsively for landscape and portrait / 响应式定位画布，适配横屏和竖屏

let resizeTimer = null;
const RESIZE_DEBOUNCE_MS = 100; // Debounce delay in ms / 防抖延迟（毫秒）

let resize = () => {
  if (resizeTimer) clearTimeout(resizeTimer);
  resizeTimer = setTimeout(performResize, RESIZE_DEBOUNCE_MS);
};

function performResize() {
  const canvas = document.getElementById("sand-canvas");
  const canvas2 = document.getElementById("fluid-canvas");
  const ui = document.getElementById("ui");

  if (!canvas || !canvas2 || !ui) return;

  let screen_width = window.innerWidth;
  let uiheight = 50;
  let screen_height = window.innerHeight - uiheight;

  let canvasStyle = "";
  let uiStyle = "";

  if (screen_width > screen_height) {
    // Landscape mode / 横屏模式
    if (screen_width - window.innerHeight < 400) {
      // Compressed landscape / 压缩横屏：画布占满高度，UI 占剩余宽度
      canvasStyle = `height: ${window.innerHeight}px; margin:3px`;
      uiStyle = `width: ${screen_width - window.innerHeight - 12}px; margin: 2px;`;
    } else {
      // Wide landscape / 宽横屏：方形画布居左，UI 固定 200px 宽
      canvasStyle = `
       height: ${window.innerHeight}px;
       width:${window.innerHeight}px;
       margin:0;
       left: auto;
       right: 206px`;
      uiStyle = `width: 200px; margin: 2px;`;
    }
  } else {
    // Portrait (mobile) / 竖屏（移动端）：画布宽度占满屏幕
    canvasStyle = `width: ${screen_width}px; bottom:3px;`;
    uiStyle = "";
  }

  ui.style = uiStyle;
  canvas.style = canvasStyle;
  canvas2.style = canvasStyle;

  // Update pull tab position / 更新下拉标签位置
  let btnHeight = ui.getBoundingClientRect().height;
  const pullTabContent = document.getElementById("PullTabContent");
  if (pullTabContent) pullTabContent.style.top = btnHeight + "px";
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', resize);
} else {
  resize();
}

window.addEventListener("deviceorientation", resize, true);
window.addEventListener("resize", resize);

// Observe UI element size changes (e.g., element bar expanding) / 监听UI元素大小变化（如元素栏展开）
if (window.ResizeObserver) {
  const ui_element = document.getElementById("ui");
  if (ui_element) {
    new ResizeObserver(resize).observe(ui_element);
  }
}

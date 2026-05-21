let resizeTimer = null;
const RESIZE_DEBOUNCE_MS = 100;

let resize = () => {
  if (resizeTimer) clearTimeout(resizeTimer);
  resizeTimer = setTimeout(performResize, RESIZE_DEBOUNCE_MS);
};

function performResize() {
  const canvas = document.getElementById("sand-canvas");
  const canvas2 = document.getElementById("fluid-canvas");
  const ui = document.getElementById("ui");

  if (!canvas || !canvas2 || !ui) return;

  const screen_width = window.innerWidth;
  const screen_height = window.innerHeight;
  const ui_rect = ui.getBoundingClientRect();

  let canvasStyle = "";
  let uiStyle = "";

  if (screen_width > screen_height) {
    // Landscape mode
    const ui_width = ui_rect.width || 0;
    const canvas_size = Math.min(screen_height - 4, screen_width - ui_width - 16);
    canvasStyle = `height: ${canvas_size}px; width: ${canvas_size}px; margin: 2px;`;
    if (ui_width > 200) {
      canvasStyle += ` left: auto; right: ${ui_width + 6}px;`;
    }
    uiStyle = "";
  } else {
    // Portrait (mobile)
    const max_size = Math.min(screen_width, screen_height - (ui_rect.height || 50));
    canvasStyle = `width: ${max_size}px; height: ${max_size}px; margin: auto; bottom: 0;`;
    uiStyle = "";
  }

  ui.style = uiStyle;
  canvas.style = canvasStyle;
  canvas2.style = canvasStyle;

  // Update pull tab position
  const pullTabContent = document.getElementById("PullTabContent");
  if (pullTabContent) {
    pullTabContent.style.top = Math.max(ui_rect.height || 10, 10) + "px";
  }
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', resize);
} else {
  resize();
}

window.addEventListener("resize", resize);
window.addEventListener("deviceorientation", resize, true);

// Observe UI element size changes (e.g., element bar expanding)
if (window.ResizeObserver) {
  const ui_element = document.getElementById("ui");
  if (ui_element) {
    new ResizeObserver(resize).observe(ui_element);
  }
}

// Mouse and touch paint handling / 鼠标和触控绘制处理
// Manages painting elements onto the canvas via mouse or touch input / 通过鼠标或触控输入在画布上绘制元素

import { height, universe, width } from "./index.js";
import { sizeMap } from "./components/ui";
const canvas = document.getElementById("sand-canvas");

// Euclidean distance between two mouse/touch events / 两个鼠标/触摸事件之间的欧几里得距离
const dist = (a, b) => {
  const dx = a.clientX - b.clientX, dy = a.clientY - b.clientY;
  return Math.sqrt(dx * dx + dy * dy);
};

let painting = false;
let lastPaint = null;
let repeat = null;

canvas.addEventListener("mousedown", (event) => {
  event.preventDefault();
  universe.push_undo();
  painting = true;
  clearInterval(repeat);
  repeat = window.setInterval(() => paint(event), 100);
  paint(event);
  lastPaint = event;
});

document.body.addEventListener("mouseup", (event) => {
  clearInterval(repeat);
  if (painting) {
    event.preventDefault();
    lastPaint = null;
    painting = false;
  }
});

canvas.addEventListener("mousemove", (event) => {
  clearInterval(repeat);
  smoothPaint(event);
});

canvas.addEventListener("mouseleave", (event) => {
  clearInterval(repeat);
  lastPaint = null;
});

canvas.addEventListener("touchstart", (event) => {
  universe.push_undo();
  if (event.cancelable) {
    event.preventDefault();
  }
  painting = true;
  lastPaint = event;
  handleTouches(event);
});

canvas.addEventListener("touchend", (event) => {
  if (event.cancelable) {
    event.preventDefault();
  }
  lastPaint = null;
  painting = false;
  clearInterval(repeat);
});

canvas.addEventListener("touchmove", (event) => {
  if (!window.paused) {
    if (event.cancelable) {
      event.preventDefault();
    }
  }
  clearInterval(repeat);
  handleTouches(event);
});

// Interpolate paint strokes between last and current mouse position / 在上一鼠标位置和当前鼠标位置之间插值绘制笔画
function smoothPaint(event) {
  clearInterval(repeat);
  repeat = window.setInterval(() => paint(event), 100);
  if (!painting) return;

  let size = sizeMap[window.UI.state.size];
  let step = Math.max(size / 5, 1); // Step size for interpolation / 插值步长
  if (lastPaint) {
    const sx = lastPaint.clientX, sy = lastPaint.clientY;
    const ex = event.clientX, ey = event.clientY;
    const totalDist = dist(lastPaint, event);
    let traveled = 0;
    let i = 0;
    while (traveled + step < totalDist) {
      traveled += step;
      const t = traveled / totalDist;
      paint({ clientX: sx + (ex - sx) * t, clientY: sy + (ey - sy) * t });
      i++;
      if (i > 1000) break; // Safety limit to prevent infinite loops / 安全限制，防止无限循环
    }
  }
  paint(event);
  lastPaint = event;
}

// Route single touch to smoothPaint, multi-touch to individual paints / 单指触控使用平滑绘制，多指触控各自单独绘制
const handleTouches = (event) => {
  let touches = Array.from(event.touches);
  if (touches.length == 1) {
    smoothPaint(touches[0]);
  } else {
    touches.forEach(paint);
  }
};

// Paint a single element stamp at the cursor position / 在光标位置绘制单个元素印记
const paint = (event) => {
  if (!painting) {
    return;
  }
  const boundingRect = canvas.getBoundingClientRect();

  // Scale from CSS pixels to canvas pixels accounting for devicePixelRatio / 从CSS像素缩放到画布像素，考虑设备像素比
  const scaleX =
    canvas.width / Math.ceil(window.devicePixelRatio) / boundingRect.width;
  const scaleY =
    canvas.height / Math.ceil(window.devicePixelRatio) / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const x = Math.min(Math.floor(canvasLeft), width - 1);
  const y = Math.min(Math.floor(canvasTop), height - 1);
  if (window.UI.state.selectedElement < 0) return;
  universe.paint(
    x,
    y,
    sizeMap[window.UI.state.size],
    window.UI.state.selectedElement
  );
};

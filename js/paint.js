import { height, universe, width } from "./index.js";
import { sizeMap } from "./components/ui";
const canvas = document.getElementById("sand-canvas");

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

function smoothPaint(event) {
  clearInterval(repeat);
  repeat = window.setInterval(() => paint(event), 100);
  if (!painting) return;

  let size = sizeMap[window.UI.state.size];
  let step = Math.max(size / 5, 1);
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
      if (i > 1000) break;
    }
  }
  paint(event);
  lastPaint = event;
}

const handleTouches = (event) => {
  let touches = Array.from(event.touches);
  if (touches.length == 1) {
    smoothPaint(touches[0]);
  } else {
    touches.forEach(paint);
  }
};

const paint = (event) => {
  if (!painting) {
    return;
  }
  const boundingRect = canvas.getBoundingClientRect();

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

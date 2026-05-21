// Startup animation / 启动动画
// Gradually paints sand terrain and seeds when the game first loads / 游戏首次加载时逐渐绘制沙地地形和种子

import { Species } from "../crate/pkg";

// Simple promise-based sleep / 简单的基于Promise的延迟
function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

// Animate sand dunes and seeds appearing on the canvas / 动画展示沙丘和种子出现在画布上
async function boot(width, height) {
  for (let x = 5; x <= width - 5; x += 10) {
    window.u.paint(
      x,
      Math.floor(height - 40 + 5 * Math.sin(x / 20)),
      Math.random() * 6 + 10,
      Species.Sand
    );
    if (window.stopboot) return;
    await sleep(16);
  }
  for (let x = 40; x <= width - 40; x += 50 + Math.random() * 10) {
    window.u.paint(
      x,
      Math.floor(height / 2 + 20 * Math.sin(x / 20)),
      6,
      Species.Seed
    );
    if (window.stopboot) return;
    await sleep(180);
  }
}
export { sleep, boot };

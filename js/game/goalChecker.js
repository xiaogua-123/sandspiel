// Goal checking logic for levels / 关卡目标检查逻辑
// Scans cell grid to verify CLEAR_ALL or CREATE goals / 扫描细胞网格以验证清除全部或创建目标

import * as wasm from "../../crate/pkg/sandtable_bg.wasm";
const memory = wasm.memory;

// Only check goals every N ticks to save performance / 每隔N帧检查目标以节省性能
const GOAL_CHECK_INTERVAL = 30;
let tickCounter = 0;
let goalAchieved = false;
let highestCount = 0;

// Check if level goal is met by counting target species cells / 通过计数目标元素细胞来检查关卡目标是否达成
function checkGoal(goal, universe, width, height) {
  if (!goal || goalAchieved) return goalAchieved;

  tickCounter++;
  if (tickCounter < GOAL_CHECK_INTERVAL) return false;
  tickCounter = 0;

  const cells = new Uint8Array(
    memory.buffer,
    universe.cells(),
    width * height * 4
  );

  const targetSpecies = goal.species;
  let count = 0;
  const totalCells = width * height;

  for (let i = 0; i < totalCells; i++) {
    if (cells[i * 4] === targetSpecies) {
      count++;
    }
  }

  switch (goal.type) {
    case "CLEAR_ALL":
      if (count === 0) {
        goalAchieved = true;
      }
      break;
    case "CREATE":
      highestCount = Math.max(highestCount, count);
      if (highestCount >= goal.target) {
        goalAchieved = true;
      }
      break;
  }

  return goalAchieved;
}

// Reset goal tracking state for a new level / 重置关卡目标追踪状态
function resetGoalState() {
  tickCounter = 0;
  goalAchieved = false;
  highestCount = 0;
}

function isGoalAchieved() {
  return goalAchieved;
}

export { checkGoal, resetGoalState, isGoalAchieved };

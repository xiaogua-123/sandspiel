import * as wasm from "../../crate/pkg/sandtable_bg.wasm";
const memory = wasm.memory;

const GOAL_CHECK_INTERVAL = 30;
let tickCounter = 0;
let goalAchieved = false;
let highestCount = 0;

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

function resetGoalState() {
  tickCounter = 0;
  goalAchieved = false;
  highestCount = 0;
}

function isGoalAchieved() {
  return goalAchieved;
}

export { checkGoal, resetGoalState, isGoalAchieved };

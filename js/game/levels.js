// Level definitions with goals and setup logic / 关卡定义，包含目标和初始化逻辑
// Each level specifies initial state via setup() and win condition via goal{} / 每个关卡通过setup()指定初始状态，通过goal{}指定胜利条件

import { Species } from "../../crate/pkg/sandtable";

const GoalType = {
  CLEAR_ALL: "CLEAR_ALL", // Eliminate all cells of target species
  CREATE: "CREATE",       // Reach at least N cells of target species
};

/**
 * Level definition format / 关卡定义格式:
 * {
 *   id: number,
 *   name: string,
 *   description: string,
 *   difficulty: 1-5,
 *   setup: (universe, width, height) => void,  // Paint initial state / 绘制初始状态
 *   goal?: { type, species, target?, label }    // Win condition / 胜利条件
 * }
 */

// Helper to paint a rectangular region with random fill / 用随机填充绘制矩形区域的辅助函数
function fillRect(u, x1, y1, x2, y2, species, size) {
  for (let x = x1; x <= x2; x++) {
    for (let y = y1; y <= y2; y++) {
      if (Math.random() > 0.4) {
        u.paint(x, y, size || 3, species);
      }
    }
  }
}

const levels = [
  {
    id: 0,
    name: "自由沙盒",
    description: "随意玩耍，释放你的创造力",
    difficulty: 1,
    setup: (u, w, h) => {
      // Just a clean sandbox - nothing to set up
    }
  },
  {
    id: 1,
    name: "灭火行动",
    description: "森林着火了！用水扑灭火焰",
    difficulty: 2,
    goal: { type: GoalType.CLEAR_ALL, species: Species.Fire, label: "扑灭所有火焰" },
    setup: (u, w, h) => {
      // Create a fire in the middle
      for (let x = Math.floor(w * 0.3); x <= Math.floor(w * 0.7); x++) {
        for (let y = Math.floor(h * 0.3); y <= Math.floor(h * 0.5); y++) {
          if (Math.random() > 0.6) {
            u.paint(x, y, 3, Species.Fire);
          }
        }
      }
      // Surround with wood
      for (let x = Math.floor(w * 0.25); x <= Math.floor(w * 0.75); x++) {
        u.paint(x, Math.floor(h * 0.25), 2, Species.Wood);
        u.paint(x, Math.floor(h * 0.55), 2, Species.Wood);
      }
      // Some water nearby
      for (let x = Math.floor(w * 0.6); x <= Math.floor(w * 0.8); x++) {
        u.paint(x, Math.floor(h * 0.7), 5, Species.Water);
      }
    }
  },
  {
    id: 2,
    name: "沙漠绿洲",
    description: "在沙漠中播撒种子，浇灌出一片绿洲",
    difficulty: 2,
    goal: { type: GoalType.CREATE, species: Species.Plant, target: 30, label: "种出30株植物" },
    setup: (u, w, h) => {
      // Sand ground
      for (let x = 0; x < w; x++) {
        u.paint(x, Math.floor(h * 0.7), 8, Species.Sand);
      }
      // Seeds
      u.paint(Math.floor(w * 0.3), Math.floor(h * 0.65), 2, Species.Seed);
      u.paint(Math.floor(w * 0.5), Math.floor(h * 0.63), 2, Species.Seed);
      u.paint(Math.floor(w * 0.7), Math.floor(h * 0.66), 2, Species.Seed);
      // Water source
      for (let x = Math.floor(w * 0.4); x <= Math.floor(w * 0.6); x++) {
        u.paint(x, Math.floor(h * 0.75), 4, Species.Water);
      }
    }
  },
  {
    id: 3,
    name: "冰与火之歌",
    description: "用岩浆攻破冰封堡垒",
    difficulty: 3,
    goal: { type: GoalType.CREATE, species: Species.Water, target: 200, label: "生成200格水" },
    setup: (u, w, h) => {
      // Ice fortress on the right
      for (let x = Math.floor(w * 0.55); x <= Math.floor(w * 0.8); x++) {
        for (let y = Math.floor(h * 0.2); y <= Math.floor(h * 0.6); y++) {
          if (x === Math.floor(w * 0.55) || x === Math.floor(w * 0.8)
            || y === Math.floor(h * 0.2) || y === Math.floor(h * 0.6)) {
            u.paint(x, y, 2, Species.Ice);
          }
        }
      }
      // Lava on the left
      for (let y = Math.floor(h * 0.3); y <= Math.floor(h * 0.5); y++) {
        u.paint(Math.floor(w * 0.2), y, 4, Species.Lava);
      }
    }
  },
  {
    id: 4,
    name: "石油泄漏",
    description: "控制石油泄漏，防止污染扩散",
    difficulty: 3,
    goal: { type: GoalType.CLEAR_ALL, species: Species.Oil, label: "清除所有石油" },
    setup: (u, w, h) => {
      // Water base
      for (let y = Math.floor(h * 0.5); y <= Math.floor(h * 0.8); y++) {
        for (let x = Math.floor(w * 0.1); x <= Math.floor(w * 0.6); x++) {
          if (Math.random() > 0.3) {
            u.paint(x, y, 3, Species.Water);
          }
        }
      }
      // Oil source at top
      for (let x = Math.floor(w * 0.3); x <= Math.floor(w * 0.45); x++) {
        u.paint(x, Math.floor(h * 0.2), 3, Species.Oil);
      }
    }
  },
  {
    id: 5,
    name: "真菌危机",
    description: "真菌正在吞噬一切！",
    difficulty: 3,
    goal: { type: GoalType.CLEAR_ALL, species: Species.Fungus, label: "清除所有真菌" },
    setup: (u, w, h) => {
      // Wood structure
      for (let x = Math.floor(w * 0.2); x <= Math.floor(w * 0.8); x++) {
        for (let y = Math.floor(h * 0.3); y <= Math.floor(h * 0.4); y++) {
          if (Math.random() > 0.3) {
            u.paint(x, y, 2, Species.Wood);
          }
        }
      }
      for (let x = Math.floor(w * 0.4); x <= Math.floor(w * 0.6); x++) {
        for (let y = Math.floor(h * 0.4); y <= Math.floor(h * 0.6); y++) {
          if (Math.random() > 0.5) {
            u.paint(x, y, 2, Species.Wood);
          }
        }
      }
      // Fungus starter
      u.paint(Math.floor(w * 0.5), Math.floor(h * 0.35), 5, Species.Fungus);
    }
  },
  {
    id: 6,
    name: "火箭试射",
    description: "点燃引信，发射火箭！",
    difficulty: 4,
    goal: { type: GoalType.CREATE, species: Species.Fire, target: 5, label: "点燃5个火焰点" },
    setup: (u, w, h) => {
      // Launch pad
      for (let x = Math.floor(w * 0.4); x <= Math.floor(w * 0.6); x++) {
        u.paint(x, Math.floor(h * 0.75), 5, Species.Stone);
      }
      // Rockets
      u.paint(Math.floor(w * 0.5), Math.floor(h * 0.65), 2, Species.Rocket);
      u.paint(Math.floor(w * 0.45), Math.floor(h * 0.65), 2, Species.Rocket);
      u.paint(Math.floor(w * 0.55), Math.floor(h * 0.65), 2, Species.Rocket);
      // Cloner
      u.paint(Math.floor(w * 0.5), Math.floor(h * 0.55), 2, Species.Cloner);
    }
  },
  {
    id: 7,
    name: "酸蚀试验",
    description: "酸能腐蚀大部分物质，但墙是坚不可摧的",
    difficulty: 4,
    goal: { type: GoalType.CLEAR_ALL, species: Species.Ice, label: "用酸融化所有冰" },
    setup: (u, w, h) => {
      // Walls forming chambers
      for (let y = Math.floor(h * 0.2); y <= Math.floor(h * 0.6); y++) {
        u.paint(Math.floor(w * 0.3), y, 2, Species.Wall);
        u.paint(Math.floor(w * 0.7), y, 2, Species.Wall);
      }
      // Different materials in chambers
      for (let x = Math.floor(w * 0.35); x <= Math.floor(w * 0.65); x++) {
        u.paint(x, Math.floor(h * 0.55), 4, Species.Wood);
        u.paint(x, Math.floor(h * 0.45), 4, Species.Stone);
        u.paint(x, Math.floor(h * 0.35), 4, Species.Ice);
      }
      // Acid
      for (let x = Math.floor(w * 0.1); x <= Math.floor(w * 0.25); x++) {
        u.paint(x, Math.floor(h * 0.3), 4, Species.Acid);
      }
    }
  },
  {
    id: 8,
    name: "冰雪世界",
    description: "大雪纷飞，用水和火创造奇妙的景观",
    difficulty: 2,
    goal: { type: GoalType.CREATE, species: Species.Plant, target: 10, label: "种出10株植物" },
    setup: (u, w, h) => {
      // Snow-covered ground
      fillRect(u, 0, Math.floor(h * 0.65), w, Math.floor(h * 0.7), Species.Snow, 5);
      // Some ice formations
      fillRect(u, Math.floor(w * 0.1), Math.floor(h * 0.55), Math.floor(w * 0.25), Math.floor(h * 0.6), Species.Ice, 3);
      fillRect(u, Math.floor(w * 0.7), Math.floor(h * 0.5), Math.floor(w * 0.85), Math.floor(h * 0.6), Species.Ice, 3);
      // Water pool
      fillRect(u, Math.floor(w * 0.35), Math.floor(h * 0.55), Math.floor(w * 0.55), Math.floor(h * 0.6), Species.Water, 4);
      // Snowflakes falling (just a few spots)
      for (let i = 0; i < 20; i++) {
        u.paint(
          Math.floor(Math.random() * w * 0.8) + Math.floor(w * 0.1),
          Math.floor(Math.random() * h * 0.4) + Math.floor(h * 0.05),
          3, Species.Snow
        );
      }
    }
  },
  {
    id: 9,
    name: "元素大乱斗",
    description: "所有元素混在一起，会怎样？",
    difficulty: 5,
    goal: { type: GoalType.CREATE, species: Species.Fire, target: 100, label: "引发100格火焰" },
    setup: (u, w, h) => {
      const species_list = [
        Species.Sand, Species.Water, Species.Fire, Species.Lava,
        Species.Gas, Species.Oil, Species.Dust, Species.Acid,
        Species.Seed, Species.Mite, Species.Ice, Species.Snow,
        Species.Slime, Species.Glass, Species.Coral
      ];
      for (let i = 0; i < 40; i++) {
        const x = Math.floor(Math.random() * w * 0.8) + Math.floor(w * 0.1);
        const y = Math.floor(Math.random() * h * 0.6) + Math.floor(h * 0.1);
        const s = species_list[Math.floor(Math.random() * species_list.length)];
        u.paint(x, y, 8 + Math.floor(Math.random() * 10), s);
      }
    }
  }
];

export { GoalType };
export default levels;

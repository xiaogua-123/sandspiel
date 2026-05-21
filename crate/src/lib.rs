//! Core Universe module for the falling-sand simulation game.
//! / 沙盘模拟游戏核心 Universe 模块。
//!
//! Manages the cell grid, physics ticks, wind simulation, undo history, and the paint API
//! exposed to JavaScript via wasm-bindgen.
//! / 管理单元格网格、物理更新、风力模拟、撤销历史以及通过 wasm-bindgen 暴露给 JavaScript 的绘制 API。

extern crate cfg_if;
extern crate js_sys;
extern crate rand;
extern crate rand_xoshiro;
extern crate wasm_bindgen;
extern crate web_sys;

mod species;
mod utils;

use rand::{Rng, SeedableRng};
use rand_xoshiro::SplitMix64;
use species::Species;
use std::collections::VecDeque;
use wasm_bindgen::prelude::*;

/// Wind / fluid dynamics data stored per cell.
/// / 每个单元格存储的风力/流体动力学数据。
#[wasm_bindgen]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Wind {
    dx: u8,       // wind direction X / 风向 X
    dy: u8,       // wind direction Y / 风向 Y
    pressure: u8, // air pressure / 气压
    density: u8,  // fluid density / 流体密度
}
    dx: u8,
    dy: u8,
    pressure: u8,
    density: u8,
}

/// A single cell in the simulation grid.
/// / 模拟网格中的单个单元格。
///
/// `ra` and `rb` are generic registers used differently by each species
/// (e.g., ra = health/fuel/moisture, rb = burn timer/direction/state).
/// / `ra` 和 `rb` 是通用寄存器，供每个物种以不同方式使用
/// / （例如 ra = 健康值/燃料/湿度, rb = 燃烧计时器/方向/状态）。
#[wasm_bindgen]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    species: Species, // element type / 元素类型
    ra: u8,           // generic register A / 通用寄存器 A
    rb: u8,           // generic register B / 通用寄存器 B
    clock: u8,        // last-update generation / 上次更新的代数
}

impl Cell {
    /// Create a new cell with random initial ra value.
    /// / 创建一个带有随机初始 ra 值的新单元格。
    pub fn new(species: Species) -> Cell {
        Cell {
            species,
            ra: 100 + (js_sys::Math::random() * 50.) as u8,
            rb: 0,
            clock: 0,
        }
    }
    /// Dispatch to the species-specific update function.
    /// / 分派到物种特定的更新函数。
    pub fn update(&self, api: SandApi) {
        self.species.update(*self, api);
    }
}

/// Singleton empty cell used for clearing grid positions.
/// / 用于清除网格位置的单例空单元格。
pub static EMPTY_CELL: Cell = Cell {
    species: Species::Empty,
    ra: 0,
    rb: 0,
    clock: 0,
};

/// Wind resistance threshold per species (index by Species as u8).
/// / 每种物种的风阻阈值（按 Species 作为 u8 索引）。
/// Higher values = harder to blow / 数值越高 = 越难被吹动
const WIND_THRESHOLD: [u8; 136] = [
    200, // Empty = 0  (never moved by wind)
    200, // Wall = 1
    30,  // Sand = 2
    40,  // Water = 3
    5,   // Gas = 4
    200, // Cloner = 5
    5,   // Fire = 6
    70,  // Wood = 7
    60,  // Lava = 8
    60,  // Ice = 9
    15,  // Snow = 10 (light, easily blown)
    60,  // Plant = 11
    40,  // Acid = 12
    70,  // Stone = 13
    10,  // Dust = 14
    30,  // Mite = 15
    50,  // Oil = 16
    30,  // Rocket = 17
    54,  // Fungus = 18
    35,  // Seed = 19
    200, // Sponge = 20 (stationary)
    60,  // Slime = 21
    70,  // Glass = 22 (heavy)
    200, // Coral = 23 (stationary)
    // Metals (24-33) - heavy, hard to blow
    80,  // Iron = 24
    75,  // Copper = 25
    90,  // Gold = 26
    85,  // Silver = 27
    50,  // Aluminum = 28
    90,  // Lead = 29
    70,  // Zinc = 30
    65,  // Tin = 31
    85,  // Bronze = 32
    95,  // Steel = 33
    // Crystals (34-41) - hard, mostly stationary
    80,  // Diamond = 34
    80,  // Ruby = 35
    80,  // Sapphire = 36
    75,  // Emerald = 37
    70,  // Amethyst = 38
    70,  // Quartz = 39
    65,  // Crystal = 40
    85,  // Obsidian = 41
    // Powders (42-49) - very light
    8,   // Gunpowder = 42
    10,  // Flour = 43
    12,  // Sugar = 44
    15,  // Salt = 45
    10,  // Pepper = 46
    5,   // Ash = 47
    5,   // Soot = 48
    12,  // Charcoal = 49
    // Liquids (50-57)
    45,  // Mud = 50
    40,  // Blood = 51
    55,  // Honey = 52
    45,  // Milk = 53
    40,  // Poison = 54
    70,  // Mercury = 55
    35,  // Alcohol = 56
    55,  // Syrup = 57
    // Gases (58-65) - very light
    3,   // Steam = 58
    4,   // Smoke = 59
    2,   // Helium = 60
    5,   // Chlorine = 61
    5,   // Oxygen = 62
    3,   // Hydrogen = 63
    3,   // PlasmaGas = 64
    4,   // Methane = 65
    // Organics (66-75)
    15,  // Leaf = 66
    20,  // Flower = 67
    25,  // Grass = 68
    40,  // Vine = 69
    200, // Moss = 70 (stationary)
    60,  // Mushroom = 71
    70,  // Bark = 72
    70,  // Root = 73
    50,  // Fruit = 74
    60,  // Thorn = 75
    // Creatures (76-83)
    20,  // Ant = 76
    25,  // Spider = 77
    15,  // Bee = 78
    10,  // Butterfly = 79
    30,  // Fish = 80
    10,  // Bird = 81
    35,  // Snake = 82
    50,  // Worm = 83
    // Explosives (84-91)
    60,  // TNT = 84
    55,  // Bomb = 85
    40,  // Nitro = 86
    90,  // Plutonium = 87
    90,  // Uranium = 88
    65,  // C4 = 89
    70,  // Thermite = 90
    55,  // Napalm = 91
    // Construction (92-99)
    80,  // Brick = 92
    90,  // Concrete = 93
    60,  // Cement = 94
    75,  // Tile = 95
    70,  // Plaster = 96
    85,  // Marble = 97
    90,  // Granite = 98
    95,  // Basalt = 99
    // Magical (100-109)
    200, // Portal = 100
    200, // Teleporter = 101
    10,  // Antigravity = 102
    200, // Magnet = 103
    5,   // Lightning = 104
    200, // Void = 105
    5,   // Chaos = 106
    3,   // Energy = 107
    200, // Shield = 108
    200, // Mirror = 109
    // Food (110-115)
    50,  // Bread = 110
    60,  // Cheese = 111
    55,  // Meat = 112
    40,  // Egg = 113
    10,  // Rice = 114
    25,  // Wheat = 115
    // Nature (116-123)
    55,  // Clay = 116
    35,  // Soil = 117
    40,  // Peat = 118
    75,  // Limestone = 119
    40,  // Chalk = 120
    70,  // Shale = 121
    75,  // Slate = 122
    65,  // Sandstone = 123
    // Tech (124-129)
    200, // Wire = 124
    200, // Circuit = 125
    200, // Battery = 126
    200, // SolarCell = 127
    200, // Laser = 128
    200, // LED = 129
    // Misc (130-135)
    5,   // Bubble = 130
    8,   // Balloon = 131
    3,   // Confetti = 132
    5,   // Glitter = 133
    200, // Spring = 134
    70,  // Domino = 135
];

/// The main simulation universe containing all cells, winds, and state.
/// / 主模拟宇宙，包含所有单元格、风力和状态。
#[wasm_bindgen]
pub struct Universe {
    width: i32,        // grid width / 网格宽度
    height: i32,       // grid height / 网格高度
    cells: Vec<Cell>,  // flat cell array / 扁平单元格数组
    undo_stack: VecDeque<Vec<Cell>>, // undo history (max 50) / 撤销历史（最多 50 个）
    winds: Vec<Wind>,  // wind data per cell / 每个单元格的风力数据
    burns: Vec<Wind>,  // burn/fire data per cell / 每个单元格的燃烧/火焰数据
    generation: u8,    // current tick generation / 当前更新代数
    rng: SplitMix64,   // deterministic random number generator / 确定性随机数生成器
}

/// Mutable reference to a specific cell within the Universe,
/// with convenience methods for neighbor lookups.
/// / Universe 中特定单元格的可变引用，
/// / 带有方便的邻居查找方法。
pub struct SandApi<'a> {
    x: i32,
    y: i32,
    universe: &'a mut Universe,
}

impl<'a> SandApi<'a> {
    /// Get the cell at relative offset (dx, dy) from current position.
    /// / 获取距离当前位置相对偏移 (dx, dy) 处的单元格。
    /// Out-of-bounds returns a virtual Wall cell.
    /// / 越界返回一个虚拟的 Wall 单元格。
    pub fn get(&mut self, dx: i32, dy: i32) -> Cell {
        if dx > 2 || dx < -2 || dy > 2 || dy < -2 {
            panic!("oob get");
        }
        let nx = self.x + dx;
        let ny = self.y + dy;
        // Treat edges as walls / 将边界视为墙壁
        if nx < 0 || nx > self.universe.width - 1 || ny < 0 || ny > self.universe.height - 1 {
            return Cell {
                species: Species::Wall,
                ra: 0,
                rb: 0,
                clock: self.universe.generation,
            };
        }
        self.universe.get_cell(nx, ny)
    }
    /// Set a cell at relative offset (dx, dy) from current position.
    /// / 设置距离当前位置相对偏移 (dx, dy) 处的单元格。
    /// Out-of-bounds writes are silently ignored.
    /// / 越界写入将被静默忽略。
    pub fn set(&mut self, dx: i32, dy: i32, v: Cell) {
        if dx > 2 || dx < -2 || dy > 2 || dy < -2 {
            panic!("oob set");
        }
        let nx = self.x + dx;
        let ny = self.y + dy;
        if nx < 0 || nx > self.universe.width - 1 || ny < 0 || ny > self.universe.height - 1 {
            return;
        }
        let i = self.universe.get_index(nx, ny);
        self.universe.cells[i] = v;
        // Mark this cell as updated this tick / 将此单元格标记为本帧已更新
        self.universe.cells[i].clock = self.universe.generation.wrapping_add(1);
    }
    /// Get the wind data at the current position.
    /// / 获取当前位置的风力数据。
    pub fn get_fluid(&mut self) -> Wind {
        let idx = self.universe.get_index(self.x, self.y);
        self.universe.winds[idx]
    }
    /// Set the burn/fire wind data at the current position.
    /// / 设置当前位置的燃烧/火焰风力数据。
    pub fn set_fluid(&mut self, v: Wind) {
        let idx = self.universe.get_index(self.x, self.y);
        self.universe.burns[idx] = v;
    }

    /// Generate a random integer in [0, n).
    /// / 在 [0, n) 范围内生成随机整数。
    pub fn rand_int(&mut self, n: i32) -> i32 {
        self.universe.rng.gen_range(0..n)
    }
    /// True with probability 1/n (roughly once every n calls).
    /// / 以 1/n 的概率为真（大约每 n 次调用一次）。
    pub fn once_in(&mut self, n: i32) -> bool {
        self.rand_int(n) == 0
    }
    /// Random direction: -1, 0, or 1.
    /// / 随机方向：-1、0 或 1。
    pub fn rand_dir(&mut self) -> i32 {
        (self.rand_int(1000) % 3) - 1
    }
    /// Random lateral direction: -1 or 1.
    /// / 随机横向方向：-1 或 1。
    pub fn rand_dir_2(&mut self) -> i32 {
        if (self.rand_int(1000) % 2) == 0 { -1 } else { 1 }
    }

    /// Random 8-direction vector including (0,0).
    /// / 包含 (0,0) 的随机八方向向量。
    pub fn rand_vec(&mut self) -> (i32, i32) {
        match self.rand_int(2000) % 9 {
            0 => (1, 1),
            1 => (1, 0),
            2 => (1, -1),
            3 => (0, -1),
            4 => (-1, -1),
            5 => (-1, 0),
            6 => (-1, 1),
            7 => (0, 1),
            _ => (0, 0),
        }
    }

    /// Random 8-direction vector excluding (0,0).
    /// / 不包含 (0,0) 的随机八方向向量。
    pub fn rand_vec_8(&mut self) -> (i32, i32) {
        match self.rand_int(8) {
            0 => (1, 1),
            1 => (1, 0),
            2 => (1, -1),
            3 => (0, -1),
            4 => (-1, -1),
            5 => (-1, 0),
            6 => (-1, 1),
            _ => (0, 1),
        }
    }
}

// Public API exposed to JavaScript via wasm-bindgen.
// / 通过 wasm-bindgen 暴露给 JavaScript 的公共 API。
#[wasm_bindgen]
impl Universe {
    /// Clear all cells to empty.
    /// / 将所有单元格清为空。
    pub fn reset(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let idx = self.get_index(x, y);
                self.cells[idx] = EMPTY_CELL;
            }
        }
    }

    /// Advance the simulation by one frame (two passes: wind then physics).
    /// / 将模拟推进一帧（两个阶段：风力 + 物理）。
    pub fn tick(&mut self) {
        self.generation = self.generation.wrapping_add(1);

        // Wind pass: move cells based on fluid dynamics
        // / 风力阶段：根据流体动力学移动单元格
        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.get_cell(x, y);
                let wind = self.get_wind(x, y);
                Self::blow_wind(cell, wind, SandApi { universe: self, x, y });
            }
        }

        // Physics update pass: each cell runs its species-specific update
        // / 物理更新阶段：每个单元格运行其物种特定的更新
        self.generation = self.generation.wrapping_add(1);
        let scan_reverse = self.generation % 2 == 0; // alternate scan direction / 交替扫描方向
        for x in 0..self.width {
            let scanx = if scan_reverse {
                self.width - (1 + x) // right-to-left scan / 从右向左扫描
            } else {
                x // left-to-right scan / 从左向右扫描
            };
            for y in 0..self.height {
                let idx = self.get_index(scanx, y);
                let cell = self.get_cell(scanx, y);
                // Reset burn data for this cell / 重置此单元格的燃烧数据
                self.burns[idx] = Wind { dx: 0, dy: 0, pressure: 0, density: 0 };
                Self::update_cell(cell, SandApi { universe: self, x: scanx, y });
            }
        }
    }

    pub fn width(&self) -> i32 { self.width }
    pub fn height(&self) -> i32 { self.height }
    pub fn cells(&self) -> *const Cell { self.cells.as_ptr() }
    pub fn winds(&self) -> *const Wind { self.winds.as_ptr() }
    pub fn burns(&self) -> *const Wind { self.burns.as_ptr() }

    /// Paint a circular brush of the given species at (x, y).
    /// / 在 (x, y) 处用给定物种绘制圆形笔刷。
    /// Only overwrites empty cells (unless erasing with Species::Empty).
    /// / 仅覆盖空单元格（除非用 Species::Empty 擦除）。
    pub fn paint(&mut self, x: i32, y: i32, size: i32, species: Species) {
        let radius = (size as f64) / 2.0;
        let floor = (radius + 1.0) as i32;
        let ciel = (radius + 1.5) as i32;

        for dx in -floor..ciel {
            for dy in -floor..ciel {
                // Check if within circle / 检查是否在圆内
                if ((dx * dx + dy * dy) as f64) > (radius * radius) {
                    continue;
                }
                let px = x + dx;
                let py = y + dy;
                if px < 0 || px > self.width - 1 || py < 0 || py > self.height - 1 {
                    continue;
                }
                let i = self.get_index(px, py);
                if self.get_cell(px, py).species == Species::Empty || species == Species::Empty {
                    self.cells[i] = Cell {
                        species,
                        ra: 60 + (size as u8) + (self.rng.gen::<f32>() * 30.) as u8
                            + ((self.generation % 127) as i8 - 60).abs() as u8,
                        rb: 0,
                        clock: self.generation,
                    }
                }
            }
        }
    }

    /// Save current state for undo.
    /// / 保存当前状态以供撤销。
    pub fn push_undo(&mut self) {
        self.undo_stack.push_front(self.cells.clone());
        self.undo_stack.truncate(50); // keep last 50 states / 保留最近 50 个状态
    }

    /// Restore the most recently saved state.
    /// / 恢复最近保存的状态。
    pub fn pop_undo(&mut self) {
        if let Some(state) = self.undo_stack.pop_front() {
            self.cells = state;
        }
    }

    /// Clear all undo history.
    /// / 清除所有撤销历史。
    pub fn flush_undos(&mut self) {
        self.undo_stack.clear();
    }

    /// Create a new simulation universe.
    /// / 创建一个新的模拟宇宙。
    pub fn new(width: i32, height: i32) -> Universe {
        let cells = (0..width * height).map(|_| EMPTY_CELL).collect();
        let zero_wind = Wind { dx: 0, dy: 0, pressure: 0, density: 0 };
        let winds = vec![zero_wind; (width * height) as usize];
        let burns = vec![zero_wind; (width * height) as usize];
        let rng = SeedableRng::seed_from_u64(0x734f6b89de5f83cc); // fixed seed for determinism / 固定种子保证确定性
        Universe {
            width,
            height,
            cells,
            undo_stack: VecDeque::with_capacity(50),
            burns,
            winds,
            generation: 0,
            rng,
        }
    }
}

// Private helper methods for internal Universe operations.
// / Universe 内部操作的私有辅助方法。
impl Universe {
    /// Convert 2D coordinates to flat array index (column-major: x * height + y).
    /// / 将二维坐标转换为扁平数组索引（列优先：x * height + y）。
    fn get_index(&self, x: i32, y: i32) -> usize {
        (x * self.height + y) as usize
    }

    fn get_cell(&self, x: i32, y: i32) -> Cell {
        self.cells[self.get_index(x, y)]
    }

    fn get_wind(&self, x: i32, y: i32) -> Wind {
        self.winds[self.get_index(x, y)]
    }

    /// Apply wind force to a cell, moving it if wind exceeds its resistance threshold.
    /// / 对单元格施加风力，若风力超过其阻力阈值则移动该单元格。
    fn blow_wind(cell: Cell, wind: Wind, mut api: SandApi) {
        // Skip cells already updated this tick / 跳过本帧已更新的单元格
        if cell.clock.wrapping_sub(api.universe.generation) == 1 {
            return;
        }
        if cell.species == Species::Empty {
            return;
        }

        let threshold = WIND_THRESHOLD[cell.species as usize] as i32;
        // Wind data is stored as 0-255 centered around 126 / 风力数据以 0-255 存储，以 126 为中心
        let wx = (wind.dy as i32) - 126;
        let wy = (wind.dx as i32) - 126;

        let dx = if wx > threshold { 1 } else if wx < -threshold { -1 } else { 0 };
        let dy = if wy > threshold { 1 } else if wy < -threshold { -1 } else { 0 };

        if (dx != 0 || dy != 0) && api.get(dx, dy).species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            // Allow certain species to move 2 cells when blown upward / 某些物种向上吹时允许移动 2 格
            let final_dy = if dy == -1
                && api.get(dx, -2).species == Species::Empty
                && matches!(cell.species,
                    Species::Sand | Species::Water | Species::Lava
                    | Species::Acid | Species::Mite | Species::Dust
                    | Species::Oil | Species::Rocket)
            {
                -2
            } else {
                dy
            };
            api.set(dx, final_dy, cell);
        }
    }

    /// Update a single cell (skip if already updated this tick via swap).
    /// / 更新单个单元格（如果已在本帧通过交换更新则跳过）。
    fn update_cell(cell: Cell, api: SandApi) {
        // Skip cells already processed this tick (moved into from elsewhere)
        // / 跳过本帧已处理的单元格（从别处移入的）
        if cell.clock.wrapping_sub(api.universe.generation) == 1 {
            return;
        }
        cell.update(api);
    }
}

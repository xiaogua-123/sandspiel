//! Liquids simulation: water, oil, lava, and acid.
//! / 液体模拟：水、油、岩浆和酸。

use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

/// Water: flows and spreads with realistic dispersion.
/// / 水：流动并扩散，具有真实的弥散效果。
/// Uses ra % 2 to alternate flow direction per cell (prevents bias).
/// / 使用 ra % 2 为每个单元格交替流动方向（防止偏差）。
pub fn update_water(cell: Cell, mut api: SandApi) {
    let mut dx = api.rand_dir();
    let below = api.get(0, 1);
    let dx1 = api.get(dx, 1);

    // Fall down or sink through oil (water is denser than oil)
    // / 向下掉落或沉入油中（水比油密度大）
    if below.species == Species::Empty || below.species == Species::Oil {
        api.set(0, 0, below);
        let mut ra = cell.ra;
        if api.once_in(20) { ra = 100 + api.rand_int(50) as u8; } // randomize flow pattern / 随机化流动模式
        api.set(0, 1, Cell { ra, ..cell });
        return;
    } else if dx1.species == Species::Empty || dx1.species == Species::Oil {
        api.set(0, 0, dx1);
        api.set(dx, 1, cell);
        return;
    } else if api.get(-dx, 1).species == Species::Empty {
        // Try opposite diagonal / 尝试相反对角线
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 1, cell);
        return;
    }

    // Horizontal spreading: water flows sideways when it can't fall
    // / 水平扩散：当水无法下落时向侧面流动
    let left = cell.ra % 2 == 0; // ra parity determines preferred direction / ra 奇偶性决定优先方向
    dx = if left { 1 } else { -1 };
    let dx0 = api.get(dx, 0);
    let dxd = api.get(dx * 2, 0);

    if dx0.species == Species::Empty && dxd.species == Species::Empty {
        // Skip one cell for faster spread / 跳过一个单元格快速扩散
        api.set(0, 0, dxd);
        api.set(2 * dx, 0, Cell { rb: 6, ..cell });
        let (rdx, rdy) = api.rand_vec_8();
        let nbr = api.get(rdx, rdy);
        if nbr.species == Species::Water {
            if nbr.ra % 2 != cell.ra % 2 {
                api.set(rdx, rdy, Cell { ra: cell.ra, ..cell }) // equalize ra parity / 平衡 ra 奇偶性
            }
        }
    } else if dx0.species == Species::Empty || dx0.species == Species::Oil {
        api.set(0, 0, dx0);
        api.set(dx, 0, Cell { rb: 3, ..cell });
        let (rdx, rdy) = api.rand_vec_8();
        let nbr = api.get(rdx, rdy);
        if nbr.species == Species::Water {
            if nbr.ra % 2 != cell.ra % 2 {
                api.set(rdx, rdy, Cell { ra: cell.ra, ..cell })
            }
        }
    } else if cell.rb == 0 {
        if api.get(-dx, 0).species == Species::Empty {
            api.set(0, 0, Cell { ra: ((cell.ra as i32) + dx) as u8, ..cell }); // drift / 漂移
        }
    } else {
        api.set(0, 0, Cell { rb: cell.rb - 1, ..cell }); // dispersal cooldown / 扩散冷却
    }
}

/// Oil: flammable liquid, floats on water, spreads fire.
/// / 油：可燃液体，浮在水上，传播火焰。
/// rb = burn timer: 0 = unignited, 1 = consumed, 2-50 = burning.
/// / rb = 燃烧计时器：0 = 未点燃, 1 = 已消耗, 2-50 = 燃烧中。
pub fn update_oil(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;
    let (dx, dy) = api.rand_vec();

    let mut new_cell = cell;
    let nbr = api.get(dx, dy);
    // Ignite when near fire, lava, or burning oil / 靠近火源、岩浆或燃烧的油时点燃
    if rb == 0 && nbr.species == Species::Fire
        || nbr.species == Species::Lava
        || (nbr.species == Species::Oil && nbr.rb > 1 && nbr.rb < 20)
    {
        new_cell = Cell {
            species: Species::Oil,
            ra: cell.ra,
            rb: 50,
            clock: 0,
        };
    }

    if rb > 1 {
        new_cell = Cell {
            species: Species::Oil,
            ra: cell.ra,
            rb: rb - 1,
            clock: 0,
        };
        // Burning oil creates dense heat / 燃烧的油产生浓密热量
        api.set_fluid(Wind {
            dx: 0,
            dy: 10,
            pressure: 10,
            density: 180,
        });
        // Spawn flame particles periodically / 周期性生成火焰粒子
        if rb % 4 != 0 && nbr.species == Species::Empty && nbr.species != Species::Water {
            let ra = 20 + api.rand_int(30) as u8;
            api.set(dx, dy, Cell { species: Species::Fire, ra, rb: 0, clock: 0 });
        }
        if nbr.species == Species::Water {
            new_cell = Cell {
                species: Species::Oil,
                ra: 50,
                rb: 0,
                clock: 0,
            };
        }
    } else if rb == 1 {
        // Oil consumed by fire / 油被火烧尽
        api.set(0, 0, Cell { species: Species::Empty, ra: cell.ra, rb: 90, clock: 0 });
        return;
    }

    // Liquid movement: fall, diagonal, sideways / 液体移动：下落、对角线、侧向
    if api.get(0, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, new_cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, new_cell);
    } else if api.get(-dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 1, new_cell);
    } else if api.get(dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, new_cell);
    } else if api.get(-dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 0, new_cell);
    } else {
        api.set(0, 0, new_cell);
    }
}

/// Lava: hot molten rock, turns water to stone, ignites flammables.
/// / 岩浆：炽热熔岩，将水变成石头，点燃可燃物。
pub fn update_lava(cell: Cell, mut api: SandApi) {
    // Lava radiates heat / 岩浆辐射热量
    api.set_fluid(Wind {
        dx: 0,
        dy: 10,
        pressure: 0,
        density: 60,
    });
    let (dx, dy) = api.rand_vec();

    // Ignite gas/dust on contact / 接触时点燃气体/灰尘
    if api.get(dx, dy).species == Species::Gas || api.get(dx, dy).species == Species::Dust {
        api.set(dx, dy, Cell {
            species: Species::Fire,
            ra: (150 + (dx + dy) * 10) as u8,
            rb: 0,
            clock: 0,
        });
    }
    let sample = api.get(dx, dy);
    if sample.species == Species::Water {
        // Lava + water = stone (obsidian-like cooling) / 岩浆 + 水 = 石头（像黑曜石的冷却）
        api.set(0, 0, Cell { species: Species::Stone, ra: (150 + (dx + dy) * 10) as u8, rb: 0, clock: 0 });
        api.set(dx, dy, EMPTY_CELL);
    } else if api.get(0, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if api.get(dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else {
        api.set(0, 0, cell);
    }
}

/// Acid: corrosive liquid that dissolves most materials on contact.
/// / 酸：腐蚀性液体，接触时溶解大多数材料。
/// ra = acidity strength; depletes as it corrodes, dissipates when weak.
/// / ra = 酸度强度；腐蚀时消耗，变弱时消散。
pub fn update_acid(cell: Cell, mut api: SandApi) {
    let dx = api.rand_dir();
    let ra = cell.ra;
    let mut degraded = cell.clone();
    degraded.ra = ra.saturating_sub(60); // lose strength per corrosion attempt / 每次腐蚀尝试损失强度
    if degraded.ra < 80 {
        degraded = EMPTY_CELL; // too weak, dissipate / 太弱，消散
    }
    // Move like a liquid, but when blocked, corrode the blocker
    // / 像液体一样移动，但受阻时会腐蚀阻挡物
    if api.get(0, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else if api.get(-dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 0, cell);
    } else {
        // Corrode whatever is in the way / 腐蚀挡路的东西
        if api.get(0, 1).species != Species::Wall && api.get(0, 1).species != Species::Acid {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, 1, degraded);
        } else if api.get(dx, 0).species != Species::Wall && api.get(dx, 0).species != Species::Acid {
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, 0, degraded);
        } else if api.get(-dx, 0).species != Species::Wall && api.get(-dx, 0).species != Species::Acid {
            api.set(0, 0, EMPTY_CELL);
            api.set(-dx, 0, degraded);
        } else if api.get(0, -1).species != Species::Wall
            && api.get(0, -1).species != Species::Acid
            && api.get(0, -1).species != Species::Empty
        {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, -1, degraded);
        } else {
            api.set(0, 0, cell);
        }
    }
}

//! Gases simulation: generic gas and fire.
//! / 气体模拟：通用气体和火焰。

use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

/// Generic gas: rises, spreads, and merges with nearby gas cells.
/// / 通用气体：上升、扩散，并与附近气体单元格合并。
/// Uses rb as a density counter to make gas rise and spread realistically.
/// / 使用 rb 作为密度计数器，使气体真实地上升和扩散。
pub fn update_gas(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);

    // Initialize gas density / 初始化气体密度
    if cell.rb == 0 {
        api.set(0, 0, Cell { rb: 5, ..cell });
    }

    if nbr.species == Species::Empty {
        if cell.rb < 3 {
            // Low density: move freely / 低密度：自由移动
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, dy, cell);
        } else {
            // High density: split into two lower-density cells / 高密度：分裂成两个低密度单元格
            api.set(0, 0, Cell { rb: 1, ..cell });
            api.set(dx, dy, Cell { rb: cell.rb - 1, ..cell });
        }
    } else if (dx != 0 || dy != 0) && nbr.species == Species::Gas && nbr.rb < 4 {
        // Merge with adjacent gas cell / 与相邻气体单元格合并
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, dy, Cell { rb: nbr.rb + cell.rb, ..cell });
    }
}

/// Fire: spreading flame that heats surroundings and dies in water.
/// / 火焰：扩散的火焰，加热周围环境并在水中熄灭。
/// ra = fuel/heat remaining; creates heat via fluid (density field).
/// / ra = 剩余燃料/热量；通过流体（密度场）产生热量。
pub fn update_fire(cell: Cell, mut api: SandApi) {
    let ra = cell.ra;
    let mut degraded = cell.clone();
    degraded.ra = ra.saturating_sub((2 + api.rand_dir()) as u8); // burn fuel / 消耗燃料

    let (dx, dy) = api.rand_vec();

    // Fire creates upward heat / 火焰产生向上的热量
    api.set_fluid(Wind {
        dx: 0,
        dy: 150,
        pressure: 1,
        density: 120,
    });
    // Ignite nearby gas or dust / 点燃附近的气体或灰尘
    if api.get(dx, dy).species == Species::Gas || api.get(dx, dy).species == Species::Dust {
        api.set(dx, dy, Cell { species: Species::Fire, ra: (150 + (dx + dy) * 10) as u8, rb: 0, clock: 0 });
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 80, density: 40 });
    }
    if ra < 5 || api.get(dx, dy).species == Species::Water {
        // Extinguish: fire dies in water or when fuel runs out / 熄灭：火焰在水中或燃料耗尽时死亡
        api.set(0, 0, EMPTY_CELL);
    } else if api.get(dx, dy).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, dy, degraded);
    } else {
        api.set(0, 0, degraded);
    }
}

//! Falling solids simulation: sand, dust, and stone.
//! / 下落固体模拟：沙子、灰尘和石头。

use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

/// Sand: granular solid that falls and piles up.
/// / 沙子：粒状固体，会下落并堆积。
/// Sinks through water, oil, gas, and acid (denser than most liquids).
/// / 下沉穿过水、油、气体和酸（比大多数液体密度大）。
pub fn update_sand(cell: Cell, mut api: SandApi) {
    let dx = api.rand_dir_2();

    let nbr = api.get(0, 1);
    if nbr.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        // Slide diagonally / 对角滑落
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if nbr.species == Species::Water
        || nbr.species == Species::Gas
        || nbr.species == Species::Oil
        || nbr.species == Species::Acid
    {
        // Swap: sand sinks through these fluids / 交换：沙子在这些流体中下沉
        api.set(0, 0, nbr);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

/// Dust: fine powder that explodes under high pressure.
/// / 灰尘：细粉末，高压下会爆炸。
/// Dust explosions are a real industrial hazard (grain silos, etc.).
/// / 粉尘爆炸是真实的工业危害（谷物筒仓等）。
pub fn update_dust(cell: Cell, mut api: SandApi) {
    let dx = api.rand_dir();
    let fluid = api.get_fluid();

    // Dust explosion when pressure exceeds threshold
    // / 当压力超过阈值时发生粉尘爆炸
    if fluid.pressure > 120 {
        api.set(
            0,
            0,
            Cell {
                species: Species::Fire,
                ra: (150 + (cell.ra / 10)) as u8,
                rb: 0,
                clock: 0,
            },
        );
        api.set_fluid(Wind {
            dx: 0,
            dy: 0,
            pressure: 80,
            density: 5,
        });
        return;
    }

    let nbr = api.get(0, 1);
    if nbr.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if nbr.species == Species::Water {
        // Dust sinks in water / 灰尘在水中下沉
        api.set(0, 0, nbr);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

/// Stone: heavy solid that can be crushed into sand under pressure.
/// / 石头：重型固体，在压力下可被压碎为沙子。
/// Forms arches: if supported by stone on both top corners, it stays put.
/// / 形成拱门结构：若两个顶角都有石头支撑，则保持原位。
pub fn update_stone(cell: Cell, mut api: SandApi) {
    // Arch support: stone doesn't fall if supported by other stones above corners
    // / 拱支撑：若上方角落有其他石头支撑，则石头不下落
    if api.get(-1, -1).species == Species::Stone && api.get(1, -1).species == Species::Stone {
        return;
    }
    let fluid = api.get_fluid();

    // Crush into sand under high pressure / 在高压下压碎成沙子
    if fluid.pressure > 120 && api.rand_int(1) == 0 {
        api.set(
            0,
            0,
            Cell {
                species: Species::Sand,
                ra: cell.ra,
                rb: 0,
                clock: 0,
            },
        );
        return;
    }

    let nbr = api.get(0, 1);
    let nbr_species = nbr.species;
    if nbr_species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if nbr_species == Species::Water
        || nbr_species == Species::Gas
        || nbr_species == Species::Oil
        || nbr_species == Species::Acid
    {
        // Sink through lighter fluids / 下沉穿过较轻的流体
        api.set(0, 0, nbr);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

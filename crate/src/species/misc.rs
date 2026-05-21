//! Miscellaneous and toy element simulation: bubble, balloon, confetti, glitter, spring, domino.
//! / 杂项和玩具元素模拟：气泡、气球、彩纸屑、闪光粉、弹簧、多米诺骨牌。
//!
//! These are fun elements with purely aesthetic or toy physics behaviors.
//! / 这些是有趣的元素，具有纯美学或玩具物理行为。

use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

/// Bubble: floats upward with drift, pops on contact with anything, eventually pops on its own.
/// / 气泡：向上浮动并飘移，接触任何东西时破裂，最终自行破裂。
pub fn update_bubble(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species != Species::Empty {
        // Pop!
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    // Float up with drift
    if api.get(dx, -1).species == Species::Empty && api.once_in(3) {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, -1, cell);
    } else if api.get(0, -1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, -1, cell);
    } else {
        api.set(0, 0, cell);
    }
    // Eventually pop
    if api.once_in(200) {
        api.set(0, 0, EMPTY_CELL);
    }
}

/// Balloon: inflated balloon floats up (ra > 20), deflates and falls, bounces off surfaces, pops near fire.
/// / 气球：充气的气球向上浮（ra > 20），放气后下落，在表面上弹跳，靠近火焰时爆裂。
pub fn update_balloon(cell: Cell, mut api: SandApi) {
    let ra = cell.ra; // air inside
    if ra < 20 {
        // Deflated
        let below = api.get(0, 1);
        if below.species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, 1, cell);
        } else {
            api.set(0, 0, cell);
        }
        return;
    }
    // Inflated - floats up
    let (dx, dy) = api.rand_vec();
    let above = api.get(dx, -1);
    if above.species == Species::Empty && api.once_in(2) {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, -1, Cell { ra: ra.saturating_sub(1), ..cell });
    } else if above.species != Species::Empty && above.species != Species::Balloon {
        // Bounce sideways
        if api.get(dx * 2, 0).species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(dx * 2, 0, Cell { ra: ra.saturating_sub(2), ..cell });
        } else {
            api.set(0, 0, cell);
        }
    } else {
        // Drift
        if api.get(dx, 0).species == Species::Empty && api.once_in(5) {
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, 0, cell);
        } else {
            api.set(0, 0, cell);
        }
    }
    // Pops near fire
    let (sx, sy) = api.rand_vec();
    if api.get(sx, sy).species == Species::Fire {
        api.set(0, 0, Cell { species: Species::Fire, ra: 30, rb: 0, clock: 0 });
        return;
    }
}

/// Confetti: colorful paper, floats around randomly, falls slowly, flammable.
/// / 彩纸屑：彩色纸片，随机漂浮，缓慢下落，可燃。
pub fn update_confetti(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    if api.get(dx, dy).species == Species::Empty && api.once_in(2) {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, dy, cell);
    } else {
        // Slowly fall
        let below = api.get(0, 1);
        if below.species == Species::Empty && api.once_in(4) {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, 1, cell);
        } else {
            api.set(0, 0, cell);
        }
    }
    // Burnable
    let (sx, sy) = api.rand_vec();
    if api.get(sx, sy).species == Species::Fire {
        api.set(0, 0, Cell { species: Species::Fire, ra: 20, rb: 0, clock: 0 });
    }
}

/// Glitter: sparkly particles, falls very slowly, drifts sideways easily.
/// / 闪光粉：闪亮颗粒，极慢下落，容易侧向飘移。
pub fn update_glitter(cell: Cell, mut api: SandApi) {
    let dx = api.rand_dir();
    // Very light, drifts
    let below = api.get(0, 1);
    if below.species == Species::Empty && api.once_in(3) {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty && api.once_in(4) {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if api.get(dx, 0).species == Species::Empty && api.once_in(6) {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else {
        api.set(0, 0, cell);
    }
}

/// Spring: bouncy element, launches things on top upward, compresses when stepped on.
/// / 弹簧：有弹性的元素，将上面的物体向上弹射，被踩压时压缩。
pub fn update_spring(cell: Cell, mut api: SandApi) {
    let above = api.get(0, -1);
    if above.species != Species::Empty && above.species != Species::Wall
        && above.species != Species::Spring {
        // Launch it!
        let launch_species = above.species;
        let launch_ra = above.ra;
        api.set(0, -1, EMPTY_CELL);
        // Launch high up
        let target_y = -2;
        if api.get(0, target_y).species == Species::Empty {
            api.set(0, target_y, Cell { species: launch_species, ra: launch_ra, rb: 0, clock: 0 });
        }
    }
    // Spring compresses and releases
    let below = api.get(0, 1);
    if below.species != Species::Empty && api.once_in(5) {
        api.set_fluid(Wind { dx: 0, dy: 100, pressure: 50, density: 10 });
    }
}

/// Domino: stands upright, falls when knocked (rb > 0), triggers chain reaction with neighbors.
/// / 多米诺骨牌：竖直站立，被推倒时（rb > 0）倒下，触发相邻骨牌的连锁反应。
pub fn update_domino(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;
    let (dx, dy) = api.rand_vec_8();
    let nbr = api.get(dx, dy);

    if rb > 0 {
        // Falling over - trigger chain reaction
        api.set(0, 0, Cell { rb: rb - 1, ..cell });
        if rb == 1 {
            api.set(0, 0, EMPTY_CELL);
            // Knock down neighbor if it's a domino
            if nbr.species == Species::Domino && nbr.rb == 0 {
                api.set(dx, dy, Cell { rb: 15, ..nbr });
            }
        }
        return;
    }

    // Standing - check if knocked
    if nbr.species == Species::Domino && nbr.rb > 0 && api.once_in(3) {
        api.set(0, 0, Cell { rb: 15, ..cell });
        return;
    }

    // Stands upright (stationary)
    api.set(0, 0, cell);
}

//! Extended gas simulation: steam, smoke, helium, chlorine, oxygen, hydrogen, plasma, methane.
//! / 扩展气体模拟：蒸汽、烟雾、氦气、氯气、氧气、氢气、等离子体、甲烷。
//!
//! Gases rise, diffuse, and many are flammable or chemically reactive.
//! / 气体上升、扩散，许多是可燃的或具有化学反应性。

use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

/// Helper: generic gas rising and diffusing physics.
/// / 辅助函数：通用气体上升和扩散物理。
fn gas_rise(cell: Cell, api: &mut SandApi, density: u8, flammable: bool) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);

    if nbr.species == Species::Empty {
        // Prefer rising
        let go_up = dy == -1 || dy == 0;
        if go_up {
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, dy, cell);
        } else if api.once_in(3) {
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, dy, cell);
        } else {
            api.set(0, 0, cell);
        }
    } else if nbr.species == Species::Gas && api.once_in(2) {
        // Merge with other gas
    } else {
        api.set(0, 0, cell);
    }

    if flammable {
        let (sx, sy) = api.rand_vec();
        let snbr = api.get(sx, sy);
        if snbr.species == Species::Fire {
            api.set(0, 0, Cell { species: Species::Fire, ra: 200, rb: 0, clock: 0 });
            api.set_fluid(Wind { dx: 0, dy: 0, pressure: 80, density: 60 });
        }
    }
}

/// Steam: hot water vapor, condenses on cool surfaces (ice/snow), eventually condenses.
/// / 蒸汽：热的水蒸气，在冷表面（冰/雪）上凝结，最终会自然凝结。
pub fn update_steam(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Ice || nbr.species == Species::Snow {
        api.set(0, 0, Cell { species: Species::Water, ra: 100, rb: 0, clock: 0 });
        return;
    }
    if api.once_in(200) {
        api.set(0, 0, Cell { species: Species::Water, ra: 100, rb: 0, clock: 0 });
        return;
    }
    // Rises and heats things
    api.set_fluid(Wind { dx: 0, dy: 10, pressure: 0, density: 20 });
    gas_rise(cell, &mut api, 40, false);
}

/// Smoke: rises, dissipates over time (rb = age counter).
/// / 烟雾：上升，随时间消散（rb = 年龄计数器）。
pub fn update_smoke(cell: Cell, mut api: SandApi) {
    let rb = cell.rb.saturating_add(1);
    if rb > 120 {
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    api.set_fluid(Wind { dx: 0, dy: 20, pressure: 0, density: 30 });
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Empty && (dy == -1 || dy == 0 || api.once_in(4)) {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, dy, Cell { rb, ..cell });
    } else {
        api.set(0, 0, Cell { rb, ..cell });
    }
}

/// Helium: very light inert gas, rises rapidly through other elements.
/// / 氦气：极轻的惰性气体，快速上升穿过其他元素。
pub fn update_helium(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    if dy == -1 {
        let nbr = api.get(dx, -1);
        if nbr.species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, -1, cell);
            return;
        }
    }
    gas_rise(cell, &mut api, 10, false);
}

/// Chlorine: toxic heavy gas, kills organic life, tends to sink rather than rise.
/// / 氯气：有毒重气体，杀死有机生命，倾向于下沉而非上升。
pub fn update_chlorine(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if matches!(nbr.species,
        Species::Plant | Species::Wood | Species::Fungus | Species::Mite
        | Species::Seed | Species::Grass | Species::Flower | Species::Moss
        | Species::Leaf | Species::Vine | Species::Mushroom | Species::Fruit
        | Species::Ant | Species::Spider | Species::Bee | Species::Butterfly
        | Species::Worm | Species::Fish | Species::Bird | Species::Snake) {
        api.set(dx, dy, EMPTY_CELL);
    }
    // Tends to sink rather than rise
    let below = api.get(0, 1);
    if below.species == Species::Empty && api.once_in(2) {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else {
        gas_rise(cell, &mut api, 60, false);
    }
}

/// Oxygen: feeds fire, making it burn hotter and spread faster.
/// / 氧气：助燃火焰，使其燃烧更旺、扩散更快。
pub fn update_oxygen(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire {
        // Fire grows stronger near oxygen
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 40, density: 80 });
        if api.once_in(5) {
            api.set(0, 0, EMPTY_CELL); // consumed
            return;
        }
    }
    gas_rise(cell, &mut api, 30, false);
}

/// Hydrogen: very light and extremely flammable, produces powerful explosions.
/// / 氢气：极轻且极度易燃，产生强力爆炸。
pub fn update_hydrogen(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava || nbr.species == Species::Lightning {
        api.set(0, 0, Cell { species: Species::Fire, ra: 250, rb: 0, clock: 0 });
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 150, density: 100 });
        return;
    }
    // Very light
    if api.get(0, -1).species == Species::Empty && api.once_in(1) {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, -1, cell);
    } else {
        gas_rise(cell, &mut api, 5, true);
    }
}

/// Plasma: super-hot ionized gas, destroys nearly everything, short-lived.
/// / 等离子体：超高温电离气体，几乎摧毁一切，寿命短暂。
pub fn update_plasma(cell: Cell, mut api: SandApi) {
    api.set_fluid(Wind { dx: 0, dy: 100, pressure: 50, density: 200 });
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species != Species::Empty && nbr.species != Species::Wall
        && nbr.species != Species::PlasmaGas && nbr.species != Species::Void {
        api.set(dx, dy, Cell { species: Species::Fire, ra: 200, rb: 0, clock: 0 });
    }
    // Short-lived
    if api.once_in(40) {
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    let (mdx, mdy) = api.rand_vec();
    if api.get(mdx, mdy).species == Species::Empty && api.once_in(3) {
        api.set(0, 0, EMPTY_CELL);
        api.set(mdx, mdy, cell);
    } else {
        gas_rise(cell, &mut api, 15, false);
    }
}

/// Methane: flammable greenhouse gas, burns intensely with fire/lava.
/// / 甲烷：可燃温室气体，遇火焰/岩浆剧烈燃烧。
pub fn update_methane(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Fire, ra: 220, rb: 0, clock: 0 });
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 100, density: 70 });
        return;
    }
    gas_rise(cell, &mut api, 25, true);
}

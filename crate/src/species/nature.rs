//! Natural material simulation: clay, soil, peat, limestone, chalk, shale, slate, sandstone.
//! / 自然材料模拟：粘土、土壤、泥炭、石灰岩、白垩、页岩、板岩、砂岩。
//!
//! These represent earth materials with erosion, absorption, plant support, and geological behaviors.
//! / 这些代表具有侵蚀、吸收、植物支持和地质行为的土质材料。

use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

/// Helper: natural material falling physics with optional crumbling under pressure.
/// / 辅助函数：自然材料下落物理，压力下可选碎裂。
fn nature_fall(cell: Cell, api: &mut SandApi, crumbles: bool, crush_resist: i32) {
    let fluid = api.get_fluid();
    if crumbles && fluid.pressure > 130 && api.once_in(crush_resist) {
        api.set(0, 0, Cell { species: Species::Sand, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if below.species == Species::Water || below.species == Species::Oil {
        api.set(0, 0, below);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

/// Clay: moldable, hardens into brick near fire/lava, absorbs water.
/// / 粘土：可塑，在火焰/岩浆附近硬化成砖，吸收水分。
pub fn update_clay(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if (nbr.species == Species::Fire || nbr.species == Species::Lava) && api.once_in(30) {
        api.set(0, 0, Cell { species: Species::Brick, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }
    // Absorbs water
    if nbr.species == Species::Water && api.once_in(10) {
        api.set(dx, dy, EMPTY_CELL);
        api.set(0, 0, Cell { ra: cell.ra.saturating_add(5), ..cell });
    }
    nature_fall(cell, &mut api, false, 0);
}

/// Soil: base medium for plant growth, holds water, supports seeds taking root.
/// / 土壤：植物生长的基础介质，保水，支持种子生根。
pub fn update_soil(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Seed && api.once_in(30) {
        // Seeds can take root
        api.set(0, 0, cell);
    }
    if nbr.species == Species::Water && api.once_in(15) {
        // Absorbs and retains water
        api.set(0, 0, Cell { ra: cell.ra.saturating_add(2), ..cell });
    }
    nature_fall(cell, &mut api, false, 0);
}

/// Peat: organic soil, highly flammable, great for accelerating seed growth.
/// / 泥炭：有机土壤，高度可燃，极利于加速种子生长。
pub fn update_peat(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Fire, ra: 60, rb: 0, clock: 0 });
        return;
    }
    // Great for plants
    if nbr.species == Species::Seed && api.once_in(20) {
        let seed = nbr;
        api.set(dx, dy, Cell { ra: seed.ra + 20, rb: 1, ..seed });
    }
    nature_fall(cell, &mut api, false, 0);
}

/// Limestone: sedimentary rock, dissolves in acid, slowly eroded by water.
/// / 石灰岩：沉积岩，溶于酸，被水缓慢侵蚀。
pub fn update_limestone(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Acid && api.once_in(15) {
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    if nbr.species == Species::Water && api.once_in(50) {
        api.set(0, 0, Cell { ra: cell.ra.saturating_sub(1), ..cell });
    }
    nature_fall(cell, &mut api, true, 10);
}

/// Chalk: soft sedimentary material, dissolves quickly in acid, crumbles easily.
/// / 白垩：柔软的沉积材料，在酸中迅速溶解，容易碎裂。
pub fn update_chalk(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Acid && api.once_in(5) {
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    // Soft: crumbles easily
    let fluid = api.get_fluid();
    if fluid.pressure > 80 && api.once_in(5) {
        api.set(0, 0, Cell { species: Species::Dust, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }
    nature_fall(cell, &mut api, true, 3);
}

/// Shale: layered sedimentary rock, splits into sand under pressure.
/// / 页岩：层状沉积岩，在压力下碎裂成沙子。
pub fn update_shale(cell: Cell, mut api: SandApi) {
    let fluid = api.get_fluid();
    if fluid.pressure > 100 && api.once_in(8) {
        api.set(0, 0, Cell { species: Species::Sand, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }
    nature_fall(cell, &mut api, true, 8);
}

/// Slate: metamorphic rock with flat layers, good crush resistance.
/// / 板岩：变质岩，平整层状，良好的抗压性。
pub fn update_slate(cell: Cell, mut api: SandApi) {
    nature_fall(cell, &mut api, true, 12);
}

/// Sandstone: cemented sand grains, porous (eroded by water), crumbles under pressure.
/// / 砂岩：胶结的沙粒，多孔（被水侵蚀），压力下碎裂。
pub fn update_sandstone(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Water && api.once_in(30) {
        api.set(0, 0, Cell { ra: cell.ra.saturating_sub(2), ..cell });
    }
    let fluid = api.get_fluid();
    if fluid.pressure > 110 && api.once_in(8) {
        api.set(0, 0, Cell { species: Species::Sand, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }
    nature_fall(cell, &mut api, true, 8);
}

//! Construction material simulation: brick, concrete, cement, tile, plaster, marble, granite, basalt.
//! / 建筑材料模拟：砖、混凝土、水泥、瓷砖、灰泥、大理石、花岗岩、玄武岩。
//!
//! Construction materials are solid structural elements with varying crush resistance and melt points.
//! / 建筑材料是具有不同抗压强度和熔点的固体结构元素。

use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

/// Helper: construction solid physics - heavy, crushes to sand, melts to lava.
/// / 辅助函数：建筑材料物理 - 沉重，可压碎为沙子，熔化后变成岩浆。
fn construction_solid(cell: Cell, api: &mut SandApi, crush_resist: i32, melt_temp: i32) {
    let fluid = api.get_fluid();
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);

    // Crush under pressure
    if fluid.pressure > 150 && api.once_in(crush_resist) {
        api.set(0, 0, Cell { species: Species::Sand, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }
    // Melt
    if melt_temp > 0 && nbr.species == Species::Lava && api.once_in(melt_temp) {
        api.set(0, 0, Cell { species: Species::Lava, ra: 100, rb: 0, clock: 0 });
        return;
    }
    // Fall
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if below.species == Species::Water || below.species == Species::Oil
        || below.species == Species::Gas || below.species == Species::Acid {
        api.set(0, 0, below);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

/// Brick: fired clay block, moderate strength.
/// / 砖：烧制粘土块，中等强度。
pub fn update_brick(cell: Cell, mut api: SandApi) {
    construction_solid(cell, &mut api, 15, 40);
}

/// Concrete: strong structural material, high crush resistance.
/// / 混凝土：坚固的结构材料，高抗压性。
pub fn update_concrete(cell: Cell, mut api: SandApi) {
    construction_solid(cell, &mut api, 25, 50);
}

/// Cement: powder-like, hardens into concrete when mixed with water (curing process).
/// / 水泥：粉末状，与水混合后硬化成混凝土（固化过程）。
/// rb = curing timer: 0=dry powder, 1=fully cured->concrete, >1=curing.
/// / rb = 固化计时器：0=干粉, 1=完全固化变成混凝土, >1=固化中。
pub fn update_cement(cell: Cell, mut api: SandApi) {
    let rb = cell.rb; // curing timer
    if rb > 1 {
        // Hardening
        api.set(0, 0, Cell { rb: rb - 1, ..cell });
        return;
    } else if rb == 1 {
        api.set(0, 0, Cell { species: Species::Concrete, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }
    // Check for water to start curing
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Water {
        api.set(dx, dy, EMPTY_CELL);
        api.set(0, 0, Cell { rb: 30, ..cell });
        return;
    }
    // Falls like powder
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else {
        let sfx = api.rand_dir_2();
        if api.get(sfx, 1).species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(sfx, 1, cell);
        } else {
            api.set(0, 0, cell);
        }
    }
}

/// Tile: ceramic tile, good strength and moderate heat resistance.
/// / 瓷砖：陶瓷砖，良好强度和中等的耐热性。
pub fn update_tile(cell: Cell, mut api: SandApi) {
    construction_solid(cell, &mut api, 20, 45);
}

/// Plaster: soft construction material, eroded by water, dissolved by acid.
/// / 灰泥：柔软的建筑材料，被水侵蚀，被酸溶解。
pub fn update_plaster(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Water && api.once_in(30) {
        api.set(0, 0, Cell { species: Species::Mud, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }
    if nbr.species == Species::Acid && api.once_in(10) {
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    construction_solid(cell, &mut api, 8, 0);
}

/// Marble: beautiful stone, slowly dissolved by acid (calcium carbonate reaction).
/// / 大理石：美丽的石头，会被酸缓慢溶解（碳酸钙反应）。
pub fn update_marble(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Acid && api.once_in(25) {
        api.set(0, 0, Cell { ra: cell.ra.saturating_sub(10), ..cell });
    }
    construction_solid(cell, &mut api, 18, 60);
}

/// Granite: very strong igneous rock, high crush resistance.
/// / 花岗岩：极坚固的火成岩，高抗压性。
pub fn update_granite(cell: Cell, mut api: SandApi) {
    construction_solid(cell, &mut api, 30, 70);
}

/// Basalt: volcanic rock, the strongest construction material, extremely lava-resistant.
/// / 玄武岩：火山岩，最强的建筑材料，极耐岩浆。
pub fn update_basalt(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Lava {
        // Already cooled lava, very resistant
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 0, density: 20 });
    }
    construction_solid(cell, &mut api, 35, 80);
}

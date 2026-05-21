//! Extended liquid simulation: mud, blood, honey, milk, poison, mercury, alcohol, syrup.
//! / 扩展液体模拟：泥浆、血液、蜂蜜、牛奶、毒药、汞、酒精、糖浆。
//!
//! These liquids have varying viscosity, special reactions, and density-based interactions.
//! / 这些液体具有不同的粘度、特殊反应和基于密度的相互作用。

use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

/// Helper: generic liquid flow physics.
/// / 辅助函数：通用液体流动物理。
/// viscosity: higher = flows slower / 越高 = 流动越慢
/// heavier_than_water: sinks through water / 比水重：沉入水中
fn liquid_flow(cell: Cell, api: &mut SandApi, viscosity: i32, heavier_than_water: bool) {
    let mut dx = api.rand_dir();
    let below = api.get(0, 1);
    let dx1 = api.get(dx, 1);

    let sinks = heavier_than_water;

    if below.species == Species::Empty {
        api.set(0, 0, below);
        let mut ra = cell.ra;
        if api.once_in(20) { ra = 100 + api.rand_int(50) as u8; }
        api.set(0, 1, Cell { ra, ..cell });
        return;
    } else if !sinks && below.species == Species::Water {
        // Floats on water / 浮在水上
        api.set(0, 0, cell);
        return;
    } else if sinks && (below.species == Species::Water || below.species == Species::Oil) {
        // Sinks through lighter liquids / 沉入较轻的液体中
        api.set(0, 0, below);
        api.set(0, 1, cell);
        return;
    } else if dx1.species == Species::Empty && api.once_in(viscosity) {
        api.set(0, 0, dx1);
        api.set(dx, 1, cell);
        return;
    } else if api.get(-dx, 1).species == Species::Empty && api.once_in(viscosity) {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 1, cell);
        return;
    }

    // Sideways spread based on viscosity
    if api.get(dx, 0).species == Species::Empty && api.once_in(viscosity * 2) {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else if api.get(-dx, 0).species == Species::Empty && api.once_in(viscosity * 2) {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 0, cell);
    } else {
        api.set(0, 0, cell);
    }
}

/// Mud: thick slow-flowing liquid, dries out into soil, fire bakes it.
/// / 泥浆：浓稠缓慢流动的液体，会干燥成土壤，火烘烤后可硬化。
pub fn update_mud(cell: Cell, mut api: SandApi) {
    if api.once_in(200) {
        api.set(0, 0, Cell { species: Species::Soil, ra: cell.ra, rb: 0, clock: 0 }); // dries out / 干燥
        return;
    }
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire && api.once_in(50) {
        api.set(0, 0, Cell { species: Species::Soil, ra: 50, rb: 0, clock: 0 }); // baked by fire / 被火烧硬
        return;
    }
    liquid_flow(cell, &mut api, 3, true);
}

/// Blood: like water but slightly thicker, organic, dries out over time.
/// / 血液：像水但略浓稠，有机物，随时间干燥。
pub fn update_blood(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Fire, ra: 40, rb: 0, clock: 0 });
        return;
    }
    if api.once_in(150) {
        api.set(0, 0, EMPTY_CELL); // evaporates / 蒸发
        return;
    }
    liquid_flow(cell, &mut api, 1, false);
}

/// Honey: very thick sticky liquid, flammable, edible by ants.
/// / 蜂蜜：非常浓稠粘稠的液体，可燃，蚂蚁可食用。
pub fn update_honey(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Fire, ra: 150, rb: 0, clock: 0 });
        return;
    }
    // Ants consume honey / 蚂蚁消耗蜂蜜
    if nbr.species == Species::Ant && api.once_in(20) {
        api.set(0, 0, EMPTY_CELL);
    }
    liquid_flow(cell, &mut api, 5, true);
}

/// Milk: spoils near heat, neutralizes acid on contact.
/// / 牛奶：在热附近变质，接触时中和酸。
pub fn update_milk(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Acid {
        // Milk neutralizes acid: both disappear / 牛奶中和酸：两者都消失
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, dy, EMPTY_CELL);
        return;
    }
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Fire, ra: 30, rb: 0, clock: 0 });
        return;
    }
    liquid_flow(cell, &mut api, 1, false);
}

/// Poison: kills organic things on contact (plants, insects, creatures).
/// / 毒药：接触时杀死有机物（植物、昆虫、生物）。
pub fn update_poison(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if matches!(nbr.species,
        Species::Plant | Species::Wood | Species::Fungus | Species::Mite
        | Species::Seed | Species::Moss | Species::Grass | Species::Flower
        | Species::Leaf | Species::Vine | Species::Mushroom | Species::Fruit
        | Species::Worm | Species::Ant | Species::Spider | Species::Bee
        | Species::Butterfly | Species::Fish) {
        api.set(dx, dy, EMPTY_CELL); // destroys organic life / 消灭有机生命
    }
    liquid_flow(cell, &mut api, 1, false);
}

/// Mercury: very heavy liquid metal, dissolves gold (amalgam).
/// / 汞：极重的液态金属，溶解黄金（汞齐化）。
pub fn update_mercury(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if matches!(nbr.species, Species::Gold) {
        // Mercury dissolves gold into amalgam / 汞将黄金溶解成汞齐
        api.set(dx, dy, EMPTY_CELL);
    }
    liquid_flow(cell, &mut api, 1, true);
}

/// Alcohol: highly flammable thin liquid, evaporates over time.
/// / 酒精：高度易燃的稀薄液体，随时间蒸发。
pub fn update_alcohol(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Fire, ra: 200, rb: 0, clock: 0 });
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 60, density: 60 });
        return;
    }
    if api.once_in(100) {
        api.set(0, 0, Cell { species: Species::Gas, ra: 100, rb: 0, clock: 0 }); // evaporates / 蒸发
        return;
    }
    liquid_flow(cell, &mut api, 1, false);
}

/// Syrup: very sticky slow-flowing sweet liquid, flammable.
/// / 糖浆：非常粘稠缓慢流动的甜液体，可燃。
pub fn update_syrup(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire {
        api.set(0, 0, Cell { species: Species::Fire, ra: 100, rb: 0, clock: 0 });
        return;
    }
    liquid_flow(cell, &mut api, 4, true);
}

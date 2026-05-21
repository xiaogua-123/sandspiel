//! Metal element simulation: iron, copper, gold, silver, aluminum, lead, zinc, tin, bronze, steel.
//! / 金属元素模拟：铁、铜、金、银、铝、铅、锌、锡、青铜、钢。
//!
//! Metals are heavy solids that sink through most liquids, conduct heat,
//! and many have unique reactions (rusting, tarnishing, amalgamation, etc.).
//! / 金属是重型固体，会沉入大多数液体中，导热，
//! / 许多金属有独特的反应（生锈、失去光泽、汞齐化等）。

use crate::{Cell, SandApi, EMPTY_CELL};
use super::Species;

/// Helper: heavy falling solid that sinks through most liquids.
/// / 辅助函数：会沉入大多数液体中的重型下落固体。
/// melt_temp: how resistant to melting (0 = doesn't melt).
/// / melt_temp: 抗熔化程度（0 = 不熔化）。
/// melt_into: what species to become when melted.
/// / melt_into: 熔化后变成什么物种。
fn heavy_fall(cell: Cell, api: &mut SandApi, melt_temp: u8, melt_into: Species) {
    // Check if near fire/lava for melting
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if melt_temp > 0 && (nbr.species == Species::Fire || nbr.species == Species::Lava) {
        if api.once_in(melt_temp as i32) {
            api.set(0, 0, Cell { species: melt_into, ra: cell.ra, rb: 0, clock: 0 });
            return;
        }
    }
    // Heavy: sinks through lighter liquids / 重：沉入较轻的液体中
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if below.species == Species::Water || below.species == Species::Oil
        || below.species == Species::Gas || below.species == Species::Acid {
        api.set(0, 0, below);
        api.set(0, 1, cell);
    } else {
        let dxf = api.rand_dir_2();
        if api.get(dxf, 1).species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(dxf, 1, cell);
        } else {
            api.set(0, 0, cell);
        }
    }
}

/// Iron: heavy ferromagnetic metal, rusts in water, melts in extreme heat.
/// / 铁：重型铁磁金属，在水中生锈，极端高温下熔化。
pub fn update_iron(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec_8();
    let nbr = api.get(dx, dy);
    if (nbr.species == Species::Lava) && api.once_in(20) {
        api.set(0, 0, Cell { species: Species::Lava, ra: 100, rb: 0, clock: 0 });
        return;
    }
    // Iron rusts in water, gradually degrading / 铁在水中生锈，逐渐降解
    if nbr.species == Species::Water && api.once_in(50) {
        api.set(0, 0, Cell { ra: cell.ra.saturating_sub(2), ..cell });
    }
    heavy_fall(cell, &mut api, 0, Species::Empty);
}

/// Copper: conducts heat, turns green (patina) near acid.
/// / 铜：导热，在酸附近变绿（铜绿）。
pub fn update_copper(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec_8();
    let nbr = api.get(dx, dy);
    if (nbr.species == Species::Lava) && api.once_in(25) {
        api.set(0, 0, Cell { species: Species::Lava, ra: 120, rb: 0, clock: 0 });
        return;
    }
    if nbr.species == Species::Acid && api.once_in(30) {
        api.set(0, 0, Cell { ra: cell.ra.saturating_sub(5), ..cell });
    }
    heavy_fall(cell, &mut api, 0, Species::Empty);
}

/// Gold: very dense, inert noble metal, sinks through many liquids.
/// / 金：密度极高的惰性贵金属，可沉入多种液体中。
pub fn update_gold(cell: Cell, mut api: SandApi) {
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if matches!(below.species, Species::Water | Species::Oil | Species::Gas | Species::Acid
        | Species::Mud | Species::Blood | Species::Honey | Species::Milk | Species::Poison
        | Species::Alcohol | Species::Syrup) {
        api.set(0, 0, below);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

/// Silver: tarnishes near acid, conductive, melts at moderate heat.
/// / 银：在酸附近失去光泽，导电，中等温度下熔化。
pub fn update_silver(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec_8();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Acid && api.once_in(20) {
        api.set(0, 0, Cell { ra: cell.ra.saturating_sub(8), ..cell });
    }
    heavy_fall(cell, &mut api, 30, Species::Lava);
}

/// Aluminum: lightweight metal, melts at moderate heat, doesn't sink through all liquids.
/// / 铝：轻质金属，中等温度下熔化，不会沉入所有液体中。
pub fn update_aluminum(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec_8();
    let nbr = api.get(dx, dy);
    if (nbr.species == Species::Fire || nbr.species == Species::Lava) && api.once_in(12) {
        api.set(0, 0, Cell { species: Species::Lava, ra: 80, rb: 0, clock: 0 });
        return;
    }
    // Light: doesn't sink through all liquids
    let below = api.get(0, 1);
    if below.species == Species::Empty || below.species == Species::Gas {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else {
        let dxf = api.rand_dir_2();
        if api.get(dxf, 1).species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(dxf, 1, cell);
        } else {
            api.set(0, 0, cell);
        }
    }
}

/// Lead: very dense, toxic (poisons nearby water), low melting point.
/// / 铅：密度极大，有毒（污染附近的水），低熔点。
pub fn update_lead(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec_8();
    let nbr = api.get(dx, dy);
    if (nbr.species == Species::Fire || nbr.species == Species::Lava) && api.once_in(15) {
        api.set(0, 0, Cell { species: Species::Lava, ra: 100, rb: 0, clock: 0 });
        return;
    }
    // Toxic: poisons nearby water
    if nbr.species == Species::Water && api.once_in(40) {
        api.set(dx, dy, Cell { species: Species::Poison, ra: 100, rb: 0, clock: 0 });
    }
    heavy_fall(cell, &mut api, 0, Species::Empty);
}

/// Zinc: brittle metal, reacts vigorously with acid releasing gas.
/// / 锌：脆性金属，与酸剧烈反应释放气体。
pub fn update_zinc(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec_8();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Acid && api.once_in(10) {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, dy, Cell { species: Species::Gas, ra: 100, rb: 0, clock: 0 });
        return;
    }
    if nbr.species == Species::Lava && api.once_in(20) {
        api.set(0, 0, Cell { species: Species::Lava, ra: 90, rb: 0, clock: 0 });
        return;
    }
    heavy_fall(cell, &mut api, 0, Species::Empty);
}

/// Tin: soft metal with very low melting point.
/// / 锡：软金属，熔点极低。
pub fn update_tin(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec_8();
    let nbr = api.get(dx, dy);
    if (nbr.species == Species::Fire || nbr.species == Species::Lava) && api.once_in(10) {
        api.set(0, 0, Cell { species: Species::Lava, ra: 70, rb: 0, clock: 0 });
        return;
    }
    heavy_fall(cell, &mut api, 0, Species::Empty);
}

/// Bronze: strong copper-tin alloy, resistant to melting.
/// / 青铜：坚固的铜锡合金，抗熔化。
pub fn update_bronze(cell: Cell, mut api: SandApi) {
    heavy_fall(cell, &mut api, 35, Species::Lava);
}

/// Steel: strongest metal, very resistant to heat and melting.
/// / 钢：最强金属，极耐热和熔化。
pub fn update_steel(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec_8();
    let nbr = api.get(dx, dy);
    if (nbr.species == Species::Lava) && api.once_in(40) {
        api.set(0, 0, Cell { species: Species::Lava, ra: 120, rb: 0, clock: 0 });
        return;
    }
    heavy_fall(cell, &mut api, 0, Species::Empty);
}

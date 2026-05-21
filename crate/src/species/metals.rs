use crate::{Cell, SandApi, EMPTY_CELL};
use super::Species;

/// Helper: heavy falling solid that sinks through most liquids
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
    // Heavy: sinks through lighter liquids
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

pub fn update_iron(cell: Cell, mut api: SandApi) {
    // Iron: heavy, rusts in water, melts in extreme heat
    let (dx, dy) = api.rand_vec_8();
    let nbr = api.get(dx, dy);
    if (nbr.species == Species::Lava) && api.once_in(20) {
        api.set(0, 0, Cell { species: Species::Lava, ra: 100, rb: 0, clock: 0 });
        return;
    }
    // Rust in water
    if nbr.species == Species::Water && api.once_in(50) {
        api.set(0, 0, Cell { ra: cell.ra.saturating_sub(2), ..cell });
    }
    heavy_fall(cell, &mut api, 0, Species::Empty);
}

pub fn update_copper(cell: Cell, mut api: SandApi) {
    // Copper: conducts heat, turns green near acid
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

pub fn update_gold(cell: Cell, mut api: SandApi) {
    // Gold: very dense, inert, doesn't react much
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

pub fn update_silver(cell: Cell, mut api: SandApi) {
    // Silver: tarnishes near acid, conductive
    let (dx, dy) = api.rand_vec_8();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Acid && api.once_in(20) {
        api.set(0, 0, Cell { ra: cell.ra.saturating_sub(8), ..cell });
    }
    heavy_fall(cell, &mut api, 30, Species::Lava);
}

pub fn update_aluminum(cell: Cell, mut api: SandApi) {
    // Aluminum: lightweight, melts at moderate heat
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

pub fn update_lead(cell: Cell, mut api: SandApi) {
    // Lead: very dense, toxic, melts
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

pub fn update_zinc(cell: Cell, mut api: SandApi) {
    // Zinc: brittle, reacts with acid
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

pub fn update_tin(cell: Cell, mut api: SandApi) {
    // Tin: soft, low melting point
    let (dx, dy) = api.rand_vec_8();
    let nbr = api.get(dx, dy);
    if (nbr.species == Species::Fire || nbr.species == Species::Lava) && api.once_in(10) {
        api.set(0, 0, Cell { species: Species::Lava, ra: 70, rb: 0, clock: 0 });
        return;
    }
    heavy_fall(cell, &mut api, 0, Species::Empty);
}

pub fn update_bronze(cell: Cell, mut api: SandApi) {
    // Bronze: strong alloy, resistant
    heavy_fall(cell, &mut api, 35, Species::Lava);
}

pub fn update_steel(cell: Cell, mut api: SandApi) {
    // Steel: strongest, very resistant to heat
    let (dx, dy) = api.rand_vec_8();
    let nbr = api.get(dx, dy);
    if (nbr.species == Species::Lava) && api.once_in(40) {
        api.set(0, 0, Cell { species: Species::Lava, ra: 120, rb: 0, clock: 0 });
        return;
    }
    heavy_fall(cell, &mut api, 0, Species::Empty);
}

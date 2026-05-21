use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

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

pub fn update_clay(cell: Cell, mut api: SandApi) {
    // Clay: moldable, hardens near heat
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

pub fn update_soil(cell: Cell, mut api: SandApi) {
    // Soil: base for plants, holds water
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

pub fn update_peat(cell: Cell, mut api: SandApi) {
    // Peat: organic soil, flammable
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

pub fn update_limestone(cell: Cell, mut api: SandApi) {
    // Limestone: sedimentary, dissolves in acid
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

pub fn update_chalk(cell: Cell, mut api: SandApi) {
    // Chalk: soft, marks surfaces
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

pub fn update_shale(cell: Cell, mut api: SandApi) {
    // Shale: layered rock, splits easily
    let fluid = api.get_fluid();
    if fluid.pressure > 100 && api.once_in(8) {
        api.set(0, 0, Cell { species: Species::Sand, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }
    nature_fall(cell, &mut api, true, 8);
}

pub fn update_slate(cell: Cell, mut api: SandApi) {
    // Slate: metamorphic, flat layers
    nature_fall(cell, &mut api, true, 12);
}

pub fn update_sandstone(cell: Cell, mut api: SandApi) {
    // Sandstone: cemented sand, porous
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

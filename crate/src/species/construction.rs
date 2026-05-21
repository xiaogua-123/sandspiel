use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

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

pub fn update_brick(cell: Cell, mut api: SandApi) {
    construction_solid(cell, &mut api, 15, 40);
}

pub fn update_concrete(cell: Cell, mut api: SandApi) {
    construction_solid(cell, &mut api, 25, 50);
}

pub fn update_cement(cell: Cell, mut api: SandApi) {
    // Cement: powder-like, hardens when wet
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

pub fn update_tile(cell: Cell, mut api: SandApi) {
    construction_solid(cell, &mut api, 20, 45);
}

pub fn update_plaster(cell: Cell, mut api: SandApi) {
    // Plaster: soft, damaged by water
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

pub fn update_marble(cell: Cell, mut api: SandApi) {
    // Marble: beautiful stone, acid dissolves it
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Acid && api.once_in(25) {
        api.set(0, 0, Cell { ra: cell.ra.saturating_sub(10), ..cell });
    }
    construction_solid(cell, &mut api, 18, 60);
}

pub fn update_granite(cell: Cell, mut api: SandApi) {
    construction_solid(cell, &mut api, 30, 70);
}

pub fn update_basalt(cell: Cell, mut api: SandApi) {
    // Basalt: volcanic rock, strongest construction material
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Lava {
        // Already cooled lava, very resistant
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 0, density: 20 });
    }
    construction_solid(cell, &mut api, 35, 80);
}

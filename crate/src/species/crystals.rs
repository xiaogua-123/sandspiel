use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

fn crystal_fall(cell: Cell, api: &mut SandApi, shatter_resist: i32, melt_temp: i32, melt_into: Species) {
    let (dx, dy) = api.rand_vec_8();
    let nbr = api.get(dx, dy);
    let fluid = api.get_fluid();

    // Melt near extreme heat
    if melt_temp > 0 && api.once_in(melt_temp) && (nbr.species == Species::Lava
        || (nbr.species == Species::Fire && api.once_in(3))) {
        api.set(0, 0, Cell { species: melt_into, ra: 100, rb: 0, clock: 0 });
        return;
    }
    // Shatter under high pressure
    if fluid.pressure > 150 && api.once_in(shatter_resist) {
        api.set(0, 0, Cell { species: Species::Sand, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }
    // Fall like stone
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

pub fn update_diamond(cell: Cell, mut api: SandApi) {
    crystal_fall(cell, &mut api, 30, 60, Species::Lava); // hardest to shatter
}

pub fn update_ruby(cell: Cell, mut api: SandApi) {
    let rdx = api.rand_dir();
    let rdy = api.rand_dir();
    let nbr = api.get(rdx, rdy);
    if nbr.species == Species::Laser {
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 0, density: 200 });
    }
    crystal_fall(cell, &mut api, 20, 50, Species::Lava);
}

pub fn update_sapphire(cell: Cell, mut api: SandApi) {
    crystal_fall(cell, &mut api, 22, 55, Species::Lava);
}

pub fn update_emerald(cell: Cell, mut api: SandApi) {
    // Emerald: slightly more fragile
    let fluid = api.get_fluid();
    if fluid.pressure > 130 && api.once_in(15) {
        api.set(0, 0, Cell { species: Species::Sand, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }
    crystal_fall(cell, &mut api, 15, 45, Species::Lava);
}

pub fn update_amethyst(cell: Cell, mut api: SandApi) {
    crystal_fall(cell, &mut api, 18, 40, Species::Lava);
}

pub fn update_quartz(cell: Cell, mut api: SandApi) {
    // Quartz: piezoelectric - generates sparks under pressure
    let fluid = api.get_fluid();
    if fluid.pressure > 100 && api.once_in(15) {
        api.set_fluid(Wind { dx: 50, dy: 50, pressure: 30, density: 10 });
    }
    crystal_fall(cell, &mut api, 12, 35, Species::Lava);
}

pub fn update_crystal(cell: Cell, mut api: SandApi) {
    // Regular crystal: fragile, low melt point
    crystal_fall(cell, &mut api, 10, 25, Species::Glass);
}

pub fn update_obsidian(cell: Cell, mut api: SandApi) {
    // Obsidian: volcanic glass, sharp, shatters into sharp fragments
    let fluid = api.get_fluid();
    if fluid.pressure > 140 && api.once_in(14) {
        // Shatters into sharp glass-like fragments
        let (sdx, sdy) = api.rand_vec();
        api.set(sdx, sdy, Cell { species: Species::Sand, ra: 50, rb: 0, clock: 0 });
        api.set(0, 0, Cell { species: Species::Sand, ra: 50, rb: 0, clock: 0 });
        return;
    }
    // Near water: cools and cracks
    let odx = api.rand_dir();
    let ody = api.rand_dir();
    let nbr = api.get(odx, ody);
    if nbr.species == Species::Water && api.once_in(20) {
        api.set(0, 0, Cell { species: Species::Stone, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }
    crystal_fall(cell, &mut api, 14, 45, Species::Lava);
}

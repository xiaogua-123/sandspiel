use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

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
        // Floats on water
        api.set(0, 0, cell);
        return;
    } else if sinks && (below.species == Species::Water || below.species == Species::Oil) {
        // Sinks through lighter liquids
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

pub fn update_mud(cell: Cell, mut api: SandApi) {
    // Mud: thick, slow-flowing, dries out
    if api.once_in(200) {
        api.set(0, 0, Cell { species: Species::Soil, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire && api.once_in(50) {
        api.set(0, 0, Cell { species: Species::Soil, ra: 50, rb: 0, clock: 0 });
        return;
    }
    liquid_flow(cell, &mut api, 3, true);
}

pub fn update_blood(cell: Cell, mut api: SandApi) {
    // Blood: like water but slightly thicker, attracts mites
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Fire, ra: 40, rb: 0, clock: 0 });
        return;
    }
    // Dries out
    if api.once_in(150) {
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    liquid_flow(cell, &mut api, 1, false);
}

pub fn update_honey(cell: Cell, mut api: SandApi) {
    // Honey: very thick, sticky, flammable
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Fire, ra: 150, rb: 0, clock: 0 });
        return;
    }
    // Ants love honey
    if nbr.species == Species::Ant && api.once_in(20) {
        api.set(0, 0, EMPTY_CELL);
    }
    liquid_flow(cell, &mut api, 5, true);
}

pub fn update_milk(cell: Cell, mut api: SandApi) {
    // Milk: spoils near heat, neutralizes acid
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Acid {
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

pub fn update_poison(cell: Cell, mut api: SandApi) {
    // Poison: kills organic things, green toxic liquid
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if matches!(nbr.species,
        Species::Plant | Species::Wood | Species::Fungus | Species::Mite
        | Species::Seed | Species::Moss | Species::Grass | Species::Flower
        | Species::Leaf | Species::Vine | Species::Mushroom | Species::Fruit
        | Species::Worm | Species::Ant | Species::Spider | Species::Bee
        | Species::Butterfly | Species::Fish) {
        api.set(dx, dy, EMPTY_CELL);
    }
    liquid_flow(cell, &mut api, 1, false);
}

pub fn update_mercury(cell: Cell, mut api: SandApi) {
    // Mercury: very heavy, toxic liquid metal
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if matches!(nbr.species, Species::Gold) {
        // Amalgam: mercury dissolves gold
        api.set(dx, dy, EMPTY_CELL);
    }
    liquid_flow(cell, &mut api, 1, true);
}

pub fn update_alcohol(cell: Cell, mut api: SandApi) {
    // Alcohol: highly flammable, thin liquid
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Fire, ra: 200, rb: 0, clock: 0 });
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 60, density: 60 });
        return;
    }
    // Evaporates
    if api.once_in(100) {
        api.set(0, 0, Cell { species: Species::Gas, ra: 100, rb: 0, clock: 0 });
        return;
    }
    liquid_flow(cell, &mut api, 1, false);
}

pub fn update_syrup(cell: Cell, mut api: SandApi) {
    // Syrup: very sticky, slow, sweet
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire {
        api.set(0, 0, Cell { species: Species::Fire, ra: 100, rb: 0, clock: 0 });
        return;
    }
    liquid_flow(cell, &mut api, 4, true);
}

use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

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

pub fn update_steam(cell: Cell, mut api: SandApi) {
    // Steam: condenses on cool surfaces
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

pub fn update_smoke(cell: Cell, mut api: SandApi) {
    // Smoke: rises, dissipates over time
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

pub fn update_helium(cell: Cell, mut api: SandApi) {
    // Helium: very light, rises fast, inert
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

pub fn update_chlorine(cell: Cell, mut api: SandApi) {
    // Chlorine: toxic gas, kills organics, heavier than air
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

pub fn update_oxygen(cell: Cell, mut api: SandApi) {
    // Oxygen: feeds fire
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

pub fn update_hydrogen(cell: Cell, mut api: SandApi) {
    // Hydrogen: very light, extremely flammable
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

pub fn update_plasma(cell: Cell, mut api: SandApi) {
    // Plasma: super-hot ionized gas, destroys nearly everything
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

pub fn update_methane(cell: Cell, mut api: SandApi) {
    // Methane: flammable, greenhouse gas
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Fire, ra: 220, rb: 0, clock: 0 });
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 100, density: 70 });
        return;
    }
    gas_rise(cell, &mut api, 25, true);
}

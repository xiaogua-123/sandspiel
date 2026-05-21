use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

/// Powder that falls slowly and can drift
fn powder_fall(cell: Cell, api: &mut SandApi, flammable: bool, explosive: bool) {
    let dx = api.rand_dir();
    let fluid = api.get_fluid();

    // Explosive in high pressure
    if explosive && fluid.pressure > 100 {
        api.set(0, 0, Cell { species: Species::Fire, ra: 200, rb: 0, clock: 0 });
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 100, density: 50 });
        return;
    }

    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if below.species == Species::Water {
        // Some powders dissolve or sink
        api.set(0, 0, below);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if api.get(dx, 0).species == Species::Empty && api.once_in(4) {
        // Drift sideways
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else {
        api.set(0, 0, cell);
    }

    // Check nearby fire
    if flammable {
        let (sx, sy) = api.rand_vec();
        let nbr = api.get(sx, sy);
        if nbr.species == Species::Fire || nbr.species == Species::Lava {
            api.set(0, 0, Cell { species: Species::Fire, ra: 80, rb: 0, clock: 0 });
        }
    }
}

pub fn update_gunpowder(cell: Cell, mut api: SandApi) {
    let fluid = api.get_fluid();
    // Very explosive - ignites with any fire or spark
    let (sx, sy) = api.rand_vec();
    let nbr = api.get(sx, sy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava || nbr.species == Species::Lightning {
        api.set(0, 0, Cell { species: Species::Fire, ra: 250, rb: 0, clock: 0 });
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 120, density: 80 });
        // Chain reaction
        for _ in 0..3 {
            let (cdx, cdy) = api.rand_vec();
            api.set(cdx, cdy, Cell { species: Species::Fire, ra: 150, rb: 0, clock: 0 });
        }
        return;
    }
    if fluid.pressure > 80 {
        api.set(0, 0, Cell { species: Species::Fire, ra: 200, rb: 0, clock: 0 });
        return;
    }
    powder_fall(cell, &mut api, true, true);
}

pub fn update_flour(cell: Cell, mut api: SandApi) {
    // Flour: flammable when airborne (dust explosion)
    let (sx, sy) = api.rand_vec();
    let nbr = api.get(sx, sy);
    if nbr.species == Species::Fire && api.once_in(5) {
        api.set(0, 0, Cell { species: Species::Fire, ra: 180, rb: 0, clock: 0 });
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 90, density: 40 });
        return;
    }
    // Absorbs water, becomes paste-like
    let below = api.get(0, 1);
    if below.species == Species::Water {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, Cell { species: Species::Mud, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }
    powder_fall(cell, &mut api, true, false);
}

pub fn update_sugar(cell: Cell, mut api: SandApi) {
    // Sugar: dissolves in water, caramelizes near heat
    let (sx, sy) = api.rand_vec();
    let nbr = api.get(sx, sy);
    if nbr.species == Species::Water {
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Fire, ra: 100, rb: 0, clock: 0 });
        return;
    }
    powder_fall(cell, &mut api, true, false);
}

pub fn update_salt(cell: Cell, mut api: SandApi) {
    // Salt: dissolves in water, doesn't burn
    let (sx, sy) = api.rand_vec();
    let nbr = api.get(sx, sy);
    if nbr.species == Species::Water && api.once_in(8) {
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    // Salt melts ice
    if nbr.species == Species::Ice && api.once_in(10) {
        api.set(sx, sy, Cell { species: Species::Water, ra: 100, rb: 0, clock: 0 });
    }
    powder_fall(cell, &mut api, false, false);
}

pub fn update_pepper(cell: Cell, mut api: SandApi) {
    // Pepper: irritant, floats on water
    let below = api.get(0, 1);
    if below.species == Species::Water {
        // Floats
        api.set(0, 0, cell);
    } else {
        powder_fall(cell, &mut api, true, false);
    }
}

pub fn update_ash(cell: Cell, mut api: SandApi) {
    // Ash: very light, drifts, already burnt
    let dx = api.rand_dir();
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if api.get(dx, 0).species == Species::Empty && api.once_in(2) {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else {
        api.set(0, 0, cell);
    }
}

pub fn update_soot(cell: Cell, mut api: SandApi) {
    // Soot: sticky light powder
    let dx = api.rand_dir();
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if api.get(dx, 0).species == Species::Empty && api.once_in(3) {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else {
        // Sticks to surfaces
        api.set(0, 0, cell);
    }
}

pub fn update_charcoal(cell: Cell, mut api: SandApi) {
    // Charcoal: burns slowly
    let (sx, sy) = api.rand_vec();
    let nbr = api.get(sx, sy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        if api.once_in(30) {
            api.set(0, 0, Cell { species: Species::Fire, ra: 60, rb: 0, clock: 0 });
        } else {
            api.set(0, 0, cell);
        }
        return;
    }
    // Burns slowly when ignited
    if cell.rb > 0 {
        api.set(0, 0, Cell { rb: cell.rb.saturating_sub(1), ..cell });
        if cell.rb % 5 == 0 {
            api.set_fluid(Wind { dx: 0, dy: 5, pressure: 5, density: 30 });
        }
        return;
    }
    powder_fall(cell, &mut api, true, false);
}

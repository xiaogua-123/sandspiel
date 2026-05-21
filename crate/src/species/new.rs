use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

/// Snow - light powder, floats, melts when hot
pub fn update_snow(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();

    // Melt if near fire/lava
    let sample = api.get(dx, dy);
    if sample.species == Species::Fire || sample.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Water, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }
    // Melt from heat
    let fluid = api.get_fluid();
    if fluid.pressure > 100 {
        api.set(0, 0, Cell { species: Species::Water, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }

    // Snow falls slowly, drifts
    let dx2 = api.rand_dir();
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if below.species == Species::Water {
        // Snow floats on water, then melts
        if api.once_in(10) {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, 1, cell);
        } else {
            api.set(0, 0, cell);
        }
    } else if api.get(dx2, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx2, 1, cell);
    } else if api.get(dx2, 0).species == Species::Empty && api.once_in(3) {
        // Light drift sideways
        api.set(0, 0, EMPTY_CELL);
        api.set(dx2, 0, cell);
    } else if below.species == Species::Ice && api.once_in(4) {
        // Accumulate on ice
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

/// Sponge - absorbs nearby liquids, expands when wet
pub fn update_sponge(cell: Cell, mut api: SandApi) {
    let mut absorbed = cell.ra; // how much liquid absorbed (0 = dry)
    let (dx, dy) = api.rand_vec();

    // Absorb nearby liquid
    for adx in [-1, 0, 1].iter().cloned() {
        for ady in [-1, 0, 1].iter().cloned() {
            if absorbed >= 200 { break; }
            let nbr = api.get(adx, ady);
            if nbr.species == Species::Water || nbr.species == Species::Oil {
                absorbed = absorbed.saturating_add(30);
                api.set(adx, ady, EMPTY_CELL);
            }
        }
    }

    // Burn if near fire
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        if absorbed > 50 {
            // Wet sponge resists fire
            api.set(0, 0, Cell { ra: absorbed.saturating_sub(5), ..cell });
        } else {
            api.set(0, 0, Cell { species: Species::Fire, ra: 30, rb: 0, clock: 0 });
        }
        return;
    }

    // Drip if too full
    if absorbed > 180 && api.once_in(5) {
        let below = api.get(0, 1);
        if below.species == Species::Empty {
            api.set(0, 1, Cell::new(Species::Water));
            absorbed = absorbed.saturating_sub(30);
        }
    }

    // Squeeze if something falls on it
    if api.get(0, -1).species == Species::Sand || api.get(0, -1).species == Species::Stone {
        if absorbed > 30 && api.once_in(3) {
            let side_dx = api.rand_dir();
            if api.get(side_dx, 0).species == Species::Empty {
                api.set(side_dx, 0, Cell::new(Species::Water));
            }
            absorbed = absorbed.saturating_sub(30);
        }
    }

    // Slow dry
    if absorbed > 0 && api.once_in(20) {
        absorbed = absorbed.saturating_sub(1);
    }

    api.set(0, 0, Cell { ra: absorbed, ..cell });
}

/// Slime - bouncy, sticky, slowly moves down
pub fn update_slime(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;
    let (dx, dy) = api.rand_vec();

    // Burn
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Fire, ra: 20, rb: 0, clock: 0 });
        return;
    }

    // Washed away by water
    if nbr.species == Species::Water && api.once_in(3) {
        api.set(0, 0, nbr);
        api.set(dx, dy, cell);
        return;
    }

    // Stretchy movement
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        // Fall slowly
        if api.once_in(3) {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, 1, cell);
        } else {
            api.set(0, 0, cell);
        }
    } else if below.species == Species::Slime && api.once_in(4) {
        // Stick together, move as blob
        let sdx = api.rand_dir();
        if api.get(sdx, 1).species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(sdx, 1, cell);
        } else {
            api.set(0, 0, cell);
        }
    } else {
        // Spread sideways on surfaces
        let sdx = api.rand_dir();
        if api.get(sdx, 0).species == Species::Empty && api.once_in(3) {
            api.set(0, 0, EMPTY_CELL);
            api.set(sdx, 0, cell);
        } else if api.get(sdx, 1).species == Species::Empty && api.once_in(4) {
            api.set(0, 0, EMPTY_CELL);
            api.set(sdx, 1, cell);
        } else if api.get(dx, dy).species == Species::Acid {
            // Acid dissolves slime
            api.set(0, 0, nbr);
            api.set(dx, dy, EMPTY_CELL);
        } else {
            api.set(0, 0, cell);
        }
    }

    // Bounce: if fell from height, spread a bit (simulated bounce)
    if rb > 0 {
        api.set(0, 0, Cell { rb: rb.saturating_sub(1), ..cell });
        let bdx = api.rand_dir();
        if api.get(bdx, 0).species == Species::Empty {
            api.set(bdx, 0, cell);
            api.set(0, 0, EMPTY_CELL);
        }
    }
}

/// Glass - hard and transparent, melts into lava at high temp
pub fn update_glass(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();

    // Melt when near fire/lava
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        if api.once_in(15) {
            api.set(0, 0, Cell { species: Species::Lava, ra: 80, rb: 0, clock: 0 });
            return;
        }
    }

    // Broken by high pressure
    let fluid = api.get_fluid();
    if fluid.pressure > 200 && api.once_in(5) {
        api.set(0, 0, Cell { species: Species::Sand, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }

    // Acid eats glass slowly
    if nbr.species == Species::Acid && api.once_in(10) {
        api.set(0, 0, Cell { species: Species::Empty, ra: 0, rb: 0, clock: 0 });
        return;
    }

    // Stationary, but falls if nothing under it (like stone)
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if below.species == Species::Water || below.species == Species::Oil || below.species == Species::Gas {
        api.set(0, 0, below);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

/// Coral - grows in water on solid surfaces
pub fn update_coral(cell: Cell, mut api: SandApi) {
    let ra = cell.ra;
    let (dx, dy) = api.rand_vec();

    // Die from fire/lava
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Empty, ra: 0, rb: 0, clock: 0 });
        return;
    }

    // Die out of water
    let in_water = api.get(0, 1).species == Species::Water
        || api.get(1, 0).species == Species::Water
        || api.get(-1, 0).species == Species::Water
        || api.get(0, -1).species == Species::Water;
    if !in_water && api.once_in(20) {
        api.set(0, 0, Cell { species: Species::Stone, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }

    // Grow upward
    if ra > 30 && api.once_in(8) {
        let above = api.get(0, -1);
        if above.species == Species::Water {
            let new_ra = (ra as i32 + api.rand_int(10)).saturating_sub(5) as u8;
            api.set(0, -1, Cell { species: Species::Coral, ra: new_ra, rb: 0, clock: 0 });
        }
    }

    // Branch sideways
    if ra > 60 && api.once_in(12) {
        let bdx = if api.once_in(2) { 1 } else { -1 };
        let side = api.get(bdx, 0);
        if side.species == Species::Water {
            let new_ra = (ra as i32 + api.rand_int(10)).saturating_sub(5) as u8;
            api.set(bdx, 0, Cell { species: Species::Coral, ra: new_ra.clamp(10, 100), rb: 0, clock: 0 });
        }
    }

    // Grow taller
    if ra < 100 && api.once_in(6) {
        api.set(0, 0, Cell { ra: ra + 1, ..cell });
    }
}

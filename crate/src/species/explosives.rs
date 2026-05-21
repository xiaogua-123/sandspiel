use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

fn explode(api: &mut SandApi, power: u8, spread_fire: bool) {
    api.set(0, 0, EMPTY_CELL);
    let pressure = (power as u8).saturating_mul(2);
    let density = power;
    api.set_fluid(Wind { dx: 0, dy: 0, pressure, density });

    for dx in -2..=2 {
        for dy in -2..=2 {
            if dx == 0 && dy == 0 { continue; }
            let cell = api.get(dx, dy);
            if cell.species != Species::Wall && cell.species != Species::Empty
                && cell.species != Species::Void && cell.species != Species::Shield {
                if spread_fire && api.once_in(2) {
                    api.set(dx, dy, Cell { species: Species::Fire, ra: 150, rb: 0, clock: 0 });
                }
            }
            // Shockwave: push things away
            let fluid = api.get_fluid();
            if fluid.pressure < pressure {
                api.set_fluid(Wind {
                    dx: (dx * 30) as u8,
                    dy: (dy * 30) as u8,
                    pressure: pressure / 3,
                    density: density / 2,
                });
            }
        }
    }
}

fn falling_explosive(cell: Cell, api: &mut SandApi, trigger: bool, power: u8) {
    if trigger {
        explode(api, power, true);
        return;
    }
    // Fall like sand
    let dx = api.rand_dir_2();
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

pub fn update_tnt(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    let triggered = nbr.species == Species::Fire || nbr.species == Species::Lava
        || nbr.species == Species::Lightning || nbr.species == Species::PlasmaGas;
    // TNT also detonates from impact (high pressure)
    let fluid = api.get_fluid();
    let impact = fluid.pressure > 100;
    falling_explosive(cell, &mut api, triggered || impact, 120);
}

pub fn update_bomb(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    let triggered = nbr.species == Species::Fire || nbr.species == Species::Lava
        || nbr.species == Species::Lightning || nbr.species == Species::TNT;
    if triggered {
        explode(&mut api, 150, true);
        return;
    }
    // Stationary until triggered
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

pub fn update_nitro(cell: Cell, mut api: SandApi) {
    // Nitroglycerin: extremely unstable!
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    let fluid = api.get_fluid();
    // Detonates from any shock, heat, or even slight pressure
    if nbr.species == Species::Fire || nbr.species == Species::Lava
        || fluid.pressure > 40 || nbr.species != Species::Empty {
        explode(&mut api, 180, true);
        return;
    }
    // Very sensitive to movement
    let below = api.get(0, 1);
    if below.species == Species::Empty && api.once_in(2) {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

pub fn update_plutonium(cell: Cell, mut api: SandApi) {
    // Plutonium: radioactive, heats up surroundings
    api.set_fluid(Wind { dx: 0, dy: 0, pressure: 5, density: 80 });
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species != Species::Empty && nbr.species != Species::Wall
        && nbr.species != Species::Plutonium {
        if api.once_in(30) {
            api.set(dx, dy, Cell { species: Species::Fire, ra: 50, rb: 0, clock: 0 });
        }
    }
    // Very heavy, stationary
    heavy_fall_plutonium(cell, &mut api);
}

fn heavy_fall_plutonium(cell: Cell, api: &mut SandApi) {
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if matches!(below.species, Species::Water | Species::Oil | Species::Gas | Species::Acid) {
        api.set(0, 0, below);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

pub fn update_uranium(cell: Cell, mut api: SandApi) {
    // Uranium: similar to plutonium but slightly less active
    api.set_fluid(Wind { dx: 0, dy: 0, pressure: 3, density: 60 });
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species != Species::Empty && nbr.species != Species::Wall
        && nbr.species != Species::Uranium && nbr.species != Species::Plutonium {
        if api.once_in(50) {
            api.set(dx, dy, Cell { species: Species::Fire, ra: 30, rb: 0, clock: 0 });
        }
    }
    heavy_fall_plutonium(cell, &mut api);
}

pub fn update_c4(cell: Cell, mut api: SandApi) {
    // C4: stable plastic explosive, needs fire to detonate
    let edx = api.rand_dir();
    let edy = api.rand_dir();
    let nbr = api.get(edx, edy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        explode(&mut api, 160, true);
        return;
    }
    // Falls like sand, stable
    falling_explosive(cell, &mut api, false, 0);
}

pub fn update_thermite(cell: Cell, mut api: SandApi) {
    // Thermite: burns incredibly hot, melts through things
    let rb = cell.rb;
    if rb > 0 {
        // Already ignited - burns through everything
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 0, density: 200 });
        let below = api.get(0, 1);
        if below.species != Species::Wall && below.species != Species::Void {
            api.set(0, 1, Cell { species: Species::Lava, ra: 200, rb: 0, clock: 0 });
        }
        api.set(0, 0, Cell { rb: rb - 1, ..cell });
        if rb <= 1 {
            api.set(0, 0, EMPTY_CELL);
        }
        return;
    }
    // Check for ignition
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { rb: 40, ..cell });
        return;
    }
    falling_explosive(cell, &mut api, false, 0);
}

pub fn update_napalm(cell: Cell, mut api: SandApi) {
    // Napalm: sticky burning gel
    let rb = cell.rb;
    if rb > 0 {
        // Burning - spreads fire everywhere
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 0, density: 180 });
        let (dx, dy) = api.rand_vec();
        let nbr = api.get(dx, dy);
        if nbr.species == Species::Empty && api.once_in(2) {
            api.set(dx, dy, Cell { species: Species::Fire, ra: 150, rb: 0, clock: 0 });
        }
        // Sticky - doesn't fall
        api.set(0, 0, Cell { rb: rb.saturating_sub(1), ..cell });
        if rb <= 1 {
            api.set(0, 0, Cell { species: Species::Fire, ra: 50, rb: 0, clock: 0 });
        }
        return;
    }
    // Check for ignition
    let (dx, dy) = api.rand_vec();
    if api.get(dx, dy).species == Species::Fire {
        api.set(0, 0, Cell { rb: 30, ..cell });
        return;
    }
    falling_explosive(cell, &mut api, false, 0);
}

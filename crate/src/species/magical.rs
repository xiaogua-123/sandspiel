use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

pub fn update_portal(cell: Cell, mut api: SandApi) {
    // Portal: teleports nearby particles to another random location
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species != Species::Empty && nbr.species != Species::Wall
        && nbr.species != Species::Portal && nbr.species != Species::Void {
        // Teleport to random location
        let tx = (api.rand_int(api.universe.width) as i32) - api.x;
        let ty = (api.rand_int(api.universe.height) as i32) - api.y;
        let tx = tx.clamp(-2, 2);
        let ty = ty.clamp(-2, 2);
        if api.get(tx, ty).species == Species::Empty {
            api.set(tx, ty, nbr);
            api.set(dx, dy, EMPTY_CELL);
        }
    }
}

pub fn update_teleporter(cell: Cell, mut api: SandApi) {
    // Teleporter: when touched by anything, swaps positions
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species != Species::Empty && nbr.species != Species::Wall
        && nbr.species != Species::Teleporter {
        let (tdx, tdy) = api.rand_vec_8();
        if api.get(tdx, tdy).species == Species::Empty {
            api.set(tdx, tdy, nbr);
            api.set(dx, dy, EMPTY_CELL);
        }
    }
}

pub fn update_antigravity(cell: Cell, mut api: SandApi) {
    // Antigravity: makes nearby things float upward
    api.set_fluid(Wind { dx: 0, dy: 200, pressure: 0, density: 0 });
    // Push nearby cells upward
    let adx = api.rand_dir();
    let nbr = api.get(adx, -1);
    if nbr.species != Species::Empty && nbr.species != Species::Wall
        && nbr.species != Species::Antigravity {
        api.set(0, -1, nbr);
        let avx = api.rand_dir();
        api.set(avx, -1, EMPTY_CELL);
    }
}

pub fn update_magnet(cell: Cell, mut api: SandApi) {
    // Magnet: attracts metal elements
    for dx in -2..=2 {
        for dy in -2..=2 {
            if dx == 0 && dy == 0 { continue; }
            let nbr = api.get(dx, dy);
            if matches!(nbr.species,
                Species::Iron | Species::Copper | Species::Steel
                | Species::Zinc | Species::Tin | Species::Wire) {
                // Pull toward magnet
                let pull_x = if dx > 0 { dx - 1 } else if dx < 0 { dx + 1 } else { 0 };
                let pull_y = if dy > 0 { dy - 1 } else if dy < 0 { dy + 1 } else { 0 };
                if api.get(pull_x, pull_y).species == Species::Empty {
                    api.set(pull_x, pull_y, nbr);
                    api.set(dx, dy, EMPTY_CELL);
                }
            }
        }
    }
}

pub fn update_lightning(cell: Cell, mut api: SandApi) {
    // Lightning: strikes downward, powerful electric discharge
    api.set_fluid(Wind { dx: 0, dy: 100, pressure: 200, density: 200 });
    let below = api.get(0, 1);
    if below.species != Species::Empty && below.species != Species::Wall
        && below.species != Species::Void && below.species != Species::Shield {
        // Electrocute
        api.set(0, 1, Cell { species: Species::Fire, ra: 200, rb: 0, clock: 0 });
        api.set(0, 1, EMPTY_CELL);
        api.set(0, 1, Cell { species: Species::PlasmaGas, ra: 100, rb: 0, clock: 0 });
    }
    // Short lived
    if api.once_in(8) {
        api.set(0, 0, EMPTY_CELL);
    } else if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    }
}

pub fn update_void(cell: Cell, mut api: SandApi) {
    // Void: destroys everything it touches
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species != Species::Wall && nbr.species != Species::Void
        && nbr.species != Species::Shield {
        api.set(dx, dy, EMPTY_CELL);
    }
    // Slowly dissipates
    if api.once_in(200) {
        api.set(0, 0, EMPTY_CELL);
    }
}

pub fn update_chaos(cell: Cell, mut api: SandApi) {
    // Chaos: randomly transforms nearby elements
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species != Species::Empty && nbr.species != Species::Wall
        && nbr.species != Species::Chaos && nbr.species != Species::Void
        && nbr.species != Species::Shield && api.once_in(3) {
        // Random transformation
        let random_species = match api.rand_int(30) {
            0 => Species::Sand, 1 => Species::Water, 2 => Species::Gas,
            3 => Species::Fire, 4 => Species::Wood, 5 => Species::Lava,
            6 => Species::Ice, 7 => Species::Snow, 8 => Species::Plant,
            9 => Species::Acid, 10 => Species::Stone, 11 => Species::Dust,
            12 => Species::Mite, 13 => Species::Oil, 14 => Species::Rocket,
            15 => Species::Fungus, 16 => Species::Seed, 17 => Species::Slime,
            18 => Species::Glass, 19 => Species::Iron, 20 => Species::Gold,
            21 => Species::Mud, 22 => Species::Honey, 23 => Species::Mushroom,
            24 => Species::Gunpowder, 25 => Species::Diamond, 26 => Species::Obsidian,
            27 => Species::Alcohol, 28 => Species::Steam, 29 => Species::PlasmaGas,
            _ => Species::Empty,
        };
        api.set(dx, dy, Cell { species: random_species, ra: 100, rb: 0, clock: 0 });
    }
    // Slowly dissipates
    if api.once_in(300) {
        api.set(0, 0, EMPTY_CELL);
    }
}

pub fn update_energy(cell: Cell, mut api: SandApi) {
    // Energy: pure energy, spreads and powers things
    api.set_fluid(Wind { dx: 0, dy: 0, pressure: 10, density: 200 });
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Empty && api.once_in(5) {
        api.set(dx, dy, Cell { species: Species::Energy, ra: cell.ra.saturating_sub(10), rb: 0, clock: 0 });
    }
    if nbr.species == Species::Battery || nbr.species == Species::SolarCell {
        api.set(dx, dy, Cell { ra: 200, ..nbr });
    }
    // Dissipates
    let ra = cell.ra;
    if ra < 10 {
        api.set(0, 0, EMPTY_CELL);
    } else {
        api.set(0, 0, Cell { ra: ra.saturating_sub(2), ..cell });
    }
}

pub fn update_shield(cell: Cell, mut api: SandApi) {
    // Shield: impenetrable barrier that weakens over time
    let rb = cell.rb.saturating_add(1);
    if rb > 100 {
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    // Protects against void, fire, lava, acid
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if matches!(nbr.species, Species::Void | Species::Fire | Species::Lava
        | Species::Acid | Species::PlasmaGas | Species::Lightning | Species::Chaos) {
        // Block and weaken
        let new_rb = rb + 5;
        api.set(0, 0, Cell { rb: new_rb, ..cell });
    } else {
        api.set(0, 0, Cell { rb, ..cell });
    }
}

pub fn update_mirror(cell: Cell, mut api: SandApi) {
    // Mirror: reflects lasers, stationary
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Laser {
        // Reflect laser back
        api.set(-dx, -dy, Cell::new(Species::Laser));
        api.set(dx, dy, EMPTY_CELL);
    }
}

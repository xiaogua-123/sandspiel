use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

fn burnable_solid(cell: Cell, api: &mut SandApi, burn_time: u8) {
    let rb = cell.rb;
    let (dx, dy) = api.rand_vec();
    let nbr_species = api.get(dx, dy).species;

    if rb == 0 && (nbr_species == Species::Fire || nbr_species == Species::Lava) {
        api.set(0, 0, Cell { species: cell.species, ra: cell.ra, rb: burn_time, clock: 0 });
        return;
    }

    if rb > 1 {
        api.set(0, 0, Cell { species: cell.species, ra: cell.ra, rb: rb - 1, clock: 0 });
        if rb % 4 == 0 && nbr_species == Species::Empty {
            let ra = 20 + api.rand_int(30) as u8;
            api.set(dx, dy, Cell { species: Species::Fire, ra, rb: 0, clock: 0 });
        }
        if nbr_species == Species::Water {
            api.set(0, 0, Cell { species: cell.species, ra: 50, rb: 0, clock: 0 });
        }
    } else if rb == 1 {
        api.set(0, 0, Cell { species: Species::Ash, ra: cell.ra, rb: 0, clock: 0 });
    }
}

pub fn update_leaf(cell: Cell, mut api: SandApi) {
    // Leaf: light, drifts, burns easily
    let rb = cell.rb;
    if rb > 0 {
        burnable_solid(cell, &mut api, 15);
        return;
    }
    let dx = api.rand_dir();
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if api.get(dx, 0).species == Species::Empty && api.once_in(5) {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else {
        burnable_solid(cell, &mut api, 15);
    }
}

pub fn update_flower(cell: Cell, mut api: SandApi) {
    // Flower: pretty, attracts bees and butterflies
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Bee || nbr.species == Species::Butterfly {
        // Pollinated - spread seeds
        if api.once_in(30) {
            let (sdx, sdy) = api.rand_vec();
            if api.get(sdx, sdy).species == Species::Empty {
                api.set(sdx, sdy, Cell::new(Species::Seed));
            }
        }
    }
    burnable_solid(cell, &mut api, 10);
}

pub fn update_grass(cell: Cell, mut api: SandApi) {
    // Grass: spreads sideways, needs water
    let rb = cell.rb;
    let ra = cell.ra;
    burnable_solid(cell, &mut api, 12);

    if rb == 0 {
        // Spread sideways on soil/sand
        let sdx = api.rand_dir();
        let side = api.get(sdx, 0);
        if side.species == Species::Soil || side.species == Species::Sand {
            if api.get(sdx, -1).species == Species::Empty && api.once_in(40) {
                api.set(sdx, -1, Cell { ra: ra.saturating_sub(5), ..cell });
            }
        }
        // Grow upward
        if api.get(0, -1).species == Species::Empty && api.once_in(60) && ra > 30 {
            api.set(0, -1, Cell { ra: ra.saturating_sub(3), ..cell });
        }
    }
}

pub fn update_vine(cell: Cell, mut api: SandApi) {
    // Vine: climbs upward on solid surfaces
    let rb = cell.rb;
    let ra = cell.ra;
    burnable_solid(cell, &mut api, 20);

    if rb == 0 {
        // Climb upward along walls/wood
        for dy in [-1, 1].iter() {
            for dx in [-1, 0, 1].iter() {
                let nbr = api.get(*dx, *dy);
                if nbr.species == Species::Wall || nbr.species == Species::Wood
                    || nbr.species == Species::Vine {
                    // Grow in opposite direction
                    if api.get(-dx, -dy).species == Species::Empty && api.once_in(20) {
                        api.set(-dx, -dy, Cell { ra: ra.saturating_sub(2), ..cell });
                    }
                }
            }
        }
    }
}

pub fn update_moss(cell: Cell, mut api: SandApi) {
    // Moss: stationary, grows on surfaces, needs moisture
    let ra = cell.ra;
    let rb = cell.rb;
    burnable_solid(cell, &mut api, 20);

    if rb == 0 {
        // Spread on surface
        let sdx = api.rand_dir();
        if api.get(sdx, 0).species == Species::Empty && api.once_in(25) && ra > 30 {
            api.set(sdx, 0, Cell { ra: ra.saturating_sub(5), ..cell });
        }
        // Needs water nearby to thrive
        let (mdx, mdy) = api.rand_vec();
        if api.get(mdx, mdy).species == Species::Water && ra < 200 && api.once_in(3) {
            api.set(0, 0, Cell { ra: ra + 1, ..cell });
        }
        // Wither without water
        if ra > 20 && api.once_in(50) {
            api.set(0, 0, Cell { ra: ra - 1, ..cell });
        }
    }
}

pub fn update_mushroom(cell: Cell, mut api: SandApi) {
    // Mushroom: grows on wood/fungus, spreads spores
    let ra = cell.ra;
    let rb = cell.rb;
    burnable_solid(cell, &mut api, 15);

    if rb == 0 {
        // Spread on wood or fungus
        let (mdx, mdy) = api.rand_vec();
        let nbr = api.get(mdx, mdy);
        if (nbr.species == Species::Wood || nbr.species == Species::Fungus
            || nbr.species == Species::Soil) && api.once_in(15) && ra > 40 {
            let (sdx, sdy) = api.rand_vec_8();
            if api.get(sdx, sdy).species == Species::Empty {
                api.set(sdx, sdy, Cell { ra: ra - 5, ..cell });
            }
        }
    }
}

pub fn update_bark(cell: Cell, mut api: SandApi) {
    // Bark: tough outer layer, fire resistant
    let rb = cell.rb;
    let (dx, dy) = api.rand_vec();
    let nbr_species = api.get(dx, dy).species;

    if rb == 0 && (nbr_species == Species::Fire || nbr_species == Species::Lava) {
        api.set(0, 0, Cell { species: Species::Bark, ra: cell.ra, rb: 40, clock: 0 });
        return;
    }
    if rb > 1 {
        api.set(0, 0, Cell { ra: cell.ra, rb: rb - 1, ..cell });
        if nbr_species == Species::Water {
            api.set(0, 0, Cell { ra: 50, rb: 0, ..cell });
        }
    } else if rb == 1 {
        api.set(0, 0, Cell { species: Species::Charcoal, ra: cell.ra, rb: 0, clock: 0 });
    }
}

pub fn update_root(cell: Cell, mut api: SandApi) {
    // Root: grows downward into soil, absorbs water
    let ra = cell.ra;
    let rb = cell.rb;
    burnable_solid(cell, &mut api, 25);

    if rb == 0 {
        let below = api.get(0, 1);
        if (below.species == Species::Soil || below.species == Species::Sand
            || below.species == Species::Clay) && api.once_in(15) && ra > 20 {
            api.set(0, 1, Cell { ra: ra.saturating_sub(3), ..cell });
        }
        // Absorb water
        let rdx = api.rand_dir();
        let rdy = api.rand_dir();
        let nbr = api.get(rdx, rdy);
        if nbr.species == Species::Water && api.once_in(5) {
            api.set(0, 0, Cell { ra: ra + 5, ..cell });
        }
    }
}

pub fn update_fruit(cell: Cell, mut api: SandApi) {
    // Fruit: falls, edible, rots into seeds
    let rb = cell.rb;
    if rb > 0 {
        burnable_solid(cell, &mut api, 15);
        return;
    }
    // Falls
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else {
        // Slowly ripen/rot
        let ra = cell.ra.saturating_sub(1);
        if ra < 20 {
            // Rots into seeds
            api.set(0, 0, Cell::new(Species::Seed));
            let sdx = api.rand_dir();
            if api.get(sdx, 0).species == Species::Empty {
                api.set(sdx, 0, Cell::new(Species::Seed));
            }
        } else {
            burnable_solid(Cell { ra, ..cell }, &mut api, 15);
        }
    }
}

pub fn update_thorn(cell: Cell, mut api: SandApi) {
    // Thorn: sharp, damages creatures, stationary
    let rb = cell.rb;
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if matches!(nbr.species, Species::Mite | Species::Ant | Species::Spider
        | Species::Bee | Species::Butterfly | Species::Bird | Species::Snake | Species::Worm) {
        api.set(dx, dy, EMPTY_CELL);
    }
    burnable_solid(cell, &mut api, 10);
}

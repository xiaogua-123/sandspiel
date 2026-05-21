use crate::{Cell, SandApi, EMPTY_CELL};
use super::Species;

fn food_fall(cell: Cell, api: &mut SandApi) {
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if below.species == Species::Water {
        api.set(0, 0, below);
        api.set(0, 1, cell);
    } else {
        let dx = api.rand_dir_2();
        if api.get(dx, 1).species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, 1, cell);
        } else {
            api.set(0, 0, cell);
        }
    }
}

fn food_burn(cell: Cell, api: &mut SandApi, burn_time: u8) {
    let rb = cell.rb;
    let (dx, dy) = api.rand_vec();
    let nbr_species = api.get(dx, dy).species;
    if rb == 0 && (nbr_species == Species::Fire || nbr_species == Species::Lava) {
        api.set(0, 0, Cell { species: cell.species, ra: cell.ra, rb: burn_time, clock: 0 });
    } else if rb > 1 {
        api.set(0, 0, Cell { ra: cell.ra, rb: rb - 1, ..cell });
        if nbr_species == Species::Water {
            api.set(0, 0, Cell { ra: 50, rb: 0, ..cell });
        }
    } else if rb == 1 {
        api.set(0, 0, Cell { species: Species::Ash, ra: cell.ra, rb: 0, clock: 0 });
    }
}

pub fn update_bread(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Fire, ra: 80, rb: 0, clock: 0 });
        return;
    }
    if nbr.species == Species::Water && api.once_in(10) {
        api.set(0, 0, Cell { ra: cell.ra.saturating_sub(10), ..cell });
    }
    food_fall(cell, &mut api);
}

pub fn update_cheese(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Oil, ra: 80, rb: 0, clock: 0 });
        return;
    }
    if (nbr.species == Species::Mite || nbr.species == Species::Ant) && api.once_in(20) {
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    food_fall(cell, &mut api);
}

pub fn update_meat(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        let ra = cell.ra;
        if ra > 100 {
            api.set(0, 0, Cell { species: Species::Fire, ra: 100, rb: 0, clock: 0 });
        } else {
            api.set(0, 0, Cell { ra: ra + 5, ..cell });
        }
        return;
    }
    if (nbr.species == Species::Spider || nbr.species == Species::Snake
        || nbr.species == Species::Bird) && api.once_in(15) {
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    if api.once_in(500) {
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    food_fall(cell, &mut api);
}

pub fn update_egg(cell: Cell, mut api: SandApi) {
    let ra = cell.ra;
    let fluid = api.get_fluid();
    if fluid.pressure > 60 {
        api.set(0, 0, EMPTY_CELL);
        let edx = api.rand_dir();
        api.set(edx, 0, Cell { species: Species::Oil, ra: 100, rb: 0, clock: 0 });
        return;
    }
    if ra > 150 {
        api.set(0, 0, Cell::new(Species::Bird));
        return;
    }
    if api.once_in(100) {
        api.set(0, 0, Cell { ra: ra + 1, ..cell });
    }
    food_fall(cell, &mut api);
}

pub fn update_rice(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Water && api.once_in(5) {
        api.set(dx, dy, EMPTY_CELL);
        api.set(0, 0, Cell { ra: cell.ra + 10, ..cell });
        return;
    }
    if nbr.species == Species::Fire {
        api.set(0, 0, Cell { species: Species::Fire, ra: 30, rb: 0, clock: 0 });
    }
    let below = api.get(0, 1);
    let dx2 = api.rand_dir();
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx2, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx2, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

pub fn update_wheat(cell: Cell, mut api: SandApi) {
    let ra = cell.ra;
    let rb = cell.rb;
    if rb > 0 {
        food_burn(cell, &mut api, 15);
        return;
    }
    let below = api.get(0, 1);
    if below.species == Species::Soil && api.once_in(15) && ra < 200 {
        api.set(0, 0, Cell { ra: ra + 5, ..cell });
    }
    if ra > 100 && api.get(0, -1).species == Species::Empty && api.once_in(25) {
        api.set(0, -1, Cell { ra: ra - 30, ..cell });
    }
    food_burn(cell, &mut api, 10);
}

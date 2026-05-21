use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

pub fn update_gas(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);

    if cell.rb == 0 {
        api.set(0, 0, Cell { rb: 5, ..cell });
    }

    if nbr.species == Species::Empty {
        if cell.rb < 3 {
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, dy, cell);
        } else {
            api.set(0, 0, Cell { rb: 1, ..cell });
            api.set(dx, dy, Cell { rb: cell.rb - 1, ..cell });
        }
    } else if (dx != 0 || dy != 0) && nbr.species == Species::Gas && nbr.rb < 4 {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, dy, Cell { rb: nbr.rb + cell.rb, ..cell });
    }
}

pub fn update_fire(cell: Cell, mut api: SandApi) {
    let ra = cell.ra;
    let mut degraded = cell.clone();
    degraded.ra = ra.saturating_sub((2 + api.rand_dir()) as u8);

    let (dx, dy) = api.rand_vec();

    api.set_fluid(Wind {
        dx: 0,
        dy: 150,
        pressure: 1,
        density: 120,
    });
    if api.get(dx, dy).species == Species::Gas || api.get(dx, dy).species == Species::Dust {
        api.set(dx, dy, Cell { species: Species::Fire, ra: (150 + (dx + dy) * 10) as u8, rb: 0, clock: 0 });
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 80, density: 40 });
    }
    if ra < 5 || api.get(dx, dy).species == Species::Water {
        api.set(0, 0, EMPTY_CELL);
    } else if api.get(dx, dy).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, dy, degraded);
    } else {
        api.set(0, 0, degraded);
    }
}

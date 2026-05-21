use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

pub fn update_water(cell: Cell, mut api: SandApi) {
    let mut dx = api.rand_dir();
    let below = api.get(0, 1);
    let dx1 = api.get(dx, 1);

    if below.species == Species::Empty || below.species == Species::Oil {
        api.set(0, 0, below);
        let mut ra = cell.ra;
        if api.once_in(20) {
            ra = 100 + api.rand_int(50) as u8;
        }
        api.set(0, 1, Cell { ra, ..cell });
        return;
    } else if dx1.species == Species::Empty || dx1.species == Species::Oil {
        api.set(0, 0, dx1);
        api.set(dx, 1, cell);
        return;
    } else if api.get(-dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 1, cell);
        return;
    }
    let left = cell.ra % 2 == 0;
    dx = if left { 1 } else { -1 };
    let dx0 = api.get(dx, 0);
    let dxd = api.get(dx * 2, 0);

    if dx0.species == Species::Empty && dxd.species == Species::Empty {
        api.set(0, 0, dxd);
        api.set(2 * dx, 0, Cell { rb: 6, ..cell });
        let (rdx, rdy) = api.rand_vec_8();
        let nbr = api.get(rdx, rdy);
        if nbr.species == Species::Water {
            if nbr.ra % 2 != cell.ra % 2 {
                api.set(rdx, rdy, Cell { ra: cell.ra, ..cell })
            }
        }
    } else if dx0.species == Species::Empty || dx0.species == Species::Oil {
        api.set(0, 0, dx0);
        api.set(dx, 0, Cell { rb: 3, ..cell });
        let (rdx, rdy) = api.rand_vec_8();
        let nbr = api.get(rdx, rdy);
        if nbr.species == Species::Water {
            if nbr.ra % 2 != cell.ra % 2 {
                api.set(rdx, rdy, Cell { ra: cell.ra, ..cell })
            }
        }
    } else if cell.rb == 0 {
        if api.get(-dx, 0).species == Species::Empty {
            api.set(0, 0, Cell { ra: ((cell.ra as i32) + dx) as u8, ..cell });
        }
    } else {
        api.set(0, 0, Cell { rb: cell.rb - 1, ..cell });
    }
}

pub fn update_oil(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;
    let (dx, dy) = api.rand_vec();

    let mut new_cell = cell;
    let nbr = api.get(dx, dy);
    if rb == 0 && nbr.species == Species::Fire
        || nbr.species == Species::Lava
        || (nbr.species == Species::Oil && nbr.rb > 1 && nbr.rb < 20)
    {
        new_cell = Cell {
            species: Species::Oil,
            ra: cell.ra,
            rb: 50,
            clock: 0,
        };
    }

    if rb > 1 {
        new_cell = Cell {
            species: Species::Oil,
            ra: cell.ra,
            rb: rb - 1,
            clock: 0,
        };
        api.set_fluid(Wind {
            dx: 0,
            dy: 10,
            pressure: 10,
            density: 180,
        });
        if rb % 4 != 0 && nbr.species == Species::Empty && nbr.species != Species::Water {
            let ra = 20 + api.rand_int(30) as u8;
            api.set(dx, dy, Cell { species: Species::Fire, ra, rb: 0, clock: 0 });
        }
        if nbr.species == Species::Water {
            new_cell = Cell {
                species: Species::Oil,
                ra: 50,
                rb: 0,
                clock: 0,
            };
        }
    } else if rb == 1 {
        api.set(0, 0, Cell { species: Species::Empty, ra: cell.ra, rb: 90, clock: 0 });
        return;
    }

    if api.get(0, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, new_cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, new_cell);
    } else if api.get(-dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 1, new_cell);
    } else if api.get(dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, new_cell);
    } else if api.get(-dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 0, new_cell);
    } else {
        api.set(0, 0, new_cell);
    }
}

pub fn update_lava(cell: Cell, mut api: SandApi) {
    api.set_fluid(Wind {
        dx: 0,
        dy: 10,
        pressure: 0,
        density: 60,
    });
    let (dx, dy) = api.rand_vec();

    if api.get(dx, dy).species == Species::Gas || api.get(dx, dy).species == Species::Dust {
        api.set(dx, dy, Cell {
            species: Species::Fire,
            ra: (150 + (dx + dy) * 10) as u8,
            rb: 0,
            clock: 0,
        });
    }
    let sample = api.get(dx, dy);
    if sample.species == Species::Water {
        api.set(0, 0, Cell { species: Species::Stone, ra: (150 + (dx + dy) * 10) as u8, rb: 0, clock: 0 });
        api.set(dx, dy, EMPTY_CELL);
    } else if api.get(0, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if api.get(dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else {
        api.set(0, 0, cell);
    }
}

pub fn update_acid(cell: Cell, mut api: SandApi) {
    let dx = api.rand_dir();
    let ra = cell.ra;
    let mut degraded = cell.clone();
    degraded.ra = ra.saturating_sub(60);
    if degraded.ra < 80 {
        degraded = EMPTY_CELL;
    }
    if api.get(0, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else if api.get(-dx, 0).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(-dx, 0, cell);
    } else {
        if api.get(0, 1).species != Species::Wall && api.get(0, 1).species != Species::Acid {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, 1, degraded);
        } else if api.get(dx, 0).species != Species::Wall && api.get(dx, 0).species != Species::Acid {
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, 0, degraded);
        } else if api.get(-dx, 0).species != Species::Wall && api.get(-dx, 0).species != Species::Acid {
            api.set(0, 0, EMPTY_CELL);
            api.set(-dx, 0, degraded);
        } else if api.get(0, -1).species != Species::Wall
            && api.get(0, -1).species != Species::Acid
            && api.get(0, -1).species != Species::Empty
        {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, -1, degraded);
        } else {
            api.set(0, 0, cell);
        }
    }
}

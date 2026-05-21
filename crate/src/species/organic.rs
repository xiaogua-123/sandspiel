use crate::utils::*;
use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

pub fn update_wood(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;
    let (dx, dy) = api.rand_vec();
    let nbr_species = api.get(dx, dy).species;

    if rb == 0 && nbr_species == Species::Fire || nbr_species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Wood, ra: cell.ra, rb: 90, clock: 0 });
    }

    if rb > 1 {
        api.set(0, 0, Cell { species: Species::Wood, ra: cell.ra, rb: rb - 1, clock: 0 });
        if rb % 4 == 0 && nbr_species == Species::Empty {
            let ra = 30 + api.rand_int(60) as u8;
            api.set(dx, dy, Cell { species: Species::Fire, ra, rb: 0, clock: 0 })
        }
        if nbr_species == Species::Water {
            api.set(0, 0, Cell { species: Species::Wood, ra: 50, rb: 0, clock: 0 });
            api.set_fluid(Wind { dx: 0, dy: 0, pressure: 0, density: 220 });
        }
    } else if rb == 1 {
        api.set(0, 0, Cell { species: Species::Empty, ra: cell.ra, rb: 90, clock: 0 });
    }
}

pub fn update_plant(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;
    let mut i = api.rand_int(100);
    let (dx, dy) = api.rand_vec();
    let nbr_species = api.get(dx, dy).species;

    if rb == 0 && nbr_species == Species::Fire || nbr_species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Plant, ra: cell.ra, rb: 20, clock: 0 });
    }
    if nbr_species == Species::Wood {
        let (sdx, sdy) = api.rand_vec();
        let drift = (i % 15) - 7;
        let newra = (cell.ra as i32 + drift) as u8;
        if api.get(sdx, sdy).species == Species::Empty {
            api.set(sdx, sdy, Cell { species: Species::Plant, ra: newra, rb: 0, clock: 0 });
        }
    }
    if api.rand_int(100) > 80
        && (nbr_species == Species::Water
            || nbr_species == Species::Fungus
                && (api.get(-dx, dy).species == Species::Empty
                    || api.get(-dx, dy).species == Species::Water
                    || api.get(-dx, dy).species == Species::Fungus))
    {
        i = api.rand_int(100);
        let drift = (i % 15) - 7;
        let newra = (cell.ra as i32 + drift) as u8;
        api.set(dx, dy, Cell { ra: newra, rb: 0, ..cell });
        api.set(-dx, dy, EMPTY_CELL);
    }

    if rb > 1 {
        api.set(0, 0, Cell { ra: cell.ra, rb: rb - 1, ..cell });
        if nbr_species == Species::Empty {
            let ra = 20 + api.rand_int(30) as u8;
            api.set(dx, dy, Cell { species: Species::Fire, ra, rb: 0, clock: 0 });
        }
        if nbr_species == Species::Water {
            api.set(0, 0, Cell { ra: 50, rb: 0, ..cell })
        }
    } else if rb == 1 {
        api.set(0, 0, EMPTY_CELL);
    }

    let ra = cell.ra;
    if ra > 50
        && api.get(1, 1).species != Species::Plant
        && api.get(-1, 1).species != Species::Plant
    {
        if api.get(0, 1).species == Species::Empty {
            let plant_i = (js_sys::Math::random() * js_sys::Math::random() * 100.) as i32;
            let dec = api.rand_int(30) - 20;
            if (plant_i + ra as i32) > 165 {
                api.set(0, 1, Cell { ra: (ra as i32 + dec) as u8, ..cell });
            }
        } else {
            api.set(0, 0, Cell { ra: (ra - 1) as u8, ..cell });
        }
    }
}

pub fn update_seed(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;
    let ra = cell.ra;
    let (dx, dy) = api.rand_vec();
    let nbr_species = api.get(dx, dy).species;

    if nbr_species == Species::Fire || nbr_species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Fire, ra: 5, rb: 0, clock: 0 });
        return;
    }

    if rb == 0 {
        let dxf = api.rand_dir();
        let nbr_species_below = api.get(dxf, 1).species;
        if nbr_species_below == Species::Sand
            || nbr_species_below == Species::Plant
            || nbr_species_below == Species::Fungus
        {
            let new_rb = (api.rand_int(253) + 1) as u8;
            api.set(0, 0, Cell { rb: new_rb, ..cell });
            return;
        }

        let nbr = api.get(0, 1);
        if nbr.species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, 1, cell);
        } else if api.get(dxf, 1).species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(dxf, 1, cell);
        } else if nbr.species == Species::Water
            || nbr.species == Species::Gas
            || nbr.species == Species::Oil
            || nbr.species == Species::Acid
        {
            api.set(0, 0, nbr);
            api.set(0, 1, cell);
        } else {
            api.set(0, 0, cell);
        }
    } else {
        if ra > 60 {
            let dxr = api.rand_dir();
            if api.rand_int(100) > 75 {
                if (api.get(dxr, -1).species == Species::Empty
                    || api.get(dxr, -1).species == Species::Sand
                    || api.get(dxr, -1).species == Species::Seed)
                    && api.get(1, -1).species != Species::Plant
                    && api.get(-1, -1).species != Species::Plant
                {
                    let new_ra = (ra as i32 - api.rand_int(10)) as u8;
                    api.set(dxr, -1, Cell { ra: new_ra, ..cell });
                    let ra2 = 80 + api.rand_int(30) as u8;
                    api.set(0, 0, Cell { species: Species::Plant, ra: ra2, rb: 0, clock: 0 })
                } else {
                    api.set(0, 0, EMPTY_CELL);
                }
            }
        } else {
            if ra > 40 {
                let (mdx, mdy) = api.rand_vec();
                let (ldx, ldy) = adjacency_left((mdx, mdy));
                let (rdx, rdy) = adjacency_right((mdx, mdy));

                if (api.get(mdx, mdy).species == Species::Empty
                    || api.get(mdx, mdy).species == Species::Plant)
                    && (api.get(ldx, ldy).species == Species::Empty
                        || api.get(rdx, rdy).species == Species::Empty)
                {
                    let plant_i = (js_sys::Math::random() * js_sys::Math::random() * 100.) as i32;
                    let dec = 9 - api.rand_int(3);
                    if (plant_i + ra as i32) > 100 {
                        api.set(mdx, mdy, Cell { ra: (ra as i32 - dec) as u8, ..cell });
                    }
                }
            } else {
                if nbr_species == Species::Water {
                    api.set(dx, dy, Cell::new(Species::Seed))
                }
            }
        }
    }
}

pub fn update_fungus(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;
    let (dx, dy) = api.rand_vec();
    let nbr_species = api.get(dx, dy).species;

    if rb == 0 && nbr_species == Species::Fire || nbr_species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Fungus, ra: cell.ra, rb: 10, clock: 0 });
    }
    let mut i = api.rand_int(100);

    if nbr_species != Species::Empty
        && nbr_species != Species::Fungus
        && nbr_species != Species::Fire
        && nbr_species != Species::Ice
    {
        let (sdx, sdy) = api.rand_vec();
        let drift = (i % 15) - 7;
        let newra = (cell.ra as i32 + drift) as u8;
        if api.get(sdx, sdy).species == Species::Empty {
            api.set(sdx, sdy, Cell { species: Species::Fungus, ra: newra, rb: 0, clock: 0 });
        }
    }

    if i > 9
        && nbr_species == Species::Wood
        && api.get(-dx, dy).species == Species::Wood
        && api.get(dx, -dy).species == Species::Wood
        && api.get(dx, dy).ra % 4 != 0
    {
        i = api.rand_int(100);
        let drift = (i % 15) - 7;
        let newra = (cell.ra as i32 + drift) as u8;
        api.set(dx, dy, Cell { ra: newra, rb: 0, ..cell });
    }

    if rb > 1 {
        api.set(0, 0, Cell { ra: cell.ra, rb: rb - 1, ..cell });
        if nbr_species == Species::Empty {
            let ra = 10 + api.rand_int(10) as u8;
            api.set(dx, dy, Cell { species: Species::Fire, ra, rb: 0, clock: 0 })
        }
        if nbr_species == Species::Water {
            api.set(0, 0, Cell { ra: 50, rb: 0, ..cell })
        }
    } else if rb == 1 {
        api.set(0, 0, EMPTY_CELL);
    }

    let ra = cell.ra;
    if ra > 120 {
        let (mdx, mdy) = api.rand_vec();
        let (ldx, ldy) = adjacency_left((mdx, mdy));
        let (rdx, rdy) = adjacency_right((mdx, mdy));
        if api.get(mdx, mdy).species == Species::Empty
            && api.get(ldx, ldy).species != Species::Fungus
            && api.get(rdx, rdy).species != Species::Fungus
        {
            let fungus_i = (js_sys::Math::random() * js_sys::Math::random() * 100.) as i32;
            let dec = 15 - api.rand_int(20);
            if (fungus_i + ra as i32) > 165 {
                api.set(mdx, mdy, Cell { ra: (ra as i32 - dec) as u8, ..cell });
            }
        }
    }
}

pub fn update_mite(cell: Cell, mut api: SandApi) {
    let mut i = api.rand_int(100);
    let mut dx = 0;
    if cell.ra < 20 {
        dx = (cell.ra as i32) - 1;
    }
    let mut dy = 1;
    let mut mite = cell.clone();

    if cell.rb > 10 {
        mite.rb = mite.rb.saturating_sub(1);
        dy = -1;
    } else if cell.rb > 1 {
        mite.rb = mite.rb.saturating_sub(1);
    } else {
        dx = 0;
    }
    let nbr = api.get(dx, dy);

    let sx = (i % 3) - 1;
    i = api.rand_int(1000);
    let sy = (i % 3) - 1;
    let sample = api.get(sx, sy).species;
    if sample == Species::Fire
        || sample == Species::Lava
        || sample == Species::Water
        || sample == Species::Oil
    {
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    if (sample == Species::Plant || sample == Species::Wood || sample == Species::Seed) && i > 800 {
        api.set(0, 0, EMPTY_CELL);
        api.set(sx, sy, cell);
        return;
    }
    if sample == Species::Dust {
        api.set(sx, sy, if i > 800 { cell } else { EMPTY_CELL });
    }

    if nbr.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, dy, mite);
    } else if dy == 1 && i > 800 {
        i = api.rand_int(100);
        let mut ndx = (i % 3) - 1;
        if i < 6 {
            ndx = dx;
        }
        mite.ra = (1 + ndx) as u8;
        mite.rb = 10 + (i % 10) as u8;
        api.set(0, 0, mite);
    } else {
        if api.get(-1, 0).species == Species::Mite
            && api.get(1, 0).species == Species::Mite
            && api.get(0, -1).species == Species::Mite
        {
            api.set(0, 0, EMPTY_CELL);
        } else {
            if api.get(0, 1).species == Species::Ice {
                if api.get(dx, 0).species == Species::Empty {
                    api.set(0, 0, EMPTY_CELL);
                    api.set(dx, 0, mite);
                }
            } else {
                api.set(0, 0, mite);
            }
        }
    }
}

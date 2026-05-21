use crate::utils::*;
use crate::{Cell, SandApi, EMPTY_CELL};
use super::Species;
use std::mem;

pub fn update_ice(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let i = api.rand_int(100);
    let fluid = api.get_fluid();

    if fluid.pressure > 120 && api.rand_int(1) == 0 {
        api.set(0, 0, Cell { species: Species::Water, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }

    let nbr_species = api.get(dx, dy).species;
    if nbr_species == Species::Fire || nbr_species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Water, ra: cell.ra, rb: cell.rb, clock: 0 });
    } else if nbr_species == Species::Water && i < 7 {
        api.set(dx, dy, Cell { species: Species::Ice, ra: cell.ra, rb: cell.rb, clock: 0 });
    }
}

pub fn update_cloner(cell: Cell, mut api: SandApi) {
    let mut clone_species = unsafe { mem::transmute(cell.rb as u8) };
    let g = api.universe.generation;
    for cdx in [-1, 0, 1].iter().cloned() {
        for cdy in [-1, 0, 1].iter().cloned() {
            if cell.rb == 0 {
                let nbr_species = api.get(cdx, cdy).species;
                if nbr_species != Species::Empty
                    && nbr_species != Species::Cloner
                    && nbr_species != Species::Wall
                {
                    clone_species = nbr_species;
                    api.set(0, 0, Cell { species: cell.species, ra: 200, rb: clone_species as u8, clock: 0 });
                    break;
                }
            } else {
                if api.rand_int(100) > 90 && api.get(cdx, cdy).species == Species::Empty {
                    let ra = 80 + api.rand_int(30) as u8 + ((g % 127) as i8 - 60).abs() as u8;
                    api.set(cdx, cdy, Cell { species: clone_species, ra, rb: 0, clock: 0 });
                    break;
                }
            }
        }
    }
}

pub fn update_rocket(cell: Cell, mut api: SandApi) {
    if cell.rb == 0 {
        api.set(0, 0, Cell { ra: 0, rb: 100, ..cell });
        return;
    }

    let clone_species = if cell.rb != 100 {
        unsafe { mem::transmute(cell.rb as u8) }
    } else {
        Species::Sand
    };

    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);

    if cell.rb == 100
        && sample.species != Species::Empty
        && sample.species != Species::Rocket
        && sample.species != Species::Wall
        && sample.species != Species::Cloner
    {
        api.set(0, 0, Cell { ra: 1, rb: sample.species as u8, ..cell });
        return;
    }

    let ra = cell.ra;

    if ra == 0 {
        let dx = api.rand_dir();
        let nbr = api.get(0, 1);
        if nbr.species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, 1, cell);
        } else if api.get(dx, 1).species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, 1, cell);
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
    } else if ra == 1 {
        api.set(0, 0, Cell { ra: 2, ..cell });
    } else if ra == 2 {
        let (mut rdx, mut rdy) = api.rand_vec_8();
        let rnbr = api.get(rdx, rdy);
        if rnbr.species != Species::Empty {
            rdx *= -1;
            rdy *= -1;
        }
        api.set(0, 0, Cell { ra: 100 + join_dy_dx(rdx, rdy), ..cell });
    } else if ra > 50 {
        let (rdx, rdy) = split_dy_dx(cell.ra - 100);
        let rnbr = api.get(rdx, rdy * 2);

        if rnbr.species == Species::Empty
            || rnbr.species == Species::Fire
            || rnbr.species == Species::Rocket
        {
            api.set(0, 0, Cell::new(clone_species));
            api.set(0, rdy, Cell::new(clone_species));

            let (ndx, ndy) = match api.rand_int(100) % 5 {
                0 => adjacency_left((rdx, rdy)),
                1 => adjacency_right((rdx, rdy)),
                _ => (rdx, rdy),
            };
            api.set(rdx, rdy * 2, Cell { ra: 100 + join_dy_dx(ndx, ndy), ..cell });
        } else {
            api.set(0, 0, EMPTY_CELL);
        }
    }
}

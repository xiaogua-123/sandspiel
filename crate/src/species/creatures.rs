use crate::{Cell, SandApi, EMPTY_CELL};
use super::Species;

/// Simple creature that moves around, eats certain things, fears others
fn simple_creature(cell: Cell, api: &mut SandApi, favors_rise: bool) {
    let mut ra = cell.ra;
    let mut rb = cell.rb;

    // Starvation
    if ra < 5 {
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    ra = ra.saturating_sub(1);

    // Movement
    if rb > 0 {
        rb = rb.saturating_sub(1);
        let dx = (ra % 5) as i32 - 2;
        let dy = if favors_rise { -1 } else { 1 };
        let target = api.get(dx, dy);
        if target.species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(dx, dy, Cell { ra, rb, ..cell });
            return;
        } else {
            let (rdx, rdy) = api.rand_vec_8();
            if api.get(rdx, rdy).species == Species::Empty {
                api.set(0, 0, EMPTY_CELL);
                api.set(rdx, rdy, Cell { ra, rb, ..cell });
                return;
            }
        }
    }

    // Look for food or random move
    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);

    // Check danger
    if sample.species == Species::Fire || sample.species == Species::Lava
        || sample.species == Species::Poison || sample.species == Species::Acid {
        rb = 20;
        ra = ra.saturating_sub(10);
        api.set(0, 0, Cell { ra, rb, ..cell });
        return;
    }

    if sample.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(sx, sy, Cell { ra, rb, ..cell });
    } else {
        api.set(0, 0, Cell { ra, rb, ..cell });
    }
}

pub fn update_ant(cell: Cell, mut api: SandApi) {
    let ra = cell.ra;
    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);

    // Eat: sugar, honey, plants, dead things
    if matches!(sample.species, Species::Sugar | Species::Honey | Species::Bread
        | Species::Fruit | Species::Fungus | Species::Leaf | Species::Rice | Species::Wheat) {
        api.set(sx, sy, EMPTY_CELL);
        let new_ra = (ra + 30).min(200);
        api.set(0, 0, Cell { ra: new_ra, rb: cell.rb, ..cell });
        return;
    }

    // Ant trails - follow other ants
    if sample.species == Species::Ant && api.once_in(5) {
        api.set(sx, sy, cell);
        api.set(0, 0, EMPTY_CELL);
        return;
    }

    simple_creature(cell, &mut api, false);
}

pub fn update_spider(cell: Cell, mut api: SandApi) {
    let ra = cell.ra;
    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);

    // Eat: small insects
    if matches!(sample.species, Species::Ant | Species::Bee | Species::Butterfly
        | Species::Worm | Species::Mite) {
        api.set(sx, sy, EMPTY_CELL);
        let new_ra = (ra + 50).min(200);
        api.set(0, 0, Cell { ra: new_ra, rb: cell.rb, ..cell });
        return;
    }

    // Climbs on walls
    let climb_target = api.get(0, -1);
    if climb_target.species == Species::Wall || climb_target.species == Species::Wood {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, -1, cell);
        return;
    }

    simple_creature(cell, &mut api, true);
}

pub fn update_bee(cell: Cell, mut api: SandApi) {
    let ra = cell.ra;
    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);

    // Pollinates flowers
    if sample.species == Species::Flower {
        let new_ra = (ra + 20).min(200);
        api.set(0, 0, Cell { ra: new_ra, rb: cell.rb, ..cell });
        return;
    }

    // Bee flies upward
    if api.get(0, -1).species == Species::Empty && api.once_in(2) {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, -1, cell);
        return;
    }

    simple_creature(cell, &mut api, true);
}

pub fn update_butterfly(cell: Cell, mut api: SandApi) {
    // Butterfly: flutters randomly, pollinates
    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);
    if sample.species == Species::Flower {
        let new_ra = (cell.ra + 15).min(200);
        api.set(0, 0, Cell { ra: new_ra, rb: cell.rb, ..cell });
        return;
    }

    // Erratic flight pattern
    let (rdx, rdy) = api.rand_vec();
    let target = api.get(rdx, rdy);
    if target.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(rdx, rdy, cell);
    } else {
        simple_creature(cell, &mut api, true);
    }
}

pub fn update_fish(cell: Cell, mut api: SandApi) {
    // Fish: lives in water
    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);

    // Eats: seeds, small plants
    if matches!(sample.species, Species::Seed | Species::Plant | Species::Grass | Species::Moss) {
        api.set(sx, sy, EMPTY_CELL);
        let new_ra = (cell.ra + 20).min(200);
        api.set(0, 0, Cell { ra: new_ra, rb: cell.rb, ..cell });
        return;
    }

    // Dies out of water
    if sample.species != Species::Water {
        let in_water = api.get(1, 0).species == Species::Water
            || api.get(-1, 0).species == Species::Water
            || api.get(0, 1).species == Species::Water
            || api.get(0, -1).species == Species::Water;
        if !in_water {
            api.set(0, 0, Cell { ra: cell.ra.saturating_sub(5), rb: cell.rb, ..cell });
            return;
        }
    }

    // Prefer water movement
    if api.get(sx, sy).species == Species::Water {
        api.set(0, 0, Cell { species: Species::Water, ra: 100, rb: 0, clock: 0 });
        api.set(sx, sy, cell);
    } else {
        simple_creature(cell, &mut api, false);
    }
}

pub fn update_bird(cell: Cell, mut api: SandApi) {
    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);

    // Eats: seeds, insects, fruit
    if matches!(sample.species, Species::Seed | Species::Ant | Species::Bee
        | Species::Butterfly | Species::Worm | Species::Fruit | Species::Rice | Species::Wheat) {
        api.set(sx, sy, EMPTY_CELL);
        let new_ra = (cell.ra + 40).min(200);
        api.set(0, 0, Cell { ra: new_ra, rb: cell.rb, ..cell });
        return;
    }

    // Flies
    let bdx = api.rand_dir();
    if api.get(bdx, -1).species == Species::Empty && api.once_in(2) {
        api.set(0, 0, EMPTY_CELL);
        let bdx = api.rand_dir();
        api.set(bdx, -1, cell);
        return;
    }

    simple_creature(cell, &mut api, true);
}

pub fn update_snake(cell: Cell, mut api: SandApi) {
    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);

    // Eats: small creatures
    if matches!(sample.species, Species::Mite | Species::Ant | Species::Spider
        | Species::Fish | Species::Worm | Species::Egg) {
        api.set(sx, sy, EMPTY_CELL);
        let new_ra = (cell.ra + 60).min(250);
        api.set(0, 0, Cell { ra: new_ra, rb: cell.rb, ..cell });
        return;
    }

    simple_creature(cell, &mut api, false);
}

pub fn update_worm(cell: Cell, mut api: SandApi) {
    // Worm: lives in soil, aerates it
    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);

    // Eats: soil, decaying matter
    if matches!(sample.species, Species::Soil | Species::Peat
        | Species::Ash | Species::Leaf) {
        api.set(sx, sy, EMPTY_CELL);
        let new_ra = (cell.ra + 15).min(200);
        api.set(0, 0, Cell { ra: new_ra, rb: cell.rb, ..cell });
        return;
    }

    // Burrow into soil
    if sample.species == Species::Soil && api.once_in(5) {
        api.set(sx, sy, cell);
        api.set(0, 0, EMPTY_CELL);
        return;
    }

    simple_creature(cell, &mut api, false);
}

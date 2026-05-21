//! Extended organic simulation: leaf, flower, grass, vine, moss, mushroom, bark, root, fruit, thorn.
//! / 扩展有机物模拟：叶子、花、草、藤蔓、苔藓、蘑菇、树皮、根、水果、刺。
//!
//! These elements represent detailed plant and fungal life with growth, burning, and special behaviors.
//! / 这些元素代表具有生长、燃烧和特殊行为的详细植物和真菌生命。

use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

/// Helper: burnable solid that uses rb as burn timer.
/// / 辅助函数：使用 rb 作为燃烧计时器的可燃固体。
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

/// Leaf: light, drifts as it falls, burns easily.
/// / 叶子：轻，下落时飘动，容易燃烧。
pub fn update_leaf(cell: Cell, mut api: SandApi) {
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

/// Flower: attracts bees and butterflies for pollination, spreading seeds.
/// / 花：吸引蜜蜂和蝴蝶授粉，传播种子。
pub fn update_flower(cell: Cell, mut api: SandApi) {
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

/// Grass: spreads sideways on soil/sand, grows upward, burns.
/// / 草：在土壤/沙子上侧向扩散，向上生长，会燃烧。
pub fn update_grass(cell: Cell, mut api: SandApi) {
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

/// Vine: climbs upward along walls/wood/vine, spreading organically.
/// / 藤蔓：沿墙壁/木头/藤蔓向上攀爬，有机扩散。
pub fn update_vine(cell: Cell, mut api: SandApi) {
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

/// Moss: stationary ground cover, spreads on surfaces, thrives with water, withers without.
/// / 苔藓：静止的地被植物，在表面上扩散，有水则繁茂，无水则枯萎。
pub fn update_moss(cell: Cell, mut api: SandApi) {
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

/// Mushroom: grows on wood/fungus/soil, spreads spores to nearby empty spaces.
/// / 蘑菇：在木头/真菌/土壤上生长，将孢子扩散到附近的空白处。
pub fn update_mushroom(cell: Cell, mut api: SandApi) {
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

/// Bark: tough outer layer, highly fire resistant, turns to charcoal when burnt.
/// / 树皮：坚韧的外层，高度耐火，烧尽后变成木炭。
pub fn update_bark(cell: Cell, mut api: SandApi) {
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

/// Root: grows downward into soil/sand/clay, absorbs water for nourishment.
/// / 根：向下生长进入土壤/沙子/粘土，吸收水分作为养分。
pub fn update_root(cell: Cell, mut api: SandApi) {
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

/// Fruit: falls, ripens over time, rots into seeds when overripe (ra < 20).
/// / 水果：掉落，随时间成熟，过熟（ra < 20）时腐烂成种子。
pub fn update_fruit(cell: Cell, mut api: SandApi) {
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

/// Thorn: sharp defensive element, destroys creatures on contact, stationary, flammable.
/// / 刺：锋利的防御元素，接触时消灭生物，静止不动，可燃。
pub fn update_thorn(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if matches!(nbr.species, Species::Mite | Species::Ant | Species::Spider
        | Species::Bee | Species::Butterfly | Species::Bird | Species::Snake | Species::Worm) {
        api.set(dx, dy, EMPTY_CELL);
    }
    burnable_solid(cell, &mut api, 10);
}

//! Organic element simulation: wood, plant, seed, fungus, and mite.
//! / 有机元素模拟：木头、植物、种子、真菌和螨虫。

use crate::utils::*;
use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

/// Wood: flammable solid that burns slowly, extinguished by water.
/// / 木头：可燃固体，缓慢燃烧，被水熄灭。
/// rb = burn timer (0=unignited, 1=consumed, 2-90=burning).
/// / rb = 燃烧计时器（0=未点燃, 1=已消耗, 2-90=燃烧中）。
pub fn update_wood(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;
    let (dx, dy) = api.rand_vec();
    let nbr_species = api.get(dx, dy).species;

    // Ignite on contact with fire/lava / 接触火焰/岩浆时点燃
    if rb == 0 && nbr_species == Species::Fire || nbr_species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Wood, ra: cell.ra, rb: 90, clock: 0 });
    }

    if rb > 1 {
        api.set(0, 0, Cell { species: Species::Wood, ra: cell.ra, rb: rb - 1, clock: 0 });
        // Periodically spawn fire particles while burning / 燃烧时周期性产生火焰粒子
        if rb % 4 == 0 && nbr_species == Species::Empty {
            let ra = 30 + api.rand_int(60) as u8;
            api.set(dx, dy, Cell { species: Species::Fire, ra, rb: 0, clock: 0 })
        }
        // Water extinguishes the fire / 水熄灭火焰
        if nbr_species == Species::Water {
            api.set(0, 0, Cell { species: Species::Wood, ra: 50, rb: 0, clock: 0 });
            api.set_fluid(Wind { dx: 0, dy: 0, pressure: 0, density: 220 }); // steam from extinguishing / 灭火产生蒸汽
        }
    } else if rb == 1 {
        // Wood fully consumed, leaves ash behind / 木头烧尽，留下灰烬
        api.set(0, 0, Cell { species: Species::Empty, ra: cell.ra, rb: 90, clock: 0 });
    }
}

/// Plant: grows on wood, spreads, and burns easily.
/// / 植物：在木头上生长、扩散，容易燃烧。
pub fn update_plant(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;
    let mut i = api.rand_int(100);
    let (dx, dy) = api.rand_vec();
    let nbr_species = api.get(dx, dy).species;

    // Ignite / 点燃
    if rb == 0 && nbr_species == Species::Fire || nbr_species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Plant, ra: cell.ra, rb: 20, clock: 0 });
    }
    // Spread onto nearby wood / 向附近的木头扩散
    if nbr_species == Species::Wood {
        let (sdx, sdy) = api.rand_vec();
        let drift = (i % 15) - 7;
        let newra = (cell.ra as i32 + drift) as u8;
        if api.get(sdx, sdy).species == Species::Empty {
            api.set(sdx, sdy, Cell { species: Species::Plant, ra: newra, rb: 0, clock: 0 });
        }
    }
    // Grow toward water/fungus with random chance / 以随机几率向水/真菌生长
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

    // Burning: spawn fire particles, extinguish in water, turn to ash / 燃烧：产生火焰粒子，在水中熄灭，变灰烬
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

    // Growth: plant drops downward into empty space when ra > 50
    // / 生长：当 ra > 50 时植物向下方空白处延伸
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
            api.set(0, 0, Cell { ra: (ra - 1) as u8, ..cell }); // wither / 枯萎
        }
    }
}

/// Seed: falls, roots in sand/plant/fungus, grows into a plant.
/// / 种子：下落，在沙子/植物/真菌中生根，长成植物。
/// rb = growth stage: 0=falling, 1-253=rooted, high rb=spreading.
/// / rb = 生长阶段：0=下落中, 1-253=已生根, 高 rb 值=扩散中。
pub fn update_seed(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;
    let ra = cell.ra;
    let (dx, dy) = api.rand_vec();
    let nbr_species = api.get(dx, dy).species;

    // Burn on contact with fire/lava / 接触火焰/岩浆时燃烧
    if nbr_species == Species::Fire || nbr_species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Fire, ra: 5, rb: 0, clock: 0 });
        return;
    }

    if rb == 0 {
        // Falling phase: look for suitable ground to root / 下落阶段：寻找合适的土地生根
        let dxf = api.rand_dir();
        let nbr_species_below = api.get(dxf, 1).species;
        if nbr_species_below == Species::Sand
            || nbr_species_below == Species::Plant
            || nbr_species_below == Species::Fungus
        {
            // Root in suitable ground / 在合适的土地上生根
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
        // Rooted phase: grow into a plant / 生根阶段：长成植物
        if ra > 60 {
            // Full grown: spread upward / 完全成熟：向上扩散
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
                    api.set(0, 0, Cell { species: Species::Plant, ra: ra2, rb: 0, clock: 0 }) // spawn plant / 生成植物
                } else {
                    api.set(0, 0, EMPTY_CELL);
                }
            }
        } else {
            if ra > 40 {
                // Growing: spread with diffusion pattern using adjacency / 生长中：使用邻接的扩散模式扩散
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
                // Young: reproduce in water / 幼年：在水中繁殖
                if nbr_species == Species::Water {
                    api.set(dx, dy, Cell::new(Species::Seed))
                }
            }
        }
    }
}

/// Fungus: spreads on wood, flammable, grows in damp conditions.
/// / 真菌：在木头上扩散，可燃，在潮湿环境中生长。
pub fn update_fungus(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;
    let (dx, dy) = api.rand_vec();
    let nbr_species = api.get(dx, dy).species;

    // Ignite on contact with fire/lava / 接触火焰/岩浆时点燃
    if rb == 0 && nbr_species == Species::Fire || nbr_species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Fungus, ra: cell.ra, rb: 10, clock: 0 });
    }
    let mut i = api.rand_int(100);

    // Spread onto non-empty, non-fungus, non-fire, non-ice cells / 扩散到非空、非真菌、非火、非冰的单元格
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

    // Consume wood: fungus thrives on surrounded wood / 消耗木头：真菌在被木头包围时茁壮成长
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

    // Burning state / 燃烧状态
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

    // Spread when mature (ra > 120) / 成熟时扩散（ra > 120）
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

/// Mite: tiny creature that moves, eats plants, avoids fire/water.
/// / 螨虫：微小生物，移动，吃植物，避开火/水。
/// ra = health/energy; rb = jump/bounce timer.
/// / ra = 生命值/能量；rb = 跳跃/弹跳计时器。
pub fn update_mite(cell: Cell, mut api: SandApi) {
    let mut i = api.rand_int(100);
    let mut dx = 0;
    if cell.ra < 20 {
        dx = (cell.ra as i32) - 1; // weak mites drift / 虚弱的螨虫漂移
    }
    let mut dy = 1;
    let mut mite = cell.clone();

    // Jump/bounce mechanic using rb / 使用 rb 的跳跃/弹跳机制
    if cell.rb > 10 {
        mite.rb = mite.rb.saturating_sub(1);
        dy = -1; // bounce upward / 向上弹跳
    } else if cell.rb > 1 {
        mite.rb = mite.rb.saturating_sub(1);
    } else {
        dx = 0;
    }
    let nbr = api.get(dx, dy);

    // Scan random nearby cell / 扫描随机附近单元格
    let sx = (i % 3) - 1;
    i = api.rand_int(1000);
    let sy = (i % 3) - 1;
    let sample = api.get(sx, sy).species;
    // Die from fire, lava, water, oil / 死于火、岩浆、水、油
    if sample == Species::Fire
        || sample == Species::Lava
        || sample == Species::Water
        || sample == Species::Oil
    {
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    // Eat plants/wood/seeds / 吃植物/木头/种子
    if (sample == Species::Plant || sample == Species::Wood || sample == Species::Seed) && i > 800 {
        api.set(0, 0, EMPTY_CELL);
        api.set(sx, sy, cell);
        return;
    }
    // Interact with dust / 与灰尘互动
    if sample == Species::Dust {
        api.set(sx, sy, if i > 800 { cell } else { EMPTY_CELL });
    }

    if nbr.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, dy, mite);
    } else if dy == 1 && i > 800 {
        // Attempt random jump when falling is blocked / 下落受阻时尝试随机跳跃
        i = api.rand_int(100);
        let mut ndx = (i % 3) - 1;
        if i < 6 {
            ndx = dx;
        }
        mite.ra = (1 + ndx) as u8;
        mite.rb = 10 + (i % 10) as u8;
        api.set(0, 0, mite);
    } else {
        // Crowding: die if surrounded by mites / 拥挤：被螨虫包围时死亡
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

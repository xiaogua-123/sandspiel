//! Newer element types: snow, sponge, slime, glass, and coral.
//! / 较新的元素类型：雪、海绵、粘液、玻璃和珊瑚。

use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

/// Snow: light powder, floats on water, melts when hot.
/// / 雪：轻粉末，浮在水上，受热融化。
pub fn update_snow(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();

    // Melt if near fire/lava / 靠近火焰/岩浆时融化
    let sample = api.get(dx, dy);
    if sample.species == Species::Fire || sample.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Water, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }
    // Melt from heat pressure / 热量压力下融化
    let fluid = api.get_fluid();
    if fluid.pressure > 100 {
        api.set(0, 0, Cell { species: Species::Water, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }

    // Snow falls slowly, drifts / 雪缓慢下落，飘动
    let dx2 = api.rand_dir();
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if below.species == Species::Water {
        // Snow floats on water, then slowly melts / 雪浮在水上，然后慢慢融化
        if api.once_in(10) {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, 1, cell);
        } else {
            api.set(0, 0, cell);
        }
    } else if api.get(dx2, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx2, 1, cell);
    } else if api.get(dx2, 0).species == Species::Empty && api.once_in(3) {
        // Light drift sideways / 轻轻侧向飘动
        api.set(0, 0, EMPTY_CELL);
        api.set(dx2, 0, cell);
    } else if below.species == Species::Ice && api.once_in(4) {
        // Accumulate on ice surface / 在冰面上堆积
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

/// Sponge: absorbs nearby liquids, expands when wet, releases when squeezed.
/// / 海绵：吸收附近液体，湿润时膨胀，被挤压时释放。
pub fn update_sponge(cell: Cell, mut api: SandApi) {
    let mut absorbed = cell.ra; // liquid absorption level / 液体吸收量 (0 = dry / 干燥)
    let (dx, dy) = api.rand_vec();

    // Absorb nearby liquid
    for adx in [-1, 0, 1].iter().cloned() {
        for ady in [-1, 0, 1].iter().cloned() {
            if absorbed >= 200 { break; }
            let nbr = api.get(adx, ady);
            if nbr.species == Species::Water || nbr.species == Species::Oil {
                absorbed = absorbed.saturating_add(30);
                api.set(adx, ady, EMPTY_CELL);
            }
        }
    }

    // Burn if near fire (wet sponge resists fire) / 靠近火焰时燃烧（湿海绵耐火）
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        if absorbed > 50 {
            // Wet sponge resists fire, just loses moisture / 湿海绵耐火，只损失水分
            api.set(0, 0, Cell { ra: absorbed.saturating_sub(5), ..cell });
        } else {
            api.set(0, 0, Cell { species: Species::Fire, ra: 30, rb: 0, clock: 0 });
        }
        return;
    }

    // Drip excess water downward / 多余的水向下滴
    if absorbed > 180 && api.once_in(5) {
        let below = api.get(0, 1);
        if below.species == Species::Empty {
            api.set(0, 1, Cell::new(Species::Water));
            absorbed = absorbed.saturating_sub(30);
        }
    }

    // Squeeze if something heavy falls on it / 如果有重物落在上面则挤压
    if api.get(0, -1).species == Species::Sand || api.get(0, -1).species == Species::Stone {
        if absorbed > 30 && api.once_in(3) {
            let side_dx = api.rand_dir();
            if api.get(side_dx, 0).species == Species::Empty {
                api.set(side_dx, 0, Cell::new(Species::Water));
            }
            absorbed = absorbed.saturating_sub(30);
        }
    }

    // Slowly dry out over time / 随时间慢慢变干
    if absorbed > 0 && api.once_in(20) {
        absorbed = absorbed.saturating_sub(1);
    }

    api.set(0, 0, Cell { ra: absorbed, ..cell });
}

/// Slime: bouncy, sticky goo that slowly moves down and spreads.
/// / 粘液：有弹性的粘稠物，缓慢向下移动并扩散。
pub fn update_slime(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;
    let (dx, dy) = api.rand_vec();

    // Burn / 燃烧
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Fire, ra: 20, rb: 0, clock: 0 });
        return;
    }

    // Washed away by water / 被水冲走
    if nbr.species == Species::Water && api.once_in(3) {
        api.set(0, 0, nbr);
        api.set(dx, dy, cell);
        return;
    }

    // Stretchy, slow movement / 有弹性的缓慢移动
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        // Fall slowly due to stickiness / 因粘性而缓慢下落
        if api.once_in(3) {
            api.set(0, 0, EMPTY_CELL);
            api.set(0, 1, cell);
        } else {
            api.set(0, 0, cell);
        }
    } else if below.species == Species::Slime && api.once_in(4) {
        // Stick together, move as blob / 粘在一起，作为整体移动
        let sdx = api.rand_dir();
        if api.get(sdx, 1).species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            api.set(sdx, 1, cell);
        } else {
            api.set(0, 0, cell);
        }
    } else {
        // Spread sideways on surfaces / 在表面上向侧面扩散
        let sdx = api.rand_dir();
        if api.get(sdx, 0).species == Species::Empty && api.once_in(3) {
            api.set(0, 0, EMPTY_CELL);
            api.set(sdx, 0, cell);
        } else if api.get(sdx, 1).species == Species::Empty && api.once_in(4) {
            api.set(0, 0, EMPTY_CELL);
            api.set(sdx, 1, cell);
        } else if api.get(dx, dy).species == Species::Acid {
            // Acid dissolves slime / 酸溶解粘液
            api.set(0, 0, nbr);
            api.set(dx, dy, EMPTY_CELL);
        } else {
            api.set(0, 0, cell);
        }
    }

    // Bounce: if fell from height, spread a bit / 弹跳：若从高处落下，稍微扩散
    if rb > 0 {
        api.set(0, 0, Cell { rb: rb.saturating_sub(1), ..cell });
        let bdx = api.rand_dir();
        if api.get(bdx, 0).species == Species::Empty {
            api.set(bdx, 0, cell);
            api.set(0, 0, EMPTY_CELL);
        }
    }
}

/// Glass: hard and transparent, melts into lava at high temperature.
/// / 玻璃：坚硬透明，高温下熔化成岩浆。
pub fn update_glass(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();

    // Melt when near fire/lava / 靠近火焰/岩浆时熔化
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        if api.once_in(15) {
            api.set(0, 0, Cell { species: Species::Lava, ra: 80, rb: 0, clock: 0 });
            return;
        }
    }

    // Shatter under extreme pressure / 极端压力下破碎
    let fluid = api.get_fluid();
    if fluid.pressure > 200 && api.once_in(5) {
        api.set(0, 0, Cell { species: Species::Sand, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }

    // Acid slowly dissolves glass / 酸缓慢溶解玻璃
    if nbr.species == Species::Acid && api.once_in(10) {
        api.set(0, 0, Cell { species: Species::Empty, ra: 0, rb: 0, clock: 0 });
        return;
    }

    // Falls like stone through empty/density space / 像石头一样在空白/密度空间中下落
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if below.species == Species::Water || below.species == Species::Oil || below.species == Species::Gas {
        api.set(0, 0, below);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

/// Coral: grows upward in water, branches sideways, dies out of water.
/// / 珊瑚：在水中向上生长，侧向分支，离开水会死亡。
pub fn update_coral(cell: Cell, mut api: SandApi) {
    let ra = cell.ra;
    let (dx, dy) = api.rand_vec();

    // Die from fire/lava / 被火焰/岩浆杀死
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Empty, ra: 0, rb: 0, clock: 0 });
        return;
    }

    // Die out of water - check all 4 cardinal directions / 离开水就会死亡 - 检查所有四个方向
    let in_water = api.get(0, 1).species == Species::Water
        || api.get(1, 0).species == Species::Water
        || api.get(-1, 0).species == Species::Water
        || api.get(0, -1).species == Species::Water;
    if !in_water && api.once_in(20) {
        api.set(0, 0, Cell { species: Species::Stone, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }

    // Grow upward through water / 通过水向上生长
    if ra > 30 && api.once_in(8) {
        let above = api.get(0, -1);
        if above.species == Species::Water {
            let new_ra = (ra as i32 + api.rand_int(10)).saturating_sub(5) as u8;
            api.set(0, -1, Cell { species: Species::Coral, ra: new_ra, rb: 0, clock: 0 });
        }
    }

    // Branch sideways in water / 在水中侧向分支
    if ra > 60 && api.once_in(12) {
        let bdx = if api.once_in(2) { 1 } else { -1 };
        let side = api.get(bdx, 0);
        if side.species == Species::Water {
            let new_ra = (ra as i32 + api.rand_int(10)).saturating_sub(5) as u8;
            api.set(bdx, 0, Cell { species: Species::Coral, ra: new_ra.clamp(10, 100), rb: 0, clock: 0 });
        }
    }

    // Gradually grow taller / 逐渐长高
    if ra < 100 && api.once_in(6) {
        api.set(0, 0, Cell { ra: ra + 1, ..cell });
    }
}

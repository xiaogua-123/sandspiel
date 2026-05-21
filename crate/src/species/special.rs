//! Special element simulation: ice, cloner, and rocket.
//! / 特殊元素模拟：冰、克隆器和火箭。

use crate::utils::*;
use crate::{Cell, SandApi, EMPTY_CELL};
use super::Species;
use std::mem;

/// Ice: frozen water, shatters under pressure, melts near heat, freezes water.
/// / 冰：冻结的水，压力下破碎，靠近热源融化，冷冻水。
pub fn update_ice(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let i = api.rand_int(100);
    let fluid = api.get_fluid();

    // Shatter under pressure / 压力下破碎
    if fluid.pressure > 120 && api.rand_int(1) == 0 {
        api.set(0, 0, Cell { species: Species::Water, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }

    let nbr_species = api.get(dx, dy).species;
    // Melt near fire/lava / 靠近火焰/岩浆融化
    if nbr_species == Species::Fire || nbr_species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Water, ra: cell.ra, rb: cell.rb, clock: 0 });
    } else if nbr_species == Species::Water && i < 7 {
        // Freeze adjacent water / 冻结相邻的水
        api.set(dx, dy, Cell { species: Species::Ice, ra: cell.ra, rb: cell.rb, clock: 0 });
    }
}

/// Cloner: copies nearby elements and spawns them in adjacent empty spaces.
/// / 克隆器：复制附近的元素并将其生成在相邻的空白区域。
/// rb = species index of the element to clone (0 = scanning).
/// / rb = 要克隆的元素的物种索引（0 = 扫描中）。
pub fn update_cloner(cell: Cell, mut api: SandApi) {
    let mut clone_species = unsafe { mem::transmute(cell.rb as u8) };
    let g = api.universe.generation;
    for cdx in [-1, 0, 1].iter().cloned() {
        for cdy in [-1, 0, 1].iter().cloned() {
            if cell.rb == 0 {
                // Scanning: find a species to clone / 扫描：寻找要克隆的物种
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
                // Cloning: spawn copies in empty spaces / 克隆：在空白处生成副本
                if api.rand_int(100) > 90 && api.get(cdx, cdy).species == Species::Empty {
                    let ra = 80 + api.rand_int(30) as u8 + ((g % 127) as i8 - 60).abs() as u8;
                    api.set(cdx, cdy, Cell { species: clone_species, ra, rb: 0, clock: 0 });
                    break;
                }
            }
        }
    }
}

/// Rocket: copies a nearby element, then launches in a direction while spawning trails.
/// / 火箭：复制附近的元素，然后向一个方向发射并留下痕迹。
/// ra: 0=falling, 1=scanning, 2=arming, 3-50=launch readiness, 50+=flying.
/// / ra: 0=下落中, 1=扫描中, 2=准备发射, 3-50=发射就绪, 50+=飞行中。
pub fn update_rocket(cell: Cell, mut api: SandApi) {
    // Initialize rocket state if not yet set / 如果尚未设置则初始化火箭状态
    if cell.rb == 0 {
        api.set(0, 0, Cell { ra: 0, rb: 100, ..cell });
        return;
    }

    // Determine the species to clone as trail / 确定要作为轨迹克隆的物种
    let clone_species = if cell.rb != 100 {
        unsafe { mem::transmute(cell.rb as u8) } // stored species / 存储的物种
    } else {
        Species::Sand // default / 默认
    };

    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);

    // Scanning: memorize a nearby element / 扫描：记录附近的元素
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
        // Falling phase: behaves like sand / 下落阶段：像沙子一样
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
        // Armed, ready to launch / 准备就绪，等待发射
        api.set(0, 0, Cell { ra: 2, ..cell });
    } else if ra == 2 {
        // Choose a launch direction (avoid obstacles) / 选择发射方向（避开障碍物）
        let (mut rdx, mut rdy) = api.rand_vec_8();
        let rnbr = api.get(rdx, rdy);
        if rnbr.species != Species::Empty {
            rdx *= -1; // reverse direction if blocked / 如果被阻挡则反转方向
            rdy *= -1;
        }
        // Encode direction into ra using join_dy_dx / 使用 join_dy_dx 将方向编码到 ra 中
        api.set(0, 0, Cell { ra: 100 + join_dy_dx(rdx, rdy), ..cell });
    } else if ra > 50 {
        // Flying phase: move in encoded direction, leave trail / 飞行阶段：按编码方向移动，留下轨迹
        let (rdx, rdy) = split_dy_dx(cell.ra - 100); // decode direction / 解码方向
        let rnbr = api.get(rdx, rdy * 2);

        if rnbr.species == Species::Empty
            || rnbr.species == Species::Fire
            || rnbr.species == Species::Rocket
        {
            // Leave a trail behind / 在身后留下轨迹
            api.set(0, 0, Cell::new(clone_species));
            api.set(0, rdy, Cell::new(clone_species));

            // Slightly randomize trajectory / 稍微随机化轨道
            let (ndx, ndy) = match api.rand_int(100) % 5 {
                0 => adjacency_left((rdx, rdy)),
                1 => adjacency_right((rdx, rdy)),
                _ => (rdx, rdy),
            };
            api.set(rdx, rdy * 2, Cell { ra: 100 + join_dy_dx(ndx, ndy), ..cell });
        } else {
            // Hit something, disappear / 击中物体，消失
            api.set(0, 0, EMPTY_CELL);
        }
    }
}

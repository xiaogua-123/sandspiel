//! Crystal and gem element simulation: diamond, ruby, sapphire, emerald, amethyst, quartz, crystal, obsidian.
//! / 晶体和宝石元素模拟：钻石、红宝石、蓝宝石、祖母绿、紫水晶、石英、水晶、黑曜石。
//!
//! Crystals are heavy solids that shatter under pressure, melt at high temperatures,
//! and some have special properties (piezoelectricity, laser interaction, etc.).
//! / 晶体是重型固体，在压力下碎裂，高温下熔化，
//! / 有些具有特殊属性（压电效应、激光互动等）。

use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

/// Helper: crystal fall physics - applies to all gems/crystals.
/// / 辅助函数：晶体下落物理 - 适用于所有宝石/水晶。
/// shatter_resist: higher = harder to shatter / 越高 = 越难碎裂
/// melt_temp: higher = more heat resistant / 越高 = 越耐热
fn crystal_fall(cell: Cell, api: &mut SandApi, shatter_resist: i32, melt_temp: i32, melt_into: Species) {
    let (dx, dy) = api.rand_vec_8();
    let nbr = api.get(dx, dy);
    let fluid = api.get_fluid();

    // Melt near extreme heat
    if melt_temp > 0 && api.once_in(melt_temp) && (nbr.species == Species::Lava
        || (nbr.species == Species::Fire && api.once_in(3))) {
        api.set(0, 0, Cell { species: melt_into, ra: 100, rb: 0, clock: 0 });
        return;
    }
    // Shatter under high pressure
    if fluid.pressure > 150 && api.once_in(shatter_resist) {
        api.set(0, 0, Cell { species: Species::Sand, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }
    // Fall like stone
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if below.species == Species::Water || below.species == Species::Oil
        || below.species == Species::Gas || below.species == Species::Acid {
        api.set(0, 0, below);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

/// Diamond: hardest crystal, very difficult to shatter or melt.
/// / 钻石：最坚硬的晶体，极难碎裂或熔化。
pub fn update_diamond(cell: Cell, mut api: SandApi) {
    crystal_fall(cell, &mut api, 30, 60, Species::Lava); // hardest to shatter / 最难碎裂
}

/// Ruby: interacts with lasers, amplifying light energy.
/// / 红宝石：与激光互动，放大光能。
pub fn update_ruby(cell: Cell, mut api: SandApi) {
    let rdx = api.rand_dir();
    let rdy = api.rand_dir();
    let nbr = api.get(rdx, rdy);
    // Ruby amplifies laser energy into high density / 红宝石将激光能量放大为高密度
    if nbr.species == Species::Laser {
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 0, density: 200 });
    }
    crystal_fall(cell, &mut api, 20, 50, Species::Lava);
}

/// Sapphire: very hard blue gem, high heat resistance.
/// / 蓝宝石：极硬的蓝色宝石，高耐热性。
pub fn update_sapphire(cell: Cell, mut api: SandApi) {
    crystal_fall(cell, &mut api, 22, 55, Species::Lava);
}

/// Emerald: green gem, slightly more fragile than other gems.
/// / 祖母绿：绿色宝石，比其他宝石略脆弱。
pub fn update_emerald(cell: Cell, mut api: SandApi) {
    let fluid = api.get_fluid();
    if fluid.pressure > 130 && api.once_in(15) {
        api.set(0, 0, Cell { species: Species::Sand, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }
    crystal_fall(cell, &mut api, 15, 45, Species::Lava);
}

/// Amethyst: purple quartz, moderate durability.
/// / 紫水晶：紫色石英，中等耐久性。
pub fn update_amethyst(cell: Cell, mut api: SandApi) {
    crystal_fall(cell, &mut api, 18, 40, Species::Lava);
}

/// Quartz: piezoelectric crystal, generates sparks under pressure.
/// / 石英：压电晶体，在压力下产生火花。
pub fn update_quartz(cell: Cell, mut api: SandApi) {
    let fluid = api.get_fluid();
    // Piezoelectric effect: pressure creates sparks / 压电效应：压力产生火花
    if fluid.pressure > 100 && api.once_in(15) {
        api.set_fluid(Wind { dx: 50, dy: 50, pressure: 30, density: 10 });
    }
    crystal_fall(cell, &mut api, 12, 35, Species::Lava);
}

/// Regular crystal: fragile, low melting point, turns to glass when melted.
/// / 普通水晶：脆弱，低熔点，熔化后变成玻璃。
pub fn update_crystal(cell: Cell, mut api: SandApi) {
    crystal_fall(cell, &mut api, 10, 25, Species::Glass);
}

/// Obsidian: volcanic glass, sharp, shatters into fragments, cracks near water.
/// / 黑曜石：火山玻璃，锋利，碎裂成碎片，遇水开裂。
pub fn update_obsidian(cell: Cell, mut api: SandApi) {
    let fluid = api.get_fluid();
    if fluid.pressure > 140 && api.once_in(14) {
        // Shatters into sharp glass-like fragments / 碎裂成尖锐的玻璃状碎片
        let (sdx, sdy) = api.rand_vec();
        api.set(sdx, sdy, Cell { species: Species::Sand, ra: 50, rb: 0, clock: 0 });
        api.set(0, 0, Cell { species: Species::Sand, ra: 50, rb: 0, clock: 0 });
        return;
    }
    // Water causes rapid cooling and cracking / 水导致快速冷却和开裂
    let odx = api.rand_dir();
    let ody = api.rand_dir();
    let nbr = api.get(odx, ody);
    if nbr.species == Species::Water && api.once_in(20) {
        api.set(0, 0, Cell { species: Species::Stone, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }
    crystal_fall(cell, &mut api, 14, 45, Species::Lava);
}

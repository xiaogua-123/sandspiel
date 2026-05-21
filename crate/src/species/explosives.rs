//! Explosive and hazardous element simulation: TNT, bomb, nitro, plutonium, uranium, C4, thermite, napalm.
//! / 爆炸物和危险元素模拟：TNT、炸弹、硝化甘油、钚、铀、C4、铝热剂、凝固汽油。
//!
//! These elements range from stable explosives requiring triggers to radioactive elements
//! that continuously emit heat, and sticky incendiaries.
//! / 这些元素从需要触发条件的稳定爆炸物到持续释放热量的放射性元素和粘性燃烧剂不等。

use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

/// Helper: create an explosion with shockwave spreading outward.
/// / 辅助函数：产生向外扩散冲击波的爆炸。
fn explode(api: &mut SandApi, power: u8, spread_fire: bool) {
    api.set(0, 0, EMPTY_CELL);
    let pressure = (power as u8).saturating_mul(2);
    let density = power;
    api.set_fluid(Wind { dx: 0, dy: 0, pressure, density });

    for dx in -2..=2 {
        for dy in -2..=2 {
            if dx == 0 && dy == 0 { continue; }
            let cell = api.get(dx, dy);
            if cell.species != Species::Wall && cell.species != Species::Empty
                && cell.species != Species::Void && cell.species != Species::Shield {
                if spread_fire && api.once_in(2) {
                    api.set(dx, dy, Cell { species: Species::Fire, ra: 150, rb: 0, clock: 0 });
                }
            }
            // Shockwave: push things away
            let fluid = api.get_fluid();
            if fluid.pressure < pressure {
                api.set_fluid(Wind {
                    dx: (dx * 30) as u8,
                    dy: (dy * 30) as u8,
                    pressure: pressure / 3,
                    density: density / 2,
                });
            }
        }
    }
}

fn falling_explosive(cell: Cell, api: &mut SandApi, trigger: bool, power: u8) {
    if trigger {
        explode(api, power, true);
        return;
    }
    // Fall like sand
    let dx = api.rand_dir_2();
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

/// TNT: explodes from fire/lava/lightning/plasma or high-pressure impact.
/// / TNT：被火焰/岩浆/闪电/等离子体点燃或高压冲击后爆炸。
pub fn update_tnt(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    let triggered = nbr.species == Species::Fire || nbr.species == Species::Lava
        || nbr.species == Species::Lightning || nbr.species == Species::PlasmaGas;
    // TNT also detonates from impact (high pressure) / TNT 也会因高压冲击引爆
    let fluid = api.get_fluid();
    let impact = fluid.pressure > 100;
    falling_explosive(cell, &mut api, triggered || impact, 120);
}

/// Bomb: powerful explosive triggered by fire/lava/lightning/TNT, stationary until triggered.
/// / 炸弹：由火焰/岩浆/闪电/TNT 触发的强力爆炸物，在触发前保持静止。
pub fn update_bomb(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    let triggered = nbr.species == Species::Fire || nbr.species == Species::Lava
        || nbr.species == Species::Lightning || nbr.species == Species::TNT;
    if triggered {
        explode(&mut api, 150, true);
        return;
    }
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

/// Nitroglycerin: extremely unstable, detonates from any shock, heat, or slight pressure.
/// / 硝化甘油：极度不稳定，会被任何冲击、热量或轻微压力引爆。
pub fn update_nitro(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    let fluid = api.get_fluid();
    // Detonates from nearly anything / 几乎任何东西都会引爆
    if nbr.species == Species::Fire || nbr.species == Species::Lava
        || fluid.pressure > 40 || nbr.species != Species::Empty {
        explode(&mut api, 180, true);
        return;
    }
    let below = api.get(0, 1);
    if below.species == Species::Empty && api.once_in(2) {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

/// Plutonium: radioactive heavy metal, continuously heats surroundings, may ignite nearby elements.
/// / 钚：放射性重金属，持续加热周围环境，可能点燃附近的元素。
pub fn update_plutonium(cell: Cell, mut api: SandApi) {
    // Radiates heat / 辐射热量
    api.set_fluid(Wind { dx: 0, dy: 0, pressure: 5, density: 80 });
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species != Species::Empty && nbr.species != Species::Wall
        && nbr.species != Species::Plutonium {
        if api.once_in(30) {
            api.set(dx, dy, Cell { species: Species::Fire, ra: 50, rb: 0, clock: 0 });
        }
    }
    heavy_fall_plutonium(cell, &mut api);
}

/// Heavy fall helper for radioactive elements (plutonium, uranium).
/// / 放射性元素（钚、铀）的重型下落辅助函数。
fn heavy_fall_plutonium(cell: Cell, api: &mut SandApi) {
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if matches!(below.species, Species::Water | Species::Oil | Species::Gas | Species::Acid) {
        api.set(0, 0, below);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

/// Uranium: radioactive like plutonium but slightly less active.
/// / 铀：与钚类似，放射性略低。
pub fn update_uranium(cell: Cell, mut api: SandApi) {
    api.set_fluid(Wind { dx: 0, dy: 0, pressure: 3, density: 60 });
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species != Species::Empty && nbr.species != Species::Wall
        && nbr.species != Species::Uranium && nbr.species != Species::Plutonium {
        if api.once_in(50) {
            api.set(dx, dy, Cell { species: Species::Fire, ra: 30, rb: 0, clock: 0 });
        }
    }
    heavy_fall_plutonium(cell, &mut api);
}

/// C4: stable plastic explosive, needs fire/lava to detonate, otherwise inert.
/// / C4：稳定的塑性炸药，需要火焰/岩浆引爆，否则呈惰性。
pub fn update_c4(cell: Cell, mut api: SandApi) {
    let edx = api.rand_dir();
    let edy = api.rand_dir();
    let nbr = api.get(edx, edy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        explode(&mut api, 160, true);
        return;
    }
    // Falls like sand, stable
    falling_explosive(cell, &mut api, false, 0);
}

/// Thermite: burns incredibly hot (density=200), melts through nearly everything.
/// / 铝热剂：燃烧温度极高（密度=200），熔化几乎一切。
pub fn update_thermite(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;
    if rb > 0 {
        // Already ignited - burns through everything
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 0, density: 200 });
        let below = api.get(0, 1);
        if below.species != Species::Wall && below.species != Species::Void {
            api.set(0, 1, Cell { species: Species::Lava, ra: 200, rb: 0, clock: 0 });
        }
        api.set(0, 0, Cell { rb: rb - 1, ..cell });
        if rb <= 1 {
            api.set(0, 0, EMPTY_CELL);
        }
        return;
    }
    // Check for ignition
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { rb: 40, ..cell });
        return;
    }
    falling_explosive(cell, &mut api, false, 0);
}

/// Napalm: sticky burning gel, clings to surfaces, spreads fire everywhere while burning.
/// / 凝固汽油：粘性燃烧凝胶，附着在表面上，燃烧时到处扩散火焰。
pub fn update_napalm(cell: Cell, mut api: SandApi) {
    let rb = cell.rb;
    if rb > 0 {
        // Burning - spreads fire everywhere
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 0, density: 180 });
        let (dx, dy) = api.rand_vec();
        let nbr = api.get(dx, dy);
        if nbr.species == Species::Empty && api.once_in(2) {
            api.set(dx, dy, Cell { species: Species::Fire, ra: 150, rb: 0, clock: 0 });
        }
        // Sticky - doesn't fall
        api.set(0, 0, Cell { rb: rb.saturating_sub(1), ..cell });
        if rb <= 1 {
            api.set(0, 0, Cell { species: Species::Fire, ra: 50, rb: 0, clock: 0 });
        }
        return;
    }
    // Check for ignition
    let (dx, dy) = api.rand_vec();
    if api.get(dx, dy).species == Species::Fire {
        api.set(0, 0, Cell { rb: 30, ..cell });
        return;
    }
    falling_explosive(cell, &mut api, false, 0);
}

//! Powder element simulation: gunpowder, flour, sugar, salt, pepper, ash, soot, charcoal.
//! / 粉末元素模拟：火药、面粉、糖、盐、胡椒、灰烬、烟灰、木炭。
//!
//! Powders are granular solids with varying degrees of flammability and explosiveness.
//! / 粉末是颗粒状固体，具有不同程度的可燃性和爆炸性。

use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

/// Powder that falls slowly and can drift sideways.
/// / 缓慢下落并可侧向飘动的粉末。
/// flammable: catches fire / 可燃：会着火
/// explosive: detonates under pressure / 爆炸性：在压力下爆炸
fn powder_fall(cell: Cell, api: &mut SandApi, flammable: bool, explosive: bool) {
    let dx = api.rand_dir();
    let fluid = api.get_fluid();

    // Explosive powders detonate under high pressure / 爆炸性粉末在高压下引爆
    if explosive && fluid.pressure > 100 {
        api.set(0, 0, Cell { species: Species::Fire, ra: 200, rb: 0, clock: 0 });
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 100, density: 50 });
        return;
    }

    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if below.species == Species::Water {
        // Some powders sink through water / 一些粉末沉入水中
        api.set(0, 0, below);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if api.get(dx, 0).species == Species::Empty && api.once_in(4) {
        // Drift sideways / 侧向飘动
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else {
        api.set(0, 0, cell);
    }

    // Check nearby fire for flammable powders / 检查附近火焰以点燃可燃粉末
    if flammable {
        let (sx, sy) = api.rand_vec();
        let nbr = api.get(sx, sy);
        if nbr.species == Species::Fire || nbr.species == Species::Lava {
            api.set(0, 0, Cell { species: Species::Fire, ra: 80, rb: 0, clock: 0 });
        }
    }
}

/// Gunpowder: very explosive, ignites with fire/lava/lightning, chain reaction.
/// / 火药：极易爆炸，被火焰/岩浆/闪电点燃，连锁反应。
pub fn update_gunpowder(cell: Cell, mut api: SandApi) {
    let fluid = api.get_fluid();
    let (sx, sy) = api.rand_vec();
    let nbr = api.get(sx, sy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava || nbr.species == Species::Lightning {
        api.set(0, 0, Cell { species: Species::Fire, ra: 250, rb: 0, clock: 0 });
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 120, density: 80 });
        // Chain reaction spreads fire in 3 random directions / 连锁反应向 3 个随机方向扩散火焰
        for _ in 0..3 {
            let (cdx, cdy) = api.rand_vec();
            api.set(cdx, cdy, Cell { species: Species::Fire, ra: 150, rb: 0, clock: 0 });
        }
        return;
    }
    if fluid.pressure > 80 {
        api.set(0, 0, Cell { species: Species::Fire, ra: 200, rb: 0, clock: 0 });
        return;
    }
    powder_fall(cell, &mut api, true, true);
}

/// Flour: flammable when airborne (dust explosion hazard), mixes with water to form mud.
/// / 面粉：在空气中可燃（粉尘爆炸危险），与水混合形成泥浆。
pub fn update_flour(cell: Cell, mut api: SandApi) {
    let (sx, sy) = api.rand_vec();
    let nbr = api.get(sx, sy);
    if nbr.species == Species::Fire && api.once_in(5) {
        api.set(0, 0, Cell { species: Species::Fire, ra: 180, rb: 0, clock: 0 });
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 90, density: 40 });
        return;
    }
    // Absorbs water, becomes paste-like (mud) / 吸收水，变成糊状（泥浆）
    let below = api.get(0, 1);
    if below.species == Species::Water {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, Cell { species: Species::Mud, ra: cell.ra, rb: 0, clock: 0 });
        return;
    }
    powder_fall(cell, &mut api, true, false);
}

/// Sugar: dissolves in water, caramelizes (burns) near heat.
/// / 糖：溶于水，在热源附近焦化（燃烧）。
pub fn update_sugar(cell: Cell, mut api: SandApi) {
    let (sx, sy) = api.rand_vec();
    let nbr = api.get(sx, sy);
    if nbr.species == Species::Water {
        // Sugar dissolves completely in water / 糖在水中完全溶解
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Fire, ra: 100, rb: 0, clock: 0 });
        return;
    }
    powder_fall(cell, &mut api, true, false);
}

/// Salt: dissolves in water, melts ice, non-flammable.
/// / 盐：溶于水，融化冰，不可燃。
pub fn update_salt(cell: Cell, mut api: SandApi) {
    let (sx, sy) = api.rand_vec();
    let nbr = api.get(sx, sy);
    if nbr.species == Species::Water && api.once_in(8) {
        api.set(0, 0, EMPTY_CELL); // dissolves / 溶解
        return;
    }
    // Salt lowers the freezing point of water, melting ice / 盐降低水的冰点，融化冰
    if nbr.species == Species::Ice && api.once_in(10) {
        api.set(sx, sy, Cell { species: Species::Water, ra: 100, rb: 0, clock: 0 });
    }
    powder_fall(cell, &mut api, false, false);
}

/// Pepper: irritant powder, floats on water.
/// / 胡椒：刺激性粉末，浮在水面上。
pub fn update_pepper(cell: Cell, mut api: SandApi) {
    let below = api.get(0, 1);
    if below.species == Species::Water {
        // Pepper floats on water / 胡椒浮在水面上
        api.set(0, 0, cell);
    } else {
        powder_fall(cell, &mut api, true, false);
    }
}

/// Ash: very light residue from burning, drifts easily, already burnt.
/// / 灰烬：燃烧后极轻的残留物，容易飘动，已经烧过。
pub fn update_ash(cell: Cell, mut api: SandApi) {
    let dx = api.rand_dir();
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if api.get(dx, 0).species == Species::Empty && api.once_in(2) {
        // Very light, drifts sideways often / 极轻，频繁侧向飘动
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else {
        api.set(0, 0, cell);
    }
}

/// Soot: sticky light powder, clings to surfaces.
/// / 烟灰：粘性轻粉末，附着在表面上。
pub fn update_soot(cell: Cell, mut api: SandApi) {
    let dx = api.rand_dir();
    let below = api.get(0, 1);
    if below.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if api.get(dx, 0).species == Species::Empty && api.once_in(3) {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else {
        // Sticks to surfaces instead of falling / 粘在表面上而非下落
        api.set(0, 0, cell);
    }
}

/// Charcoal: burns slowly, glows, produces heat over time.
/// / 木炭：缓慢燃烧，发光，持续产生热量。
/// rb = burn timer: 0=unignited, >0=burning slowly / rb = 燃烧计时器：0=未点燃, >0=缓慢燃烧中
pub fn update_charcoal(cell: Cell, mut api: SandApi) {
    let (sx, sy) = api.rand_vec();
    let nbr = api.get(sx, sy);
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        if api.once_in(30) {
            api.set(0, 0, Cell { species: Species::Fire, ra: 60, rb: 0, clock: 0 });
        } else {
            api.set(0, 0, cell);
        }
        return;
    }
    // Burns slowly when ignited, radiating gradual heat / 点燃后缓慢燃烧，辐射逐渐的热量
    if cell.rb > 0 {
        api.set(0, 0, Cell { rb: cell.rb.saturating_sub(1), ..cell });
        if cell.rb % 5 == 0 {
            api.set_fluid(Wind { dx: 0, dy: 5, pressure: 5, density: 30 });
        }
        return;
    }
    powder_fall(cell, &mut api, true, false);
}

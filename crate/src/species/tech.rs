//! Technology element simulation: wire, circuit, battery, solar cell, laser, LED.
//! / 科技元素模拟：电线、电路、电池、太阳能电池、激光、LED。
//!
//! Tech elements form an energy-based subsystem with power generation, conduction, and emission.
//! / 科技元素形成了一个基于能量的子系统，包含发电、传导和发射功能。

use crate::{Cell, SandApi, Wind, EMPTY_CELL};
use super::Species;

/// Wire: conducts energy through itself, transmitting to opposite side.
/// / 电线：通过自身传导能量，传输到对侧。
pub fn update_wire(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Energy || nbr.species == Species::Lightning {
        // Conduct energy to other side
        api.set(-dx, -dy, Cell { species: Species::Energy, ra: nbr.ra, rb: 0, clock: 0 });
    }
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Lava, ra: 80, rb: 0, clock: 0 });
        return;
    }
}

/// Circuit: processes energy input and outputs amplified signal, shorts out in water/acid.
/// / 电路：处理能量输入并输出放大信号，遇水/酸短路。
pub fn update_circuit(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Energy {
        // Process and output amplified signal
        let energy_level = nbr.ra;
        let (odx, ody) = api.rand_vec_8();
        if api.get(odx, ody).species == Species::Empty {
            api.set(odx, ody, Cell { species: Species::Energy, ra: energy_level.saturating_mul(2).min(250), rb: 0, clock: 0 });
        }
        api.set(dx, dy, EMPTY_CELL);
    }
    if nbr.species == Species::Water || nbr.species == Species::Acid {
        api.set(0, 0, Cell { species: Species::Fire, ra: 30, rb: 0, clock: 0 });
    }
}

/// Battery: stores energy from nearby energy particles, releases to empty spaces, slowly self-discharges.
/// / 电池：从附近能量粒子中储存能量，向空白处释放，缓慢自放电。
pub fn update_battery(cell: Cell, mut api: SandApi) {
    let ra = cell.ra; // charge level
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Energy && ra < 250 {
        // Charge from energy
        api.set(0, 0, Cell { ra: (ra + 20).min(250), ..cell });
    }
    if ra > 50 && api.get(dx, dy).species == Species::Empty && api.once_in(10) {
        // Release energy
        api.set(dx, dy, Cell { species: Species::Energy, ra: 50, rb: 0, clock: 0 });
        api.set(0, 0, Cell { ra: ra - 30, ..cell });
    }
    if nbr.species == Species::Fire || nbr.species == Species::Lava {
        api.set(0, 0, Cell { species: Species::Fire, ra: 200, rb: 0, clock: 0 });
        return;
    }
    // Slowly discharge
    if ra > 0 && api.once_in(100) {
        api.set(0, 0, Cell { ra: ra - 1, ..cell });
    }
}

/// Solar cell: generates energy from light (higher on screen = more sun exposure).
/// / 太阳能电池：从光中产生能量（屏幕越靠上 = 越多的阳光照射）。
pub fn update_solar_cell(cell: Cell, mut api: SandApi) {
    let ra = cell.ra;
    // Higher y position = more sun
    let sun_exposure = 255 - (api.y as i32 * 255 / api.universe.height) as u8;
    if sun_exposure > 150 && api.once_in(20) && ra < 200 {
        api.set(0, 0, Cell { ra: (ra + 10).min(200), ..cell });
    }
    // Output energy downward
    if ra > 30 && api.get(0, 1).species == Species::Empty && api.once_in(8) {
        api.set(0, 1, Cell { species: Species::Energy, ra: 30, rb: 0, clock: 0 });
        api.set(0, 0, Cell { ra: ra.saturating_sub(20), ..cell });
    }
}

/// Laser: shoots a beam in encoded direction, burns through materials, reflected by mirrors.
/// / 激光：按编码方向发射光束，烧穿材料，被镜子反射。
pub fn update_laser(cell: Cell, mut api: SandApi) {
    let rb = cell.rb; // direction stored in rb
    let dir = if rb < 4 { 0 } else { ((rb - 4) % 8) as i32 };
    let (dx, dy) = match dir {
        0 => (0, -1), 1 => (1, -1), 2 => (1, 0), 3 => (1, 1),
        4 => (0, 1), 5 => (-1, 1), 6 => (-1, 0), _ => (-1, -1),
    };
    let target = api.get(dx, dy);
    if target.species == Species::Empty {
        api.set(dx, dy, Cell { species: Species::Energy, ra: 200, rb: 0, clock: 0 });
    } else if target.species != Species::Wall && target.species != Species::Mirror {
        // Burn through
        api.set(dx, dy, Cell { species: Species::Fire, ra: 150, rb: 0, clock: 0 });
    }
    // Short lived
    let ra = cell.ra;
    if ra < 5 {
        api.set(0, 0, EMPTY_CELL);
    } else {
        api.set(0, 0, Cell { ra: ra - 2, ..cell });
    }
}

/// LED: lights up (emits density 200 via fluid) when powered by energy or battery.
/// / LED：当由能量或电池供电时亮起（通过流体发射密度 200）。
pub fn update_led(cell: Cell, mut api: SandApi) {
    let (dx, dy) = api.rand_vec();
    let nbr = api.get(dx, dy);
    if nbr.species == Species::Energy || nbr.species == Species::Battery {
        // Light up!
        api.set_fluid(Wind { dx: 0, dy: 0, pressure: 0, density: 200 });
    }
}

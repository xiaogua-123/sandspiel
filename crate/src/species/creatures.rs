//! Creature simulation: ant, spider, bee, butterfly, fish, bird, snake, worm.
//! / 生物模拟：蚂蚁、蜘蛛、蜜蜂、蝴蝶、鱼、鸟、蛇、蠕虫。
//!
//! Creatures are mobile agents that move, eat specific foods, avoid danger, and have starvation.
//! / 生物是可以移动的代理，会移动、吃特定食物、避开危险，并且会挨饿。

use crate::{Cell, SandApi, EMPTY_CELL};
use super::Species;

/// Simple creature that moves around, eats certain things, fears fire/lava/poison/acid.
/// / 简单的生物，四处移动，吃特定食物，害怕火/岩浆/毒药/酸。
/// ra = health/energy (decreases over time); rb = danger-flee timer.
/// / ra = 生命值/能量（随时间减少）；rb = 危险逃跑计时器。
fn simple_creature(cell: Cell, api: &mut SandApi, favors_rise: bool) {
    let mut ra = cell.ra;
    let mut rb = cell.rb;

    // Starvation: die when health runs out / 饥饿：生命值耗尽时死亡
    if ra < 5 {
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    ra = ra.saturating_sub(1); // passively lose energy / 被动消耗能量

    // Movement: flee behavior when rb > 0 (danger timer) / 移动：rb > 0 时的逃跑行为（危险计时器）
    if rb > 0 {
        rb = rb.saturating_sub(1);
        let dx = (ra % 5) as i32 - 2;
        let dy = if favors_rise { -1 } else { 1 }; // some fly upward / 有些向上飞
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

    // Look for food or random move / 寻找食物或随机移动
    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);

    // Flee from danger / 逃离危险
    if sample.species == Species::Fire || sample.species == Species::Lava
        || sample.species == Species::Poison || sample.species == Species::Acid {
        rb = 20; // flee timer / 逃跑计时器
        ra = ra.saturating_sub(10); // take damage / 受伤
        api.set(0, 0, Cell { ra, rb, ..cell });
        return;
    }

    // Move into empty space / 移动到空白处
    if sample.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(sx, sy, Cell { ra, rb, ..cell });
    } else {
        api.set(0, 0, Cell { ra, rb, ..cell });
    }
}

/// Ant: eats sweets and organic matter, follows other ants (trail behavior).
/// / 蚂蚁：吃甜食和有机物，跟随其他蚂蚁（跟踪行为）。
pub fn update_ant(cell: Cell, mut api: SandApi) {
    let ra = cell.ra;
    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);

    // Eat sweet/organic foods / 吃甜食/有机食物
    if matches!(sample.species, Species::Sugar | Species::Honey | Species::Bread
        | Species::Fruit | Species::Fungus | Species::Leaf | Species::Rice | Species::Wheat) {
        api.set(sx, sy, EMPTY_CELL);
        let new_ra = (ra + 30).min(200);
        api.set(0, 0, Cell { ra: new_ra, rb: cell.rb, ..cell });
        return;
    }

    // Ant trails: follow other ants / 蚂蚁路径：跟随其他蚂蚁
    if sample.species == Species::Ant && api.once_in(5) {
        api.set(sx, sy, cell);
        api.set(0, 0, EMPTY_CELL);
        return;
    }

    simple_creature(cell, &mut api, false);
}

/// Spider: predator of small insects, climbs on walls and wood.
/// / 蜘蛛：捕食小型昆虫，在墙壁和木头上攀爬。
pub fn update_spider(cell: Cell, mut api: SandApi) {
    let ra = cell.ra;
    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);

    // Eat small insects / 吃小型昆虫
    if matches!(sample.species, Species::Ant | Species::Bee | Species::Butterfly
        | Species::Worm | Species::Mite) {
        api.set(sx, sy, EMPTY_CELL);
        let new_ra = (ra + 50).min(200);
        api.set(0, 0, Cell { ra: new_ra, rb: cell.rb, ..cell });
        return;
    }

    // Climb on walls/wood / 在墙壁/木头上攀爬
    let climb_target = api.get(0, -1);
    if climb_target.species == Species::Wall || climb_target.species == Species::Wood {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, -1, cell);
        return;
    }

    simple_creature(cell, &mut api, true);
}

/// Bee: pollinates flowers, flies upward, feeds on flowers.
/// / 蜜蜂：为花朵授粉，向上飞行，以花为食。
pub fn update_bee(cell: Cell, mut api: SandApi) {
    let ra = cell.ra;
    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);

    // Pollinate flowers for energy / 通过授粉获得能量
    if sample.species == Species::Flower {
        let new_ra = (ra + 20).min(200);
        api.set(0, 0, Cell { ra: new_ra, rb: cell.rb, ..cell });
        return;
    }

    // Bee flies upward / 蜜蜂向上飞行
    if api.get(0, -1).species == Species::Empty && api.once_in(2) {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, -1, cell);
        return;
    }

    simple_creature(cell, &mut api, true);
}

/// Butterfly: erratic flight pattern, pollinates flowers.
/// / 蝴蝶：不规则的飞行模式，为花朵授粉。
pub fn update_butterfly(cell: Cell, mut api: SandApi) {
    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);
    if sample.species == Species::Flower {
        let new_ra = (cell.ra + 15).min(200);
        api.set(0, 0, Cell { ra: new_ra, rb: cell.rb, ..cell });
        return;
    }

    // Erratic random flight pattern / 不规则的随机飞行模式
    let (rdx, rdy) = api.rand_vec();
    let target = api.get(rdx, rdy);
    if target.species == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(rdx, rdy, cell);
    } else {
        simple_creature(cell, &mut api, true);
    }
}

/// Fish: aquatic creature, moves through water, suffocates out of water.
/// / 鱼：水生生物，在水中移动，离开水会窒息死亡。
pub fn update_fish(cell: Cell, mut api: SandApi) {
    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);

    // Eat seeds and aquatic plants / 吃种子和水生植物
    if matches!(sample.species, Species::Seed | Species::Plant | Species::Grass | Species::Moss) {
        api.set(sx, sy, EMPTY_CELL);
        let new_ra = (cell.ra + 20).min(200);
        api.set(0, 0, Cell { ra: new_ra, rb: cell.rb, ..cell });
        return;
    }

    // Suffocates out of water - check for adjacent water / 离开水窒息 - 检查相邻的水
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

    // Move through water, swapping positions / 穿过水移动，交换位置
    if api.get(sx, sy).species == Species::Water {
        api.set(0, 0, Cell { species: Species::Water, ra: 100, rb: 0, clock: 0 });
        api.set(sx, sy, cell);
    } else {
        simple_creature(cell, &mut api, false);
    }
}

/// Bird: flies, eats seeds, insects, and fruit.
/// / 鸟：飞行，吃种子、昆虫和水果。
pub fn update_bird(cell: Cell, mut api: SandApi) {
    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);

    // Eat seeds, insects, fruit / 吃种子、昆虫、水果
    if matches!(sample.species, Species::Seed | Species::Ant | Species::Bee
        | Species::Butterfly | Species::Worm | Species::Fruit | Species::Rice | Species::Wheat) {
        api.set(sx, sy, EMPTY_CELL);
        let new_ra = (cell.ra + 40).min(200);
        api.set(0, 0, Cell { ra: new_ra, rb: cell.rb, ..cell });
        return;
    }

    // Fly upward with lateral drift / 向上飞行并侧向漂移
    let bdx = api.rand_dir();
    if api.get(bdx, -1).species == Species::Empty && api.once_in(2) {
        api.set(0, 0, EMPTY_CELL);
        let bdx = api.rand_dir();
        api.set(bdx, -1, cell);
        return;
    }

    simple_creature(cell, &mut api, true);
}

/// Snake: predator of small creatures, ground-dwelling.
/// / 蛇：捕食小型生物，栖息在地面上。
pub fn update_snake(cell: Cell, mut api: SandApi) {
    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);

    // Eat small creatures / 吃小型生物
    if matches!(sample.species, Species::Mite | Species::Ant | Species::Spider
        | Species::Fish | Species::Worm | Species::Egg) {
        api.set(sx, sy, EMPTY_CELL);
        let new_ra = (cell.ra + 60).min(250);
        api.set(0, 0, Cell { ra: new_ra, rb: cell.rb, ..cell });
        return;
    }

    simple_creature(cell, &mut api, false);
}

/// Worm: lives in soil, eats decaying matter, burrows and aerates.
/// / 蠕虫：生活在土壤中，吃腐烂物，挖掘和松动土壤。
pub fn update_worm(cell: Cell, mut api: SandApi) {
    let (sx, sy) = api.rand_vec();
    let sample = api.get(sx, sy);

    // Eat soil, peat, ash, leaves / 吃土壤、泥炭、灰烬、叶子
    if matches!(sample.species, Species::Soil | Species::Peat
        | Species::Ash | Species::Leaf) {
        api.set(sx, sy, EMPTY_CELL);
        let new_ra = (cell.ra + 15).min(200);
        api.set(0, 0, Cell { ra: new_ra, rb: cell.rb, ..cell });
        return;
    }

    // Burrow into soil / 钻入土壤
    if sample.species == Species::Soil && api.once_in(5) {
        api.set(sx, sy, cell);
        api.set(0, 0, EMPTY_CELL);
        return;
    }

    simple_creature(cell, &mut api, false);
}

//! Species enum and dispatch module for the falling-sand simulation.
//! / 沙盘模拟的物种枚举和分派模块。
//!
//! Defines all 136 element types and maps each to its update function.
//! / 定义了全部 136 种元素类型，并将每种类型映射到其更新函数。

use wasm_bindgen::prelude::*;

use crate::{Cell, SandApi};

// Original simulation elements / 原始模拟元素
mod falling;
mod gas;
mod liquid;
mod new;
mod organic;
mod special;

// Extended element categories / 扩展元素类别
mod metals;
mod crystals;
mod powders;
mod liquids2;
mod gases2;
mod organics2;
mod creatures;
mod explosives;
mod construction;
mod magical;
mod food;
mod nature;
mod tech;
mod misc;

/// All possible element types in the simulation (136 total).
/// / 模拟中所有可能的元素类型（总计 136 种）。
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Species {
    // --- Base Elements / 基础元素 (0-23) ---
    Empty = 0,   // void / 空
    Wall = 1,    // indestructible barrier / 不可破坏的屏障
    Sand = 2,    // granular falling solid / 粒状下落固体
    Water = 3,   // flowing liquid / 流动液体
    Gas = 4,     // generic rising gas / 通用上升气体
    Cloner = 5,  // copies nearby elements / 复制附近的元素
    Fire = 6,    // spreading flame / 扩散火焰
    Wood = 7,    // flammable solid / 可燃固体
    Lava = 8,    // hot liquid rock / 炽热液态岩石
    Ice = 9,     // frozen water / 冰冻的水
    Snow = 10,   // light powder, melts / 轻粉末，会融化
    Plant = 11,  // growing organic / 生长的有机物
    Acid = 12,   // corrosive liquid / 腐蚀性液体
    Stone = 13,  // heavy inert solid / 重型惰性固体
    Dust = 14,   // fine explosive powder / 细小爆炸性粉末
    Mite = 15,   // tiny creature / 微小生物
    Oil = 16,    // flammable liquid / 可燃液体
    Rocket = 17, // directed projectile / 定向发射物
    Fungus = 18, // spreading organic / 扩散真菌
    Seed = 19,   // grows into plants / 长成植物
    Sponge = 20, // absorbs liquids / 吸收液体
    Slime = 21,  // bouncy sticky goo / 弹性粘稠物
    Glass = 22,  // transparent solid / 透明固体
    Coral = 23,  // underwater growth / 水下生长物
    // --- Metals / 金属 (24-33) ---
    Iron = 24,
    Copper = 25,
    Gold = 26,
    Silver = 27,
    Aluminum = 28,
    Lead = 29,
    Zinc = 30,
    Tin = 31,
    Bronze = 32,
    Steel = 33,
    // --- Crystals & Gems / 晶体和宝石 (34-41) ---
    Diamond = 34,
    Ruby = 35,
    Sapphire = 36,
    Emerald = 37,
    Amethyst = 38,
    Quartz = 39,
    Crystal = 40,
    Obsidian = 41,
    // --- Powders / 粉末 (42-49) ---
    Gunpowder = 42,
    Flour = 43,
    Sugar = 44,
    Salt = 45,
    Pepper = 46,
    Ash = 47,
    Soot = 48,
    Charcoal = 49,
    // --- More Liquids / 更多液体 (50-57) ---
    Mud = 50,
    Blood = 51,
    Honey = 52,
    Milk = 53,
    Poison = 54,
    Mercury = 55,
    Alcohol = 56,
    Syrup = 57,
    // --- More Gases / 更多气体 (58-65) ---
    Steam = 58,
    Smoke = 59,
    Helium = 60,
    Chlorine = 61,
    Oxygen = 62,
    Hydrogen = 63,
    PlasmaGas = 64,
    Methane = 65,
    // --- More Organics / 更多有机物 (66-75) ---
    Leaf = 66,
    Flower = 67,
    Grass = 68,
    Vine = 69,
    Moss = 70,
    Mushroom = 71,
    Bark = 72,
    Root = 73,
    Fruit = 74,
    Thorn = 75,
    // --- Small Creatures / 小型生物 (76-83) ---
    Ant = 76,
    Spider = 77,
    Bee = 78,
    Butterfly = 79,
    Fish = 80,
    Bird = 81,
    Snake = 82,
    Worm = 83,
    // --- Explosives & Hazards / 爆炸物和危险品 (84-91) ---
    TNT = 84,
    Bomb = 85,
    Nitro = 86,
    Plutonium = 87,
    Uranium = 88,
    C4 = 89,
    Thermite = 90,
    Napalm = 91,
    // --- Construction Materials / 建筑材料 (92-99) ---
    Brick = 92,
    Concrete = 93,
    Cement = 94,
    Tile = 95,
    Plaster = 96,
    Marble = 97,
    Granite = 98,
    Basalt = 99,
    // --- Magical & Special / 魔法和特殊物品 (100-109) ---
    Portal = 100,
    Teleporter = 101,
    Antigravity = 102,
    Magnet = 103,
    Lightning = 104,
    Void = 105,
    Chaos = 106,
    Energy = 107,
    Shield = 108,
    Mirror = 109,
    // --- Food / 食物 (110-115) ---
    Bread = 110,
    Cheese = 111,
    Meat = 112,
    Egg = 113,
    Rice = 114,
    Wheat = 115,
    // --- Nature / 自然物质 (116-123) ---
    Clay = 116,
    Soil = 117,
    Peat = 118,
    Limestone = 119,
    Chalk = 120,
    Shale = 121,
    Slate = 122,
    Sandstone = 123,
    // --- Tech / 科技 (124-129) ---
    Wire = 124,
    Circuit = 125,
    Battery = 126,
    SolarCell = 127,
    Laser = 128,
    LED = 129,
    // --- Misc & Toys / 杂项和玩具 (130-135) ---
    Bubble = 130,
    Balloon = 131,
    Confetti = 132,
    Glitter = 133,
    Spring = 134,
    Domino = 135,
}

impl Species {
    /// Dispatch the species-specific update function based on the enum variant.
    /// / 根据枚举变体分派物种特定的更新函数。
    pub fn update(&self, cell: Cell, api: SandApi) {
        match self {
            Species::Empty => {}
            Species::Wall => {}
            Species::Sand => falling::update_sand(cell, api),
            Species::Dust => falling::update_dust(cell, api),
            Species::Stone => falling::update_stone(cell, api),
            Species::Water => liquid::update_water(cell, api),
            Species::Oil => liquid::update_oil(cell, api),
            Species::Lava => liquid::update_lava(cell, api),
            Species::Acid => liquid::update_acid(cell, api),
            Species::Gas => gas::update_gas(cell, api),
            Species::Fire => gas::update_fire(cell, api),
            Species::Wood => organic::update_wood(cell, api),
            Species::Plant => organic::update_plant(cell, api),
            Species::Fungus => organic::update_fungus(cell, api),
            Species::Seed => organic::update_seed(cell, api),
            Species::Mite => organic::update_mite(cell, api),
            Species::Ice => special::update_ice(cell, api),
            Species::Cloner => special::update_cloner(cell, api),
            Species::Rocket => special::update_rocket(cell, api),
            Species::Snow => new::update_snow(cell, api),
            Species::Sponge => new::update_sponge(cell, api),
            Species::Slime => new::update_slime(cell, api),
            Species::Glass => new::update_glass(cell, api),
            Species::Coral => new::update_coral(cell, api),
            // Metals
            Species::Iron => metals::update_iron(cell, api),
            Species::Copper => metals::update_copper(cell, api),
            Species::Gold => metals::update_gold(cell, api),
            Species::Silver => metals::update_silver(cell, api),
            Species::Aluminum => metals::update_aluminum(cell, api),
            Species::Lead => metals::update_lead(cell, api),
            Species::Zinc => metals::update_zinc(cell, api),
            Species::Tin => metals::update_tin(cell, api),
            Species::Bronze => metals::update_bronze(cell, api),
            Species::Steel => metals::update_steel(cell, api),
            // Crystals
            Species::Diamond => crystals::update_diamond(cell, api),
            Species::Ruby => crystals::update_ruby(cell, api),
            Species::Sapphire => crystals::update_sapphire(cell, api),
            Species::Emerald => crystals::update_emerald(cell, api),
            Species::Amethyst => crystals::update_amethyst(cell, api),
            Species::Quartz => crystals::update_quartz(cell, api),
            Species::Crystal => crystals::update_crystal(cell, api),
            Species::Obsidian => crystals::update_obsidian(cell, api),
            // Powders
            Species::Gunpowder => powders::update_gunpowder(cell, api),
            Species::Flour => powders::update_flour(cell, api),
            Species::Sugar => powders::update_sugar(cell, api),
            Species::Salt => powders::update_salt(cell, api),
            Species::Pepper => powders::update_pepper(cell, api),
            Species::Ash => powders::update_ash(cell, api),
            Species::Soot => powders::update_soot(cell, api),
            Species::Charcoal => powders::update_charcoal(cell, api),
            // More Liquids
            Species::Mud => liquids2::update_mud(cell, api),
            Species::Blood => liquids2::update_blood(cell, api),
            Species::Honey => liquids2::update_honey(cell, api),
            Species::Milk => liquids2::update_milk(cell, api),
            Species::Poison => liquids2::update_poison(cell, api),
            Species::Mercury => liquids2::update_mercury(cell, api),
            Species::Alcohol => liquids2::update_alcohol(cell, api),
            Species::Syrup => liquids2::update_syrup(cell, api),
            // More Gases
            Species::Steam => gases2::update_steam(cell, api),
            Species::Smoke => gases2::update_smoke(cell, api),
            Species::Helium => gases2::update_helium(cell, api),
            Species::Chlorine => gases2::update_chlorine(cell, api),
            Species::Oxygen => gases2::update_oxygen(cell, api),
            Species::Hydrogen => gases2::update_hydrogen(cell, api),
            Species::PlasmaGas => gases2::update_plasma(cell, api),
            Species::Methane => gases2::update_methane(cell, api),
            // More Organics
            Species::Leaf => organics2::update_leaf(cell, api),
            Species::Flower => organics2::update_flower(cell, api),
            Species::Grass => organics2::update_grass(cell, api),
            Species::Vine => organics2::update_vine(cell, api),
            Species::Moss => organics2::update_moss(cell, api),
            Species::Mushroom => organics2::update_mushroom(cell, api),
            Species::Bark => organics2::update_bark(cell, api),
            Species::Root => organics2::update_root(cell, api),
            Species::Fruit => organics2::update_fruit(cell, api),
            Species::Thorn => organics2::update_thorn(cell, api),
            // Creatures
            Species::Ant => creatures::update_ant(cell, api),
            Species::Spider => creatures::update_spider(cell, api),
            Species::Bee => creatures::update_bee(cell, api),
            Species::Butterfly => creatures::update_butterfly(cell, api),
            Species::Fish => creatures::update_fish(cell, api),
            Species::Bird => creatures::update_bird(cell, api),
            Species::Snake => creatures::update_snake(cell, api),
            Species::Worm => creatures::update_worm(cell, api),
            // Explosives
            Species::TNT => explosives::update_tnt(cell, api),
            Species::Bomb => explosives::update_bomb(cell, api),
            Species::Nitro => explosives::update_nitro(cell, api),
            Species::Plutonium => explosives::update_plutonium(cell, api),
            Species::Uranium => explosives::update_uranium(cell, api),
            Species::C4 => explosives::update_c4(cell, api),
            Species::Thermite => explosives::update_thermite(cell, api),
            Species::Napalm => explosives::update_napalm(cell, api),
            // Construction
            Species::Brick => construction::update_brick(cell, api),
            Species::Concrete => construction::update_concrete(cell, api),
            Species::Cement => construction::update_cement(cell, api),
            Species::Tile => construction::update_tile(cell, api),
            Species::Plaster => construction::update_plaster(cell, api),
            Species::Marble => construction::update_marble(cell, api),
            Species::Granite => construction::update_granite(cell, api),
            Species::Basalt => construction::update_basalt(cell, api),
            // Magical
            Species::Portal => magical::update_portal(cell, api),
            Species::Teleporter => magical::update_teleporter(cell, api),
            Species::Antigravity => magical::update_antigravity(cell, api),
            Species::Magnet => magical::update_magnet(cell, api),
            Species::Lightning => magical::update_lightning(cell, api),
            Species::Void => magical::update_void(cell, api),
            Species::Chaos => magical::update_chaos(cell, api),
            Species::Energy => magical::update_energy(cell, api),
            Species::Shield => magical::update_shield(cell, api),
            Species::Mirror => magical::update_mirror(cell, api),
            // Food
            Species::Bread => food::update_bread(cell, api),
            Species::Cheese => food::update_cheese(cell, api),
            Species::Meat => food::update_meat(cell, api),
            Species::Egg => food::update_egg(cell, api),
            Species::Rice => food::update_rice(cell, api),
            Species::Wheat => food::update_wheat(cell, api),
            // Nature
            Species::Clay => nature::update_clay(cell, api),
            Species::Soil => nature::update_soil(cell, api),
            Species::Peat => nature::update_peat(cell, api),
            Species::Limestone => nature::update_limestone(cell, api),
            Species::Chalk => nature::update_chalk(cell, api),
            Species::Shale => nature::update_shale(cell, api),
            Species::Slate => nature::update_slate(cell, api),
            Species::Sandstone => nature::update_sandstone(cell, api),
            // Tech
            Species::Wire => tech::update_wire(cell, api),
            Species::Circuit => tech::update_circuit(cell, api),
            Species::Battery => tech::update_battery(cell, api),
            Species::SolarCell => tech::update_solar_cell(cell, api),
            Species::Laser => tech::update_laser(cell, api),
            Species::LED => tech::update_led(cell, api),
            // Misc
            Species::Bubble => misc::update_bubble(cell, api),
            Species::Balloon => misc::update_balloon(cell, api),
            Species::Confetti => misc::update_confetti(cell, api),
            Species::Glitter => misc::update_glitter(cell, api),
            Species::Spring => misc::update_spring(cell, api),
            Species::Domino => misc::update_domino(cell, api),
        }
    }
}

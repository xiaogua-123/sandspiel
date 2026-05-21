extern crate cfg_if;
extern crate js_sys;
extern crate rand;
extern crate rand_xoshiro;
extern crate wasm_bindgen;
extern crate web_sys;

mod species;
mod utils;

use rand::{Rng, SeedableRng};
use rand_xoshiro::SplitMix64;
use species::Species;
use std::collections::VecDeque;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Wind {
    dx: u8,
    dy: u8,
    pressure: u8,
    density: u8,
}

#[wasm_bindgen]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    species: Species,
    ra: u8,
    rb: u8,
    clock: u8,
}

impl Cell {
    pub fn new(species: Species) -> Cell {
        Cell {
            species,
            ra: 100 + (js_sys::Math::random() * 50.) as u8,
            rb: 0,
            clock: 0,
        }
    }
    pub fn update(&self, api: SandApi) {
        self.species.update(*self, api);
    }
}

pub static EMPTY_CELL: Cell = Cell {
    species: Species::Empty,
    ra: 0,
    rb: 0,
    clock: 0,
};

/// Wind resistance threshold per species (index by Species as u8)
const WIND_THRESHOLD: [u8; 136] = [
    200, // Empty = 0  (never moved by wind)
    200, // Wall = 1
    30,  // Sand = 2
    40,  // Water = 3
    5,   // Gas = 4
    200, // Cloner = 5
    5,   // Fire = 6
    70,  // Wood = 7
    60,  // Lava = 8
    60,  // Ice = 9
    15,  // Snow = 10 (light, easily blown)
    60,  // Plant = 11
    40,  // Acid = 12
    70,  // Stone = 13
    10,  // Dust = 14
    30,  // Mite = 15
    50,  // Oil = 16
    30,  // Rocket = 17
    54,  // Fungus = 18
    35,  // Seed = 19
    200, // Sponge = 20 (stationary)
    60,  // Slime = 21
    70,  // Glass = 22 (heavy)
    200, // Coral = 23 (stationary)
    // Metals (24-33) - heavy, hard to blow
    80,  // Iron = 24
    75,  // Copper = 25
    90,  // Gold = 26
    85,  // Silver = 27
    50,  // Aluminum = 28
    90,  // Lead = 29
    70,  // Zinc = 30
    65,  // Tin = 31
    85,  // Bronze = 32
    95,  // Steel = 33
    // Crystals (34-41) - hard, mostly stationary
    80,  // Diamond = 34
    80,  // Ruby = 35
    80,  // Sapphire = 36
    75,  // Emerald = 37
    70,  // Amethyst = 38
    70,  // Quartz = 39
    65,  // Crystal = 40
    85,  // Obsidian = 41
    // Powders (42-49) - very light
    8,   // Gunpowder = 42
    10,  // Flour = 43
    12,  // Sugar = 44
    15,  // Salt = 45
    10,  // Pepper = 46
    5,   // Ash = 47
    5,   // Soot = 48
    12,  // Charcoal = 49
    // Liquids (50-57)
    45,  // Mud = 50
    40,  // Blood = 51
    55,  // Honey = 52
    45,  // Milk = 53
    40,  // Poison = 54
    70,  // Mercury = 55
    35,  // Alcohol = 56
    55,  // Syrup = 57
    // Gases (58-65) - very light
    3,   // Steam = 58
    4,   // Smoke = 59
    2,   // Helium = 60
    5,   // Chlorine = 61
    5,   // Oxygen = 62
    3,   // Hydrogen = 63
    3,   // PlasmaGas = 64
    4,   // Methane = 65
    // Organics (66-75)
    15,  // Leaf = 66
    20,  // Flower = 67
    25,  // Grass = 68
    40,  // Vine = 69
    200, // Moss = 70 (stationary)
    60,  // Mushroom = 71
    70,  // Bark = 72
    70,  // Root = 73
    50,  // Fruit = 74
    60,  // Thorn = 75
    // Creatures (76-83)
    20,  // Ant = 76
    25,  // Spider = 77
    15,  // Bee = 78
    10,  // Butterfly = 79
    30,  // Fish = 80
    10,  // Bird = 81
    35,  // Snake = 82
    50,  // Worm = 83
    // Explosives (84-91)
    60,  // TNT = 84
    55,  // Bomb = 85
    40,  // Nitro = 86
    90,  // Plutonium = 87
    90,  // Uranium = 88
    65,  // C4 = 89
    70,  // Thermite = 90
    55,  // Napalm = 91
    // Construction (92-99)
    80,  // Brick = 92
    90,  // Concrete = 93
    60,  // Cement = 94
    75,  // Tile = 95
    70,  // Plaster = 96
    85,  // Marble = 97
    90,  // Granite = 98
    95,  // Basalt = 99
    // Magical (100-109)
    200, // Portal = 100
    200, // Teleporter = 101
    10,  // Antigravity = 102
    200, // Magnet = 103
    5,   // Lightning = 104
    200, // Void = 105
    5,   // Chaos = 106
    3,   // Energy = 107
    200, // Shield = 108
    200, // Mirror = 109
    // Food (110-115)
    50,  // Bread = 110
    60,  // Cheese = 111
    55,  // Meat = 112
    40,  // Egg = 113
    10,  // Rice = 114
    25,  // Wheat = 115
    // Nature (116-123)
    55,  // Clay = 116
    35,  // Soil = 117
    40,  // Peat = 118
    75,  // Limestone = 119
    40,  // Chalk = 120
    70,  // Shale = 121
    75,  // Slate = 122
    65,  // Sandstone = 123
    // Tech (124-129)
    200, // Wire = 124
    200, // Circuit = 125
    200, // Battery = 126
    200, // SolarCell = 127
    200, // Laser = 128
    200, // LED = 129
    // Misc (130-135)
    5,   // Bubble = 130
    8,   // Balloon = 131
    3,   // Confetti = 132
    5,   // Glitter = 133
    200, // Spring = 134
    70,  // Domino = 135
];

#[wasm_bindgen]
pub struct Universe {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
    undo_stack: VecDeque<Vec<Cell>>,
    winds: Vec<Wind>,
    burns: Vec<Wind>,
    generation: u8,
    rng: SplitMix64,
}

pub struct SandApi<'a> {
    x: i32,
    y: i32,
    universe: &'a mut Universe,
}

impl<'a> SandApi<'a> {
    pub fn get(&mut self, dx: i32, dy: i32) -> Cell {
        if dx > 2 || dx < -2 || dy > 2 || dy < -2 {
            panic!("oob get");
        }
        let nx = self.x + dx;
        let ny = self.y + dy;
        if nx < 0 || nx > self.universe.width - 1 || ny < 0 || ny > self.universe.height - 1 {
            return Cell {
                species: Species::Wall,
                ra: 0,
                rb: 0,
                clock: self.universe.generation,
            };
        }
        self.universe.get_cell(nx, ny)
    }
    pub fn set(&mut self, dx: i32, dy: i32, v: Cell) {
        if dx > 2 || dx < -2 || dy > 2 || dy < -2 {
            panic!("oob set");
        }
        let nx = self.x + dx;
        let ny = self.y + dy;
        if nx < 0 || nx > self.universe.width - 1 || ny < 0 || ny > self.universe.height - 1 {
            return;
        }
        let i = self.universe.get_index(nx, ny);
        self.universe.cells[i] = v;
        self.universe.cells[i].clock = self.universe.generation.wrapping_add(1);
    }
    pub fn get_fluid(&mut self) -> Wind {
        let idx = self.universe.get_index(self.x, self.y);
        self.universe.winds[idx]
    }
    pub fn set_fluid(&mut self, v: Wind) {
        let idx = self.universe.get_index(self.x, self.y);
        self.universe.burns[idx] = v;
    }

    pub fn rand_int(&mut self, n: i32) -> i32 {
        self.universe.rng.gen_range(0..n)
    }
    pub fn once_in(&mut self, n: i32) -> bool {
        self.rand_int(n) == 0
    }
    pub fn rand_dir(&mut self) -> i32 {
        (self.rand_int(1000) % 3) - 1
    }
    pub fn rand_dir_2(&mut self) -> i32 {
        if (self.rand_int(1000) % 2) == 0 { -1 } else { 1 }
    }

    pub fn rand_vec(&mut self) -> (i32, i32) {
        match self.rand_int(2000) % 9 {
            0 => (1, 1),
            1 => (1, 0),
            2 => (1, -1),
            3 => (0, -1),
            4 => (-1, -1),
            5 => (-1, 0),
            6 => (-1, 1),
            7 => (0, 1),
            _ => (0, 0),
        }
    }

    pub fn rand_vec_8(&mut self) -> (i32, i32) {
        match self.rand_int(8) {
            0 => (1, 1),
            1 => (1, 0),
            2 => (1, -1),
            3 => (0, -1),
            4 => (-1, -1),
            5 => (-1, 0),
            6 => (-1, 1),
            _ => (0, 1),
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn reset(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let idx = self.get_index(x, y);
                self.cells[idx] = EMPTY_CELL;
            }
        }
    }

    pub fn tick(&mut self) {
        self.generation = self.generation.wrapping_add(1);

        // Wind pass
        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.get_cell(x, y);
                let wind = self.get_wind(x, y);
                Self::blow_wind(cell, wind, SandApi { universe: self, x, y });
            }
        }

        // Physics update pass
        self.generation = self.generation.wrapping_add(1);
        let scan_reverse = self.generation % 2 == 0;
        for x in 0..self.width {
            let scanx = if scan_reverse {
                self.width - (1 + x)
            } else {
                x
            };
            for y in 0..self.height {
                let idx = self.get_index(scanx, y);
                let cell = self.get_cell(scanx, y);
                self.burns[idx] = Wind { dx: 0, dy: 0, pressure: 0, density: 0 };
                Self::update_cell(cell, SandApi { universe: self, x: scanx, y });
            }
        }
    }

    pub fn width(&self) -> i32 { self.width }
    pub fn height(&self) -> i32 { self.height }
    pub fn cells(&self) -> *const Cell { self.cells.as_ptr() }
    pub fn winds(&self) -> *const Wind { self.winds.as_ptr() }
    pub fn burns(&self) -> *const Wind { self.burns.as_ptr() }

    pub fn paint(&mut self, x: i32, y: i32, size: i32, species: Species) {
        let radius = (size as f64) / 2.0;
        let floor = (radius + 1.0) as i32;
        let ciel = (radius + 1.5) as i32;

        for dx in -floor..ciel {
            for dy in -floor..ciel {
                if ((dx * dx + dy * dy) as f64) > (radius * radius) {
                    continue;
                }
                let px = x + dx;
                let py = y + dy;
                if px < 0 || px > self.width - 1 || py < 0 || py > self.height - 1 {
                    continue;
                }
                let i = self.get_index(px, py);
                if self.get_cell(px, py).species == Species::Empty || species == Species::Empty {
                    self.cells[i] = Cell {
                        species,
                        ra: 60 + (size as u8) + (self.rng.gen::<f32>() * 30.) as u8
                            + ((self.generation % 127) as i8 - 60).abs() as u8,
                        rb: 0,
                        clock: self.generation,
                    }
                }
            }
        }
    }

    pub fn push_undo(&mut self) {
        self.undo_stack.push_front(self.cells.clone());
        self.undo_stack.truncate(50);
    }

    pub fn pop_undo(&mut self) {
        if let Some(state) = self.undo_stack.pop_front() {
            self.cells = state;
        }
    }

    pub fn flush_undos(&mut self) {
        self.undo_stack.clear();
    }

    pub fn new(width: i32, height: i32) -> Universe {
        let cells = (0..width * height).map(|_| EMPTY_CELL).collect();
        let zero_wind = Wind { dx: 0, dy: 0, pressure: 0, density: 0 };
        let winds = vec![zero_wind; (width * height) as usize];
        let burns = vec![zero_wind; (width * height) as usize];
        let rng = SeedableRng::seed_from_u64(0x734f6b89de5f83cc);
        Universe {
            width,
            height,
            cells,
            undo_stack: VecDeque::with_capacity(50),
            burns,
            winds,
            generation: 0,
            rng,
        }
    }
}

// Private methods
impl Universe {
    fn get_index(&self, x: i32, y: i32) -> usize {
        (x * self.height + y) as usize
    }

    fn get_cell(&self, x: i32, y: i32) -> Cell {
        self.cells[self.get_index(x, y)]
    }

    fn get_wind(&self, x: i32, y: i32) -> Wind {
        self.winds[self.get_index(x, y)]
    }

    fn blow_wind(cell: Cell, wind: Wind, mut api: SandApi) {
        if cell.clock.wrapping_sub(api.universe.generation) == 1 {
            return;
        }
        if cell.species == Species::Empty {
            return;
        }

        let threshold = WIND_THRESHOLD[cell.species as usize] as i32;
        let wx = (wind.dy as i32) - 126;
        let wy = (wind.dx as i32) - 126;

        let dx = if wx > threshold { 1 } else if wx < -threshold { -1 } else { 0 };
        let dy = if wy > threshold { 1 } else if wy < -threshold { -1 } else { 0 };

        if (dx != 0 || dy != 0) && api.get(dx, dy).species == Species::Empty {
            api.set(0, 0, EMPTY_CELL);
            let final_dy = if dy == -1
                && api.get(dx, -2).species == Species::Empty
                && matches!(cell.species,
                    Species::Sand | Species::Water | Species::Lava
                    | Species::Acid | Species::Mite | Species::Dust
                    | Species::Oil | Species::Rocket)
            {
                -2
            } else {
                dy
            };
            api.set(dx, final_dy, cell);
        }
    }

    fn update_cell(cell: Cell, api: SandApi) {
        if cell.clock.wrapping_sub(api.universe.generation) == 1 {
            return;
        }
        cell.update(api);
    }
}

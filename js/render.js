// WebGL rendering with regl / 使用regl进行WebGL渲染
// Manages HSV color table, sand texture, snapshot export, and color palette generation / 管理HSV颜色表、沙子纹理、截图导出和调色板生成

const reglBuilder = require("regl");
import * as wasm from "../crate/pkg/sandtable_bg.wasm";
import { Species, Universe } from "../crate/pkg/sandtable";
const memory = wasm.memory;

let fsh = require("./glsl/sand.glsl");
let vsh = require("./glsl/sandVertex.glsl");

// HSV color definitions for all 136 element types [Hue, Saturation, Lightness]
// Each value encoded as byte (0-255), decoded in shader as /255.0
const SPECIES_HSV_BYTES = new Uint8Array([
  0,0,20,     // 0: Empty (nearly invisible)
  0,5,100,    // 1: Wall (neutral gray)
  30,220,170, // 2: Sand (warm golden)
  150,200,160,// 3: Water (clear blue)
  0,5,230,    // 4: Gas (faint white)
  195,180,170,// 5: Cloner (violet-purple)
  15,255,200, // 6: Fire (bright warm orange)
  22,190,80,  // 7: Wood (rich brown)
  10,255,220, // 8: Lava (bright red-orange glow)
  145,140,230,// 9: Ice (icy light blue)
  0,10,250,   // 10: Snow (bright white)
  80,210,100, // 11: Plant (vivid green)
  52,240,185, // 12: Acid (bright yellow-green)
  0,10,130,   // 13: Stone (neutral gray)
  28,120,160, // 14: Dust (warm tan)
  215,140,185,// 15: Mite (pinkish)
  18,190,35,  // 16: Oil (dark brown-black)
  0,235,205,  // 17: Rocket (bright red)
  195,130,145,// 18: Fungus (purple-gray)
  26,160,105, // 19: Seed (brown)
  45,150,205, // 20: Sponge (warm yellow)
  75,210,150, // 21: Slime (vivid green)
  150,35,225, // 22: Glass (transparent blue-white)
  5,170,185,  // 23: Coral (warm pink)
  // Metals 24-33
  10,35,95,   // 24: Iron (dark metallic gray)
  18,210,140, // 25: Copper (warm copper-orange)
  38,240,170, // 26: Gold (rich gold)
  0,8,195,    // 27: Silver (bright silver)
  0,10,180,   // 28: Aluminum (light silver)
  155,30,82,  // 29: Lead (dark blue-gray)
  145,25,155, // 30: Zinc (silvery blue-gray)
  0,15,170,   // 31: Tin (soft silver-white)
  22,210,120, // 32: Bronze (warm bronze)
  150,28,105, // 33: Steel (dark blue-gray metallic)
  // Crystals 34-41
  0,5,245,    // 34: Diamond (brilliant white sparkle)
  0,255,135,  // 35: Ruby (deep blood red)
  168,225,135,// 36: Sapphire (deep royal blue)
  100,230,105,// 37: Emerald (deep green)
  200,185,155,// 38: Amethyst (rich purple)
  40,10,235,  // 39: Quartz (milky white)
  150,55,205, // 40: Crystal (light blue crystal)
  185,35,32,  // 41: Obsidian (dark violet-black)
  // Powders 42-49
  0,15,55,    // 42: Gunpowder (dark charcoal)
  38,35,245,  // 43: Flour (creamy off-white)
  35,18,245,  // 44: Sugar (sparkly white)
  0,5,245,    // 45: Salt (pure white)
  18,85,42,   // 46: Pepper (dark brown-black)
  0,10,162,   // 47: Ash (medium gray)
  0,8,22,     // 48: Soot (near black)
  0,18,38,    // 49: Charcoal (dark gray-black)
  // Liquids 50-57
  22,135,72,  // 50: Mud (wet brown)
  0,255,82,   // 51: Blood (deep red)
  36,235,172, // 52: Honey (warm amber gold)
  32,12,250,  // 53: Milk (creamy white)
  62,250,142, // 54: Poison (toxic green)
  0,8,182,    // 55: Mercury (liquid silver)
  0,5,242,    // 56: Alcohol (clear)
  22,200,125, // 57: Syrup (thick amber)
  // Gases 58-65
  0,8,245,    // 58: Steam (white vapor)
  0,13,95,    // 59: Smoke (medium gray)
  0,5,250,    // 60: Helium (barely visible)
  58,210,182, // 61: Chlorine (yellow-green toxic)
  150,35,225, // 62: Oxygen (pale blue)
  0,5,250,    // 63: Hydrogen (barely visible)
  205,255,232,// 64: PlasmaGas (bright violet glow)
  28,65,182,  // 65: Methane (pale brown)
  // Organics 66-75
  82,215,112, // 66: Leaf (vibrant leaf green)
  210,230,210,// 67: Flower (bright pink-magenta)
  78,210,92,  // 68: Grass (fresh grass green)
  88,175,82,  // 69: Vine (darker vine green)
  70,155,72,  // 70: Moss (deep moss green)
  24,110,152, // 71: Mushroom (warm tan)
  20,125,72,  // 72: Bark (dark tree bark)
  22,105,60,  // 73: Root (earthy brown)
  8,235,162,  // 74: Fruit (bright red)
  18,110,82,  // 75: Thorn (dark woody)
  // Creatures 76-83
  15,35,30,   // 76: Ant (dark brown-black)
  0,15,22,    // 77: Spider (deep black)
  44,235,170, // 78: Bee (bright yellow-black)
  210,215,188,// 79: Butterfly (vivid purple-pink)
  145,155,162,// 80: Fish (silver-blue)
  5,165,155,  // 81: Bird (warm red)
  80,175,102, // 82: Snake (olive green)
  12,72,135,  // 83: Worm (pink-brown)
  // Explosives 84-91
  0,240,125,  // 84: TNT (bold red)
  0,25,58,    // 85: Bomb (dark iron gray)
  48,255,185, // 86: Nitro (bright yellow)
  78,255,142, // 87: Plutonium (radioactive green glow)
  52,240,162, // 88: Uranium (yellow-green glow)
  38,35,182,  // 89: C4 (pale tan)
  14,255,205, // 90: Thermite (intense orange)
  18,240,172, // 91: Napalm (orange-red gel)
  // Construction 92-99
  8,190,112,  // 92: Brick (warm red-brown)
  0,8,162,    // 93: Concrete (cool gray)
  35,12,182,  // 94: Cement (light warm gray)
  12,135,155, // 95: Tile (terracotta)
  30,8,232,   // 96: Plaster (warm white)
  35,5,222,   // 97: Marble (elegant white)
  15,18,142,  // 98: Granite (speckled gray)
  0,18,52,    // 99: Basalt (dark volcanic)
  // Magical 100-109
  195,255,155,// 100: Portal (deep purple swirl)
  172,210,182,// 101: Teleporter (electric blue)
  148,185,205,// 102: Antigravity (cyan-blue)
  0,165,135,  // 103: Magnet (red pole)
  46,255,245, // 104: Lightning (brilliant yellow-white)
  0,0,5,      // 105: Void (absolute black)
  0,255,182,  // 106: Chaos (shifting rainbow base)
  148,255,225,// 107: Energy (bright cyan-blue plasma)
  155,112,205,// 108: Shield (translucent blue)
  0,5,222,    // 109: Mirror (pure silver reflective)
  // Food 110-115
  28,195,162, // 110: Bread (golden brown crust)
  44,220,202, // 111: Cheese (rich yellow)
  2,225,125,  // 112: Meat (fresh red-pink)
  42,85,225,  // 113: Egg (pale yellow)
  35,10,242,  // 114: Rice (white)
  36,210,172, // 115: Wheat (golden amber)
  // Nature 116-123
  12,140,135, // 116: Clay (warm red-brown)
  24,125,72,  // 117: Soil (dark rich earth)
  23,105,42,  // 118: Peat (dark organic)
  34,30,195,  // 119: Limestone (pale warm gray)
  36,8,245,   // 120: Chalk (bright white)
  18,42,92,   // 121: Shale (dark layered gray)
  158,22,85,  // 122: Slate (blue-gray)
  28,135,152, // 123: Sandstone (warm tan)
  // Tech 124-129
  18,215,142, // 124: Wire (copper orange)
  82,190,112, // 125: Circuit (PCB green)
  50,210,155, // 126: Battery (yellow-gold)
  170,185,102,// 127: SolarCell (dark blue)
  0,255,255,  // 128: Laser (pure bright red)
  120,255,225,// 129: LED (bright cyan-green)
  // Misc 130-135
  155,55,232, // 130: Bubble (iridescent light blue)
  220,225,210,// 131: Balloon (bright pink-red)
  52,250,210, // 132: Confetti (bright yellow)
  40,255,225, // 133: Glitter (golden sparkle)
  0,12,162,   // 134: Spring (metallic gray)
  0,10,95,    // 135: Domino (dark with white dots)
]);

// Total number of element species in the game / 游戏中元素种类的总数
const NUM_SPECIES = 136;

// Initialize WebGL renderer with regl / 初始化WebGL渲染器
// Sets up data texture from WASM memory and HSV color table as 1D texture / 从WASM内存设置数据纹理，HSV颜色表作为1D纹理上传
let startWebGL = ({ canvas, universe, isSnapshot = false }) => {
  const regl = reglBuilder({
    canvas,
    attributes: { preserveDrawingBuffer: isSnapshot },
  });
  const width = universe.width();
  const height = universe.height();
  // Direct WASM memory view of cell grid / WASM内存中细胞网格的直接视图
  let cell_pointer = universe.cells();
  let cells = new Uint8Array(memory.buffer, cell_pointer, width * height * 4);
  const dataTexture = regl.texture({ width, height, data: cells });

  // Upload HSV color table as a 1D texture (136 pixels wide) / 将HSV颜色表作为1D纹理上传（136像素宽）
  const hsvTexture = regl.texture({
    width: NUM_SPECIES,
    height: 1,
    data: SPECIES_HSV_BYTES,
    format: "rgb",
    type: "uint8",
  });

  let drawSand = regl({
    frag: fsh,
    uniforms: {
      t: ({ tick }) => tick,
      data: () => {
        cell_pointer = universe.cells();
        cells = new Uint8Array(memory.buffer, cell_pointer, width * height * 4);
        return dataTexture({ width, height, data: cells });
      },
      colorTable: hsvTexture,
      resolution: ({ viewportWidth, viewportHeight }) => [
        viewportWidth,
        viewportHeight,
      ],
      // DPI multiplier for crisp grid rendering / DPI乘数用于清晰网格渲染
      dpi: window.devicePixelRatio * 2,
      isSnapshot,
    },

    vert: vsh,
    attributes: {
      // Large triangle covering entire viewport for full-screen quad / 覆盖整个视口的大三角形，用于全屏四边形
      position: [
        [-1, 4],
        [-1, -1],
        [4, -1],
      ],
    },
    count: 3,
  });

  return () => {
    regl.poll();
    drawSand();
  };
};

// Capture a PNG snapshot of the current universe / 截取当前宇宙的PNG截图
let snapshot = (universe) => {
  let canvas = document.createElement("canvas");
  canvas.width = universe.width() / 2;
  canvas.height = universe.height() / 2;
  let render = startWebGL({ universe, canvas, isSnapshot: true });
  render();

  return canvas.toDataURL("image/png");
};

// Generate CSS color map for each species via GPU rendering / 通过GPU渲染为每个元素生成CSS颜色映射
let pallette = () => {
  let canvas = document.createElement("canvas");

  let species = Object.values(Species).filter((x) => Number.isInteger(x));
  let range = Math.max(...species) + 1;
  let universe = Universe.new(range, 1);
  canvas.width = 3;
  canvas.height = range;
  universe.reset();

  species.forEach((id) => universe.paint(id, 0, 1, id));

  let render = startWebGL({ universe, canvas, isSnapshot: true });
  render();
  let ctx = canvas.getContext("webgl");
  let data = new Uint8Array(range * 4);
  ctx.readPixels(0, 0, 1, range, ctx.RGBA, ctx.UNSIGNED_BYTE, data);
  let colors = {};
  species.forEach((id) => {
    let index = (range - 1 - id) * 4;
    let color = `rgba(${data[index]},${data[index + 1]}, ${
      data[index + 2]
    }, 0.25)`;
    colors[id] = color;
  });
  return colors;
};

export { startWebGL, snapshot, pallette };

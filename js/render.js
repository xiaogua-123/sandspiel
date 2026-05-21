const reglBuilder = require("regl");
import * as wasm from "../crate/pkg/sandtable_bg.wasm";
import { Species, Universe } from "../crate/pkg/sandtable";
const memory = wasm.memory;

let fsh = require("./glsl/sand.glsl");
let vsh = require("./glsl/sandVertex.glsl");

// HSV color definitions for all 136 element types [Hue, Saturation, Lightness]
// Each value encoded as byte (0-255), decoded in shader as /255.0
const SPECIES_HSV_BYTES = new Uint8Array([
  0,25,25, // 0: Empty
  25,25,102, // 1: Wall
  25,127,153, // 2: Sand
  153,127,179, // 3: Water
  0,51,179, // 4: Gas
  229,76,127, // 5: Cloner
  13,179,179, // 6: Fire
  25,76,76, // 7: Wood
  20,204,179, // 8: Lava
  153,102,179, // 9: Ice
  229,102,255, // 10: Snow
  102,102,102, // 11: Plant
  46,229,204, // 12: Acid
  179,25,51, // 13: Stone (hue offset -0.4=0.6*255=153, but base is -0.4 which wraps)
  0,102,204, // 14: Dust
  204,229,204, // 15: Mite
  25,51,76, // 16: Oil
  0,102,229, // 17: Rocket
  20,25,204, // 18: Fungus
  229,179,179, // 19: Seed
  31,76,127, // 20: Sponge
  76,179,127, // 21: Slime
  148,25,217, // 22: Glass
  13,179,153, // 23: Coral
  // Metals 24-33
  20,38,115, // 24: Iron
  25,153,127, // 25: Copper
  36,204,140, // 26: Gold
  153,13,179, // 27: Silver
  153,20,153, // 28: Aluminum
  166,25,76, // 29: Lead
  31,51,127, // 30: Zinc
  25,38,140, // 31: Tin
  28,179,102, // 32: Bronze
  153,13,89, // 33: Steel
  // Crystals 34-41
  153,0,229, // 34: Diamond
  0,229,153, // 35: Ruby
  153,204,127, // 36: Sapphire
  76,204,127, // 37: Emerald
  204,153,153, // 38: Amethyst
  20,25,204, // 39: Quartz
  140,51,191, // 40: Crystal
  0,13,38, // 41: Obsidian
  // Powders 42-49
  31,25,51, // 42: Gunpowder
  33,25,217, // 43: Flour
  31,13,229, // 44: Sugar
  0,0,217, // 45: Salt
  20,51,64, // 46: Pepper
  25,13,102, // 47: Ash
  0,13,38, // 48: Soot
  20,25,38, // 49: Charcoal
  // Liquids 50-57
  31,102,64, // 50: Mud
  0,229,76, // 51: Blood
  36,204,153, // 52: Honey
  31,13,229, // 53: Milk
  89,229,127, // 54: Poison
  153,13,153, // 55: Mercury
  153,25,204, // 56: Alcohol
  25,179,115, // 57: Syrup
  // Gases 58-65
  153,13,229, // 58: Steam
  20,13,89, // 59: Smoke
  153,5,242, // 60: Helium
  64,127,153, // 61: Chlorine
  140,25,204, // 62: Oxygen
  153,5,242, // 63: Hydrogen
  179,229,229, // 64: PlasmaGas
  38,51,179, // 65: Methane
  // Organics 66-75
  64,179,102, // 66: Leaf
  0,204,179, // 67: Flower
  76,179,89, // 68: Grass
  71,153,76, // 69: Vine
  76,127,76, // 70: Moss
  25,76,140, // 71: Mushroom
  25,102,64, // 72: Bark
  25,76,51, // 73: Root
  13,204,127, // 74: Fruit
  20,76,76, // 75: Thorn
  // Creatures 76-83
  25,127,51, // 76: Ant
  0,25,38, // 77: Spider
  36,204,127, // 78: Bee
  0,204,153, // 79: Butterfly
  140,153,127, // 80: Fish
  20,127,115, // 81: Bird
  76,153,89, // 82: Snake
  20,102,102, // 83: Worm
  // Explosives 84-91
  31,179,102, // 84: TNT
  18,76,51, // 85: Bomb
  33,102,153, // 86: Nitro
  102,204,102, // 87: Plutonium
  89,179,89, // 88: Uranium
  31,76,127, // 89: C4
  20,179,179, // 90: Thermite
  20,204,127, // 91: Napalm
  // Construction 92-99
  13,153,102, // 92: Brick
  25,13,127, // 93: Concrete
  31,20,140, // 94: Cement
  20,127,127, // 95: Tile
  25,13,204, // 96: Plaster
  153,8,191, // 97: Marble
  20,25,115, // 98: Granite
  153,13,46, // 99: Basalt
  // Magical 100-109
  0,229,153, // 100: Portal
  204,204,127, // 101: Teleporter
  140,179,153, // 102: Antigravity
  153,102,102, // 103: Magnet
  38,127,229, // 104: Lightning
  204,127,13, // 105: Void
  0,229,153, // 106: Chaos
  140,229,204, // 107: Energy
  140,153,127, // 108: Shield
  153,8,217, // 109: Mirror
  // Food 110-115
  31,127,140, // 110: Bread
  36,179,153, // 111: Cheese
  8,179,102, // 112: Meat
  33,76,191, // 113: Egg
  36,51,204, // 114: Rice
  36,179,127, // 115: Wheat
  // Nature 116-123
  20,102,115, // 116: Clay
  25,102,64, // 117: Soil
  23,127,38, // 118: Peat
  31,20,166, // 119: Limestone
  31,8,217, // 120: Chalk
  25,25,76, // 121: Shale
  153,20,64, // 122: Slate
  31,76,140, // 123: Sandstone
  // Tech 124-129
  20,127,102, // 124: Wire
  153,102,76, // 125: Circuit
  38,153,115, // 126: Battery
  140,153,102, // 127: SolarCell
  0,0,255, // 128: Laser
  0,229,204, // 129: LED
  // Misc 130-135
  153,25,204, // 130: Bubble
  0,204,179, // 131: Balloon
  0,229,191, // 132: Confetti
  0,127,204, // 133: Glitter
  31,102,140, // 134: Spring
  18,127,127, // 135: Domino
]);

const NUM_SPECIES = 136;

let startWebGL = ({ canvas, universe, isSnapshot = false }) => {
  const regl = reglBuilder({
    canvas,
    attributes: { preserveDrawingBuffer: isSnapshot },
  });
  const width = universe.width();
  const height = universe.height();
  let cell_pointer = universe.cells();
  let cells = new Uint8Array(memory.buffer, cell_pointer, width * height * 4);
  const dataTexture = regl.texture({ width, height, data: cells });

  // Upload HSV color table as a 1D texture (136 pixels wide)
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
      dpi: window.devicePixelRatio * 2,
      isSnapshot,
    },

    vert: vsh,
    attributes: {
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

let snapshot = (universe) => {
  let canvas = document.createElement("canvas");
  canvas.width = universe.width() / 2;
  canvas.height = universe.height() / 2;
  let render = startWebGL({ universe, canvas, isSnapshot: true });
  render();

  return canvas.toDataURL("image/png");
};

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

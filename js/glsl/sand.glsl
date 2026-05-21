precision highp float;
uniform float t;
uniform float dpi;
uniform vec2 resolution;
uniform bool isSnapshot;
uniform sampler2D backBuffer;
uniform sampler2D data;
uniform sampler2D colorTable;

varying vec2 uv;

#pragma glslify: hsv2rgb = require('glsl-hsv2rgb')
#pragma glslify: snoise3 = require(glsl-noise/simplex/3d)
#pragma glslify: snoise2 = require(glsl-noise/simplex/2d)
#pragma glslify: random = require(glsl-random)

void main() {
  vec3 color;
  float a = 1.0;

  vec2 textCoord = ((uv * vec2(0.5, -0.5)) + vec2(0.5)).yx;

  vec4 data = texture2D(data, textCoord);
  int type = int((data.r * 255.) + 0.1);

  // Sample base HSV from color lookup table
  float texX = (float(type) + 0.5) / 136.0;
  vec3 hsv = texture2D(colorTable, vec2(texX, 0.5)).rgb;
  float hue = hsv.r;
  float saturation = hsv.s;
  float lightness = hsv.b;

  // Pre-compute noise values
  float n3 = snoise3(vec3(floor(uv * resolution / dpi), t * 0.05));
  float n2 = snoise2(floor(uv * resolution / dpi));
  float nFast = snoise3(vec3(floor(uv * resolution / dpi), t * 0.12));
  float sparkle = snoise3(vec3(floor(uv * resolution / (dpi * 0.5)), t * 0.03));

  // ── Default lightness for most solid types ──
  lightness = lightness + data.g * 0.45;

  // ── Empty (0) ──
  if (type == 0) {
    a = 0.1;
    if (isSnapshot) {
      saturation = 0.05;
      lightness = 1.01;
      a = 1.0;
    }
  }

  // ── Fire-like: Fire(6), Lava(8), PlasmaGas(64), Energy(107) ──
  else if (type == 6 || type == 8 || type == 64 || type == 107) {
    hue = hsv.r + data.g * 0.12 + nFast * 0.08;
    saturation = 0.8 + nFast * 0.2;
    lightness = hsv.b + data.g * 0.3 + nFast * 0.25;
    if (type == 107) { lightness += 0.12; } // Energy extra bright
    if (isSnapshot && type == 6) { lightness -= 0.2; }
  }

  // ── Water-like flowing liquids: Water(3), Blood(51), Alcohol(56), Poison(54) ──
  else if (type == 3 || type == 51 || type == 56 || type == 54) {
    int polarity = int(mod(data.g * 255.0, 2.0) + 0.1);
    float flow = n2 * 0.08 + data.g * 0.2;
    lightness = hsv.b + flow;
    if (polarity == 0) { lightness += 0.015; }
    if (type == 3) { lightness += nFast * 0.06; }
    if (type == 51) { saturation = 0.85 + n2 * 0.15; } // Blood rich color
  }

  // ── Viscous liquids: Mud(50), Honey(52), Milk(53), Mercury(55), Syrup(57) ──
  else if (type == 50 || type == 52 || type == 53 || type == 55 || type == 57) {
    lightness = hsv.b + data.g * 0.15 + n2 * 0.04;
    if (type == 55) { lightness += nFast * 0.08; saturation = 0.12 + n2 * 0.04; } // Mercury specular
    if (type == 52) { lightness += 0.04; } // Honey thick
  }

  // ── Gases: Gas(4), Steam(58), Smoke(59), Helium(60), Chlorine(61), Oxygen(62), Hydrogen(63), Methane(65) ──
  else if (type == 4 || (type >= 58 && type <= 65)) {
    lightness = hsv.b + nFast * 0.18;
    saturation = hsv.s + n2 * 0.1;
    if (type == 4) { saturation += data.b * 0.6; }
    if (type == 59) { lightness -= 0.05; } // Smoke darker
    if (type == 58) { lightness += 0.05; }  // Steam brighter
    if (type == 64) { hue += nFast * 0.04; lightness += 0.06; } // Plasma shimmer
  }

  // ── Wood (7) ──
  else if (type == 7) {
    hue = hsv.r + data.g * 0.08;
    lightness = hsv.b + data.g * 0.3 + n2 * 0.04;
  }

  // ── Ice (9) ──
  else if (type == 9) {
    lightness = hsv.b + data.g * 0.35 + nFast * 0.06;
    saturation = hsv.s + n2 * 0.08;
  }

  // ── Acid (12) ──
  else if (type == 12) {
    lightness = hsv.b + data.g * 0.2 + nFast * 0.08;
    saturation = hsv.s + n2 * 0.1;
  }

  // ── Stone-like solids: Stone(13), Concrete(93), Cement(94), Brick(92), Basalt(99) ──
  else if (type == 13 || type == 92 || type == 93 || type == 94 || type == 99) {
    lightness = hsv.b + data.g * 0.2 + n2 * 0.03;
  }

  // ── Dust (14) ── floating particles ──
  else if (type == 14) {
    hue = hsv.r + data.g * 0.5 + t * 0.0003;
    lightness = hsv.b + data.g * 0.25 + nFast * 0.1;
  }

  // ── Mite (15) ──
  else if (type == 15) {
    lightness = hsv.b + data.g * 0.3 + nFast * 0.06;
  }

  // ── Oil (16) ── iridescent ──
  else if (type == 16) {
    hue = hsv.r + data.g * 2.5 + t * 0.005;
    lightness = hsv.b + data.g * 0.2 + n2 * 0.05;
    saturation = 0.25 + nFast * 0.1;
  }

  // ── Rocket (17) ──
  else if (type == 17) {
    saturation = hsv.s + data.b * 0.6;
    lightness = hsv.b + data.g * 0.3 + abs(nFast) * 0.15;
  }

  // ── Fungus (18) ──
  else if (type == 18) {
    hue = hsv.r + data.g * 0.1 - 0.05;
    lightness = hsv.b + data.g * 0.25 + n2 * 0.04;
    saturation = hsv.s + data.g * 0.2;
  }

  // ── Seed (19) ──
  else if (type == 19) {
    hue = hsv.r + fract(fract(data.b * 2.0) * 0.4) - 0.15;
    lightness = hsv.b + data.g * 0.35;
    saturation = hsv.s + data.b * 0.1;
  }

  // ── Sponge (20) ──
  else if (type == 20) {
    lightness = hsv.b + data.g * 0.3 + n2 * 0.05;
  }

  // ── Slime (21) ──
  else if (type == 21) {
    lightness = hsv.b + data.g * 0.2 + nFast * 0.07;
  }

  // ── Glass (22) ── transparent, refractive look ──
  else if (type == 22) {
    lightness = hsv.b + data.g * 0.1 + abs(nFast) * 0.06;
    saturation = hsv.s + n2 * 0.05;
  }

  // ── Coral (23) ──
  else if (type == 23) {
    hue = hsv.r + data.g * 0.04;
    lightness = hsv.b + data.g * 0.2 + n2 * 0.03;
  }

  // ── Metals (24-33): specular-like variation ──
  else if (type >= 24 && type <= 33) {
    lightness = hsv.b + data.g * 0.3 + nFast * 0.05;
    if (type == 26) { lightness += 0.04; } // Gold extra shine
    if (type == 27 || type == 28 || type == 31) { lightness += 0.03; } // Silver metals
  }

  // ── Crystals (34-41): sparkle effect ──
  else if (type >= 34 && type <= 41) {
    lightness = hsv.b + data.g * 0.25 + sparkle * 0.15;
    if (type == 34) { lightness += abs(sparkle) * 0.1; } // Diamond sparkle
    if (type == 41) { saturation += nFast * 0.05; } // Obsidian subtle
  }

  // ── Powders (42-49): grainy texture ──
  else if (type >= 42 && type <= 49) {
    lightness = hsv.b + data.g * 0.3 + n2 * 0.07;
    if (type == 42) { lightness -= 0.02; } // Gunpowder dark
    if (type == 43 || type == 44 || type == 45) { lightness += 0.05; } // White powders brighter
  }

  // ── Flower (67): colorful petals ──
  else if (type == 67) {
    hue = fract(data.b * 1.8) * 0.45 + hsv.r;
    lightness = hsv.b + data.g * 0.2;
    saturation = 0.75 + data.b * 0.2;
  }

  // ── Butterfly (79): colorful wings ──
  else if (type == 79) {
    hue = fract(data.b * 2.5);
    lightness = hsv.b + data.g * 0.25;
    saturation = 0.7 + data.b * 0.2;
  }

  // ── Portal (100): swirling vortex ──
  else if (type == 100) {
    hue = fract(t * 0.008 + data.g * 0.15);
    lightness = hsv.b + data.g * 0.3 + abs(nFast) * 0.12;
    saturation = 0.8 + nFast * 0.2;
  }

  // ── Lightning (104): bright flash ──
  else if (type == 104) {
    lightness = hsv.b + data.g * 0.4 + abs(nFast) * 0.3;
    saturation = 0.2 + nFast * 0.15;
  }

  // ── Void (105): absolute darkness ──
  else if (type == 105) {
    lightness = hsv.b * 0.3 + data.g * 0.05;
    saturation = 0.05;
  }

  // ── Chaos (106): constantly shifting ──
  else if (type == 106) {
    hue = fract(t * 0.015 + data.g * 0.4);
    lightness = hsv.b + data.g * 0.25 + nFast * 0.15;
    saturation = 0.75 + n2 * 0.2;
  }

  // ── LED (129): bright glowing ──
  else if (type == 129) {
    hue = fract(data.g * 0.4);
    lightness = hsv.b + data.g * 0.25 + abs(nFast) * 0.1;
  }

  // ── Bubble (130): iridescent transparent ──
  else if (type == 130) {
    hue = hsv.r + nFast * 0.08;
    lightness = hsv.b + data.g * 0.15 + abs(nFast) * 0.08;
    saturation = hsv.s + n2 * 0.05;
  }

  // ── Balloon (131): colorful ──
  else if (type == 131) {
    hue = fract(data.b * 0.6) + hsv.r;
    lightness = hsv.b + data.g * 0.2;
  }

  // ── Confetti (132): bright festive ──
  else if (type == 132) {
    hue = fract(data.b * 2.0);
    lightness = hsv.b + data.g * 0.2 + abs(nFast) * 0.05;
    saturation = 0.75 + data.b * 0.2;
  }

  // ── Glitter (133): sparkle ──
  else if (type == 133) {
    hue = fract(data.g * 1.5 + t * 0.0008);
    lightness = hsv.b + data.g * 0.2 + abs(sparkle) * 0.2;
    saturation = 0.7 + sparkle * 0.3;
  }

  // ── All other types: basic noise variation (organics, food, nature, tech, creatures, misc) ──
  else {
    lightness = hsv.b + data.g * 0.4 + n2 * 0.03;
  }

  // ── Global subtle noise (non-snapshot) ──
  if (isSnapshot == false) {
    lightness *= (0.975 + n2 * 0.025);
  }

  color = hsv2rgb(vec3(hue, saturation, lightness));
  gl_FragColor = vec4(color, a);
}

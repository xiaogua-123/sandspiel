precision highp float;
uniform float t;
uniform float dpi;
uniform vec2 resolution;
uniform bool isSnapshot;
uniform sampler2D backBuffer;
uniform sampler2D data;

varying vec2 uv;

// clang-format off
#pragma glslify: hsv2rgb = require('glsl-hsv2rgb')
#pragma glslify: snoise3 = require(glsl-noise/simplex/3d)
#pragma glslify: snoise2 = require(glsl-noise/simplex/2d)
#pragma glslify: random = require(glsl-random)

// clang-format on

void main() {
  vec3 color;
  //   float r = abs(sin(t / 25.));
  //   if (length(uv) < r && length(uv) > r - 0.1) {
  // color = hsv2rgb(vec3(sin(t * 0.01), 0.5, 0.5));

  vec2 textCoord = ((uv * vec2(0.5, -0.5)) + vec2(0.5)).yx;
  // vec3 bb = texture2D(backBuffer, (uv * 0.5) + vec2(0.5)).rgb;

  vec4 data = texture2D(data, textCoord);
  int type = int((data.r * 255.) + 0.1);
  float hue = 0.0;
  float saturation = 0.6;
  float lightness = 0.3 + data.g * 0.5;
  float noise = snoise3(vec3(floor(uv * resolution / dpi), t * 0.05));
  float a = 1.0;

  if (type == 0) {
    hue = 0.0;
    saturation = 0.1;
    lightness = 0.1;
    a = 0.1;
    if (isSnapshot) {
      saturation = 0.05;
      lightness = 1.01;
      a = 1.0;
    }
  } else if (type == 1) {
    hue = 0.1;
    saturation = 0.1;
    lightness = 0.4;
  } else if (type == 2) {
    hue = 0.1;
    saturation = 0.5;
    lightness += 0.3;
  } else if (type == 3) { // water
    hue = 0.6;
    lightness = 0.7 + data.g * 0.25 + noise * 0.1;
    int polarity = int( mod(data.g * 255. ,2.) + 0.1);
    if(polarity == 0){
      lightness += 0.01;
    }

  } else if (type == 4) { // gas
    hue = 0.0;
    lightness += 0.4;
    saturation = 0.2 + (data.b * 1.5);
  } else if (type == 5) { // clone
    hue = 0.9;
    saturation = 0.3;
  } else if (type == 6) { // fire
  
    hue = (data.g * 0.1);
    saturation = 0.7;

    lightness = 0.7 + (data.g * 0.3) + ((noise + 0.8) * 0.5);
    if(isSnapshot){
      lightness -=0.2;
    }
  } else if (type == 7) { // wood
    hue = (data.g * 0.1);
    saturation = 0.3;
    lightness = 0.3 + data.g * 0.3;
  } else if (type == 8) { // lava
    hue = (data.g * 0.1);
    lightness = 0.7 + data.g * 0.25 + noise * 0.1;
  } else if (type == 9) { // ice
    hue = 0.6;
    saturation = 0.4;
    lightness = 0.7 + data.g * 0.5;
  } else if (type == 10) { // sink
    hue = 0.9;
    saturation = 0.4;
    lightness = 1.0;
  } else if (type == 11) { // plant
    hue = 0.4;
    saturation = 0.4;
  } else if (type == 12) { // acid
    hue = 0.18;
    saturation = 0.9;
    lightness = 0.8 + data.g * 0.2 + noise * 0.05;
  } else if (type == 13) { // stone
    hue = -0.4 + (data.g * 0.5);
    saturation = 0.1;
    // lightness = 0.2 + data.g * 0.5;
  } else if (type == 14) { // dust
    hue = (data.g * 2.0) + t * .0008;
    saturation = 0.4;
    lightness = 0.8;
  } else if (type == 15) { // mite
    hue = 0.8;
    saturation = 0.9;
    lightness = 0.8;
  } else if (type == 16) { // oil
    hue = (data.g * 5.0) + t * .008;

    saturation = 0.2;
    lightness = 0.3;
  } else if (type == 17) { // Rocket
    hue = 0.0;
    saturation = 0.4 + data.b;
    lightness = 0.9;
  } else if (type == 18) { // fungus
    hue = (data.g * 0.15) - 0.1;
    saturation = (data.g * 0.8) - 0.05;

    // (data.g * 0.00);
    lightness = 1.5 - (data.g * 0.2);
  } else if (type == 19) { // seed/flower

    hue = fract(fract(data.b * 2.) * 0.5) - 0.3;
    saturation = 0.7 * (data.g + 0.4) + data.b * 0.2;
    lightness = 0.9 * (data.g + 0.9);
  } else if (type == 20) { // sponge
    hue = 0.12;
    saturation = 0.3;
    lightness = 0.5 + data.g * 0.3;
  } else if (type == 21) { // slime
    hue = 0.3;
    saturation = 0.7;
    lightness = 0.5 + data.g * 0.2;
  } else if (type == 22) { // glass
    hue = 0.58;
    saturation = 0.1;
    lightness = 0.85 + data.g * 0.1;
  } else if (type == 23) { // coral
    hue = 0.05 + data.g * 0.05;
    saturation = 0.7;
    lightness = 0.6 + data.g * 0.2;
  } else if (type == 24) { // iron
    hue = 0.08;
    saturation = 0.15;
    lightness = 0.45 + data.g * 0.2;
  } else if (type == 25) { // copper
    hue = 0.1;
    saturation = 0.6;
    lightness = 0.5 + data.g * 0.3;
  } else if (type == 26) { // gold
    hue = 0.14;
    saturation = 0.8;
    lightness = 0.55 + data.g * 0.3;
  } else if (type == 27) { // silver
    hue = 0.6;
    saturation = 0.05;
    lightness = 0.7 + data.g * 0.2;
  } else if (type == 28) { // aluminum
    hue = 0.6;
    saturation = 0.08;
    lightness = 0.6 + data.g * 0.2;
  } else if (type == 29) { // lead
    hue = 0.65;
    saturation = 0.1;
    lightness = 0.3 + data.g * 0.2;
  } else if (type == 30) { // zinc
    hue = 0.12;
    saturation = 0.2;
    lightness = 0.5 + data.g * 0.25;
  } else if (type == 31) { // tin
    hue = 0.1;
    saturation = 0.15;
    lightness = 0.55 + data.g * 0.25;
  } else if (type == 32) { // bronze
    hue = 0.11;
    saturation = 0.7;
    lightness = 0.4 + data.g * 0.3;
  } else if (type == 33) { // steel
    hue = 0.6;
    saturation = 0.05;
    lightness = 0.35 + data.g * 0.2;
  } else if (type == 34) { // diamond
    hue = 0.6;
    saturation = 0.0;
    lightness = 0.9 + data.g * 0.1;
  } else if (type == 35) { // ruby
    hue = 0.0;
    saturation = 0.9;
    lightness = 0.6 + data.g * 0.3;
  } else if (type == 36) { // sapphire
    hue = 0.6;
    saturation = 0.8;
    lightness = 0.5 + data.g * 0.3;
  } else if (type == 37) { // emerald
    hue = 0.3;
    saturation = 0.8;
    lightness = 0.5 + data.g * 0.3;
  } else if (type == 38) { // amethyst
    hue = 0.8;
    saturation = 0.6;
    lightness = 0.6 + data.g * 0.3;
  } else if (type == 39) { // quartz
    hue = 0.08;
    saturation = 0.1;
    lightness = 0.8 + data.g * 0.15;
  } else if (type == 40) { // crystal
    hue = 0.55;
    saturation = 0.2;
    lightness = 0.75 + data.g * 0.2;
  } else if (type == 41) { // obsidian
    hue = 0.0;
    saturation = 0.05;
    lightness = 0.15 + data.g * 0.15;
  } else if (type == 42) { // gunpowder
    hue = 0.12;
    saturation = 0.1;
    lightness = 0.2 + data.g * 0.2;
  } else if (type == 43) { // flour
    hue = 0.13;
    saturation = 0.1;
    lightness = 0.85 + data.g * 0.1;
  } else if (type == 44) { // sugar
    hue = 0.12;
    saturation = 0.05;
    lightness = 0.9 + data.g * 0.1;
  } else if (type == 45) { // salt
    hue = 0.0;
    saturation = 0.0;
    lightness = 0.85 + data.g * 0.1;
  } else if (type == 46) { // pepper
    hue = 0.08;
    saturation = 0.2;
    lightness = 0.25 + data.g * 0.15;
  } else if (type == 47) { // ash
    hue = 0.1;
    saturation = 0.05;
    lightness = 0.4 + data.g * 0.2;
  } else if (type == 48) { // soot
    hue = 0.0;
    saturation = 0.05;
    lightness = 0.15 + data.g * 0.15;
  } else if (type == 49) { // charcoal
    hue = 0.08;
    saturation = 0.1;
    lightness = 0.15 + data.g * 0.15;
  } else if (type == 50) { // mud
    hue = 0.12;
    saturation = 0.4;
    lightness = 0.25 + data.g * 0.15 + noise * 0.05;
  } else if (type == 51) { // blood
    hue = 0.0;
    saturation = 0.9;
    lightness = 0.3 + data.g * 0.2;
  } else if (type == 52) { // honey
    hue = 0.14;
    saturation = 0.8;
    lightness = 0.6 + data.g * 0.25;
  } else if (type == 53) { // milk
    hue = 0.12;
    saturation = 0.05;
    lightness = 0.9 + data.g * 0.08;
  } else if (type == 54) { // poison
    hue = 0.35;
    saturation = 0.9;
    lightness = 0.5 + data.g * 0.2;
  } else if (type == 55) { // mercury
    hue = 0.6;
    saturation = 0.05;
    lightness = 0.6 + data.g * 0.2;
  } else if (type == 56) { // alcohol
    hue = 0.6;
    saturation = 0.1;
    lightness = 0.8 + data.g * 0.15;
  } else if (type == 57) { // syrup
    hue = 0.1;
    saturation = 0.7;
    lightness = 0.45 + data.g * 0.2;
  } else if (type == 58) { // steam
    hue = 0.6;
    saturation = 0.05;
    lightness = 0.9 + data.g * 0.08;
  } else if (type == 59) { // smoke
    hue = 0.08;
    saturation = 0.05;
    lightness = 0.35 + data.g * 0.15;
  } else if (type == 60) { // helium
    hue = 0.6;
    saturation = 0.02;
    lightness = 0.95 + data.g * 0.05;
  } else if (type == 61) { // chlorine
    hue = 0.25;
    saturation = 0.5;
    lightness = 0.6 + data.g * 0.2;
  } else if (type == 62) { // oxygen
    hue = 0.55;
    saturation = 0.1;
    lightness = 0.8 + data.g * 0.1;
  } else if (type == 63) { // hydrogen
    hue = 0.6;
    saturation = 0.02;
    lightness = 0.95 + data.g * 0.05;
  } else if (type == 64) { // plasma
    hue = 0.7;
    saturation = 0.9;
    lightness = 0.9 + data.g * 0.1;
  } else if (type == 65) { // methane
    hue = 0.15;
    saturation = 0.2;
    lightness = 0.7 + data.g * 0.15;
  } else if (type == 66) { // leaf
    hue = 0.25;
    saturation = 0.7;
    lightness = 0.4 + data.g * 0.3;
  } else if (type == 67) { // flower
    hue = fract(data.b * 2.0) * 0.5;
    saturation = 0.8;
    lightness = 0.7 + data.g * 0.2;
  } else if (type == 68) { // grass
    hue = 0.3;
    saturation = 0.7;
    lightness = 0.35 + data.g * 0.3;
  } else if (type == 69) { // vine
    hue = 0.28;
    saturation = 0.6;
    lightness = 0.3 + data.g * 0.25;
  } else if (type == 70) { // moss
    hue = 0.3;
    saturation = 0.5;
    lightness = 0.3 + data.g * 0.3;
  } else if (type == 71) { // mushroom
    hue = 0.1;
    saturation = 0.3;
    lightness = 0.55 + data.g * 0.3;
  } else if (type == 72) { // bark
    hue = 0.1;
    saturation = 0.4;
    lightness = 0.25 + data.g * 0.2;
  } else if (type == 73) { // root
    hue = 0.1;
    saturation = 0.3;
    lightness = 0.2 + data.g * 0.2;
  } else if (type == 74) { // fruit
    hue = 0.05;
    saturation = 0.8;
    lightness = 0.5 + data.g * 0.3;
  } else if (type == 75) { // thorn
    hue = 0.08;
    saturation = 0.3;
    lightness = 0.3 + data.g * 0.2;
  } else if (type == 76) { // ant
    hue = 0.1;
    saturation = 0.5;
    lightness = 0.2 + data.g * 0.2;
  } else if (type == 77) { // spider
    hue = 0.0;
    saturation = 0.1;
    lightness = 0.15 + data.g * 0.15;
  } else if (type == 78) { // bee
    hue = 0.14;
    saturation = 0.8;
    lightness = 0.5 + data.g * 0.2;
  } else if (type == 79) { // butterfly
    hue = fract(data.b * 3.0);
    saturation = 0.8;
    lightness = 0.6 + data.g * 0.25;
  } else if (type == 80) { // fish
    hue = 0.55;
    saturation = 0.6;
    lightness = 0.5 + data.g * 0.3;
  } else if (type == 81) { // bird
    hue = 0.08;
    saturation = 0.5;
    lightness = 0.45 + data.g * 0.3;
  } else if (type == 82) { // snake
    hue = 0.3;
    saturation = 0.6;
    lightness = 0.35 + data.g * 0.2;
  } else if (type == 83) { // worm
    hue = 0.08;
    saturation = 0.4;
    lightness = 0.4 + data.g * 0.2;
  } else if (type == 84) { // tnt
    hue = 0.12;
    saturation = 0.7;
    lightness = 0.4 + data.g * 0.2;
  } else if (type == 85) { // bomb
    hue = 0.07;
    saturation = 0.3;
    lightness = 0.2 + data.g * 0.15;
  } else if (type == 86) { // nitro
    hue = 0.13;
    saturation = 0.4;
    lightness = 0.6 + data.g * 0.2;
  } else if (type == 87) { // plutonium
    hue = 0.4;
    saturation = 0.8;
    lightness = 0.4 + data.g * 0.2 + noise * 0.1;
  } else if (type == 88) { // uranium
    hue = 0.35;
    saturation = 0.7;
    lightness = 0.35 + data.g * 0.2;
  } else if (type == 89) { // c4
    hue = 0.12;
    saturation = 0.3;
    lightness = 0.5 + data.g * 0.2;
  } else if (type == 90) { // thermite
    hue = 0.08;
    saturation = 0.7;
    lightness = 0.7 + data.g * 0.3;
  } else if (type == 91) { // napalm
    hue = 0.08;
    saturation = 0.8;
    lightness = 0.5 + data.g * 0.3;
  } else if (type == 92) { // brick
    hue = 0.05;
    saturation = 0.6;
    lightness = 0.4 + data.g * 0.2;
  } else if (type == 93) { // concrete
    hue = 0.1;
    saturation = 0.05;
    lightness = 0.5 + data.g * 0.2;
  } else if (type == 94) { // cement
    hue = 0.12;
    saturation = 0.08;
    lightness = 0.55 + data.g * 0.2;
  } else if (type == 95) { // tile
    hue = 0.08;
    saturation = 0.5;
    lightness = 0.5 + data.g * 0.2;
  } else if (type == 96) { // plaster
    hue = 0.1;
    saturation = 0.05;
    lightness = 0.8 + data.g * 0.15;
  } else if (type == 97) { // marble
    hue = 0.6;
    saturation = 0.03;
    lightness = 0.75 + data.g * 0.15;
  } else if (type == 98) { // granite
    hue = 0.08;
    saturation = 0.1;
    lightness = 0.45 + data.g * 0.2;
  } else if (type == 99) { // basalt
    hue = 0.6;
    saturation = 0.05;
    lightness = 0.18 + data.g * 0.1;
  } else if (type == 100) { // portal
    hue = fract(t * 0.01) + data.g * 0.1;
    saturation = 0.9;
    lightness = 0.6 + data.g * 0.3;
  } else if (type == 101) { // teleporter
    hue = 0.8;
    saturation = 0.8;
    lightness = 0.5 + data.g * 0.3;
  } else if (type == 102) { // antigravity
    hue = 0.55;
    saturation = 0.7;
    lightness = 0.6 + data.g * 0.3;
  } else if (type == 103) { // magnet
    hue = 0.6;
    saturation = 0.4;
    lightness = 0.4 + data.g * 0.2;
  } else if (type == 104) { // lightning
    hue = 0.15;
    saturation = 0.5;
    lightness = 0.9 + data.g * 0.1;
  } else if (type == 105) { // void
    hue = 0.8;
    saturation = 0.5;
    lightness = 0.05 + data.g * 0.05;
  } else if (type == 106) { // chaos
    hue = fract(t * 0.02 + data.g * 0.5);
    saturation = 0.9;
    lightness = 0.6 + data.g * 0.2;
  } else if (type == 107) { // energy
    hue = 0.55;
    saturation = 0.9;
    lightness = 0.8 + data.g * 0.2;
  } else if (type == 108) { // shield
    hue = 0.55;
    saturation = 0.6;
    lightness = 0.5 + data.g * 0.3;
  } else if (type == 109) { // mirror
    hue = 0.6;
    saturation = 0.03;
    lightness = 0.85 + data.g * 0.1;
  } else if (type == 110) { // bread
    hue = 0.12;
    saturation = 0.5;
    lightness = 0.55 + data.g * 0.2;
  } else if (type == 111) { // cheese
    hue = 0.14;
    saturation = 0.7;
    lightness = 0.6 + data.g * 0.2;
  } else if (type == 112) { // meat
    hue = 0.03;
    saturation = 0.7;
    lightness = 0.4 + data.g * 0.2;
  } else if (type == 113) { // egg
    hue = 0.13;
    saturation = 0.3;
    lightness = 0.75 + data.g * 0.15;
  } else if (type == 114) { // rice
    hue = 0.14;
    saturation = 0.2;
    lightness = 0.8 + data.g * 0.15;
  } else if (type == 115) { // wheat
    hue = 0.14;
    saturation = 0.7;
    lightness = 0.5 + data.g * 0.3;
  } else if (type == 116) { // clay
    hue = 0.08;
    saturation = 0.4;
    lightness = 0.45 + data.g * 0.2;
  } else if (type == 117) { // soil
    hue = 0.1;
    saturation = 0.4;
    lightness = 0.25 + data.g * 0.2;
  } else if (type == 118) { // peat
    hue = 0.09;
    saturation = 0.5;
    lightness = 0.15 + data.g * 0.1;
  } else if (type == 119) { // limestone
    hue = 0.12;
    saturation = 0.08;
    lightness = 0.65 + data.g * 0.2;
  } else if (type == 120) { // chalk
    hue = 0.12;
    saturation = 0.03;
    lightness = 0.85 + data.g * 0.1;
  } else if (type == 121) { // shale
    hue = 0.1;
    saturation = 0.1;
    lightness = 0.3 + data.g * 0.15;
  } else if (type == 122) { // slate
    hue = 0.6;
    saturation = 0.08;
    lightness = 0.25 + data.g * 0.15;
  } else if (type == 123) { // sandstone
    hue = 0.12;
    saturation = 0.3;
    lightness = 0.55 + data.g * 0.2;
  } else if (type == 124) { // wire
    hue = 0.08;
    saturation = 0.5;
    lightness = 0.4 + data.g * 0.15;
  } else if (type == 125) { // circuit
    hue = 0.6;
    saturation = 0.4;
    lightness = 0.3 + data.g * 0.2;
  } else if (type == 126) { // battery
    hue = 0.15;
    saturation = 0.6;
    lightness = 0.45 + data.g * 0.2;
  } else if (type == 127) { // solar cell
    hue = 0.55;
    saturation = 0.6;
    lightness = 0.4 + data.g * 0.2;
  } else if (type == 128) { // laser
    hue = 0.0;
    saturation = 0.0;
    lightness = 1.0;
  } else if (type == 129) { // led
    hue = fract(data.g * 0.5);
    saturation = 0.9;
    lightness = 0.8 + data.g * 0.2;
  } else if (type == 130) { // bubble
    hue = 0.6;
    saturation = 0.1;
    lightness = 0.8;
  } else if (type == 131) { // balloon
    hue = fract(data.b * 0.7);
    saturation = 0.8;
    lightness = 0.7 + data.g * 0.2;
  } else if (type == 132) { // confetti
    hue = fract(data.b * 2.5);
    saturation = 0.9;
    lightness = 0.75 + data.g * 0.2;
  } else if (type == 133) { // glitter
    hue = fract(data.g * 1.7 + t * 0.001);
    saturation = 0.5;
    lightness = 0.8 + data.g * 0.2;
  } else if (type == 134) { // spring
    hue = 0.12;
    saturation = 0.4;
    lightness = 0.55 + data.g * 0.2;
  } else if (type == 135) { // domino
    hue = 0.07;
    saturation = 0.5;
    lightness = 0.5 + data.g * 0.2;
  }
  if (isSnapshot == false) {
    lightness *= (0.975 + snoise2(floor(uv * resolution / dpi)) * 0.025);
  }
  color = hsv2rgb(vec3(hue, saturation, lightness));
  gl_FragColor = vec4(color, a);
}
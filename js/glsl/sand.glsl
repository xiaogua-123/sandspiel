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

  // Sample HSV from color lookup table
  float texX = (float(type) + 0.5) / 136.0;
  vec3 hsv = texture2D(colorTable, vec2(texX, 0.5)).rgb;
  float hue = hsv.r;
  float saturation = hsv.s;
  float baseLightness = hsv.b;

  float noise = snoise3(vec3(floor(uv * resolution / dpi), t * 0.05));
  float lightness = baseLightness;

  // Type-specific lightness adjustments
  if (type == 0) {
    a = 0.1;
    if (isSnapshot) {
      saturation = 0.05;
      lightness = 1.01;
      a = 1.0;
    }
  } else if (type == 2) {
    lightness += data.g * 0.5;
  } else if (type == 3) {
    lightness = baseLightness + data.g * 0.25 + noise * 0.1;
    int polarity = int(mod(data.g * 255.0, 2.0) + 0.1);
    if (polarity == 0) { lightness += 0.01; }
  } else if (type == 4) {
    saturation = hsv.s + (data.b * 1.5);
  } else if (type == 6) {
    hue = data.g * 0.1;
    saturation = 0.7;
    lightness = 0.7 + (data.g * 0.3) + ((noise + 0.8) * 0.5);
    if (isSnapshot) { lightness -= 0.2; }
  } else if (type == 7) {
    hue = data.g * 0.1;
    saturation = 0.3;
    lightness = baseLightness + data.g * 0.3;
  } else if (type == 8) {
    hue = data.g * 0.1;
    lightness = baseLightness + data.g * 0.25 + noise * 0.1;
  } else if (type == 9) {
    lightness = baseLightness + data.g * 0.5;
  } else if (type == 12) {
    lightness = baseLightness + data.g * 0.2 + noise * 0.05;
  } else if (type == 13) {
    hue = hsv.r + (data.g * 0.5);
  } else if (type == 14) {
    hue = (data.g * 2.0) + t * 0.0008;
    saturation = 0.4;
    lightness = 0.8;
  } else if (type == 16) {
    hue = (data.g * 5.0) + t * 0.008;
    saturation = 0.2;
    lightness = 0.3;
  } else if (type == 17) {
    saturation = hsv.s + data.b;
  } else if (type == 18) {
    hue = (data.g * 0.15) - 0.1;
    saturation = (data.g * 0.8) - 0.05;
    lightness = 1.5 - (data.g * 0.2);
  } else if (type == 19) {
    hue = fract(fract(data.b * 2.0) * 0.5) - 0.3;
    saturation = 0.7 * (data.g + 0.4) + data.b * 0.2;
    lightness = 0.9 * (data.g + 0.9);
  } else if (type == 20) {
    lightness = baseLightness + data.g * 0.3;
  } else if (type == 21) {
    lightness = baseLightness + data.g * 0.2;
  } else if (type == 22) {
    lightness = baseLightness + data.g * 0.1;
  } else if (type == 23) {
    hue = hsv.r + data.g * 0.05;
    lightness = baseLightness + data.g * 0.2;
  } else if (type == 67) {
    // flower
    hue = fract(data.b * 2.0) * 0.5;
    lightness = baseLightness + data.g * 0.2;
  } else if (type == 79) {
    // butterfly
    hue = fract(data.b * 3.0);
    lightness = baseLightness + data.g * 0.25;
  } else if (type == 100) {
    // portal
    hue = fract(t * 0.01) + data.g * 0.1;
    lightness = baseLightness + data.g * 0.3;
  } else if (type == 106) {
    // chaos
    hue = fract(t * 0.02 + data.g * 0.5);
    lightness = baseLightness + data.g * 0.2;
  } else if (type == 129) {
    // led
    hue = fract(data.g * 0.5);
    lightness = baseLightness + data.g * 0.2;
  } else if (type == 131) {
    // balloon
    hue = fract(data.b * 0.7);
    lightness = baseLightness + data.g * 0.2;
  } else if (type == 132) {
    // confetti
    hue = fract(data.b * 2.5);
    lightness = baseLightness + data.g * 0.2;
  } else if (type == 133) {
    // glitter
    hue = fract(data.g * 1.7 + t * 0.001);
    lightness = baseLightness + data.g * 0.2;
  } else {
    // All other types: dynamic lightness from data.g
    lightness = baseLightness + data.g * 0.5;
  }

  if (isSnapshot == false) {
    lightness *= (0.975 + snoise2(floor(uv * resolution / dpi)) * 0.025);
  }
  color = hsv2rgb(vec3(hue, saturation, lightness));
  gl_FragColor = vec4(color, a);
}

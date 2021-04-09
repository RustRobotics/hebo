// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "base/color.h"

#include <cmath>
#include <sstream>
#include <string>

#include <QRandomGenerator>

namespace hebo {
namespace {

struct NamedColor {
  const char* const name;
  const QColor color;
};

const NamedColor gNamedColors[] = {
    {"transparent", {0, 0, 0, 0}}, {"aliceblue", {240, 248, 255}},
    {"antiquewhite", {250, 235, 215}}, {"aqua", {0, 255, 255}},
    {"aquamarine", {127, 255, 212}}, {"azure", {240, 255, 255}},
    {"beige", {245, 245, 220}}, {"bisque", {255, 228, 196}},
    {"black", {0, 0, 0}}, {"blanchedalmond", {255, 235, 205}},
    {"blue", {0, 0, 255}}, {"blueviolet", {138, 43, 226}},
    {"brown", {165, 42, 42}}, {"burlywood", {222, 184, 135}},
    {"cadetblue", {95, 158, 160}}, {"chartreuse", {127, 255, 0}},
    {"chocolate", {210, 105, 30}}, {"coral", {255, 127, 80}},
    {"cornflowerblue", {100, 149, 237}}, {"cornsilk", {255, 248, 220}},
    {"crimson", {220, 20, 60}}, {"cyan", {0, 255, 255}},
    {"darkblue", {0, 0, 139}}, {"darkcyan", {0, 139, 139}},
    {"darkgoldenrod", {184, 134, 11}}, {"darkgray", {169, 169, 169}},
    {"darkgreen", {0, 100, 0}}, {"darkgrey", {169, 169, 169}},
    {"darkkhaki", {189, 183, 107}}, {"darkmagenta", {139, 0, 139}},
    {"darkolivegreen", {85, 107, 47}}, {"darkorange", {255, 140, 0}},
    {"darkorchid", {153, 50, 204}}, {"darkred", {139, 0, 0}},
    {"darksalmon", {233, 150, 122}}, {"darkseagreen", {143, 188, 143}},
    {"darkslateblue", {72, 61, 139}}, {"darkslategray", {47, 79, 79}},
    {"darkslategrey", {47, 79, 79}}, {"darkturquoise", {0, 206, 209}},
    {"darkviolet", {148, 0, 211}}, {"deeppink", {255, 20, 147}},
    {"deepskyblue", {0, 191, 255}}, {"dimgray", {105, 105, 105}},
    {"dimgrey", {105, 105, 105}}, {"dodgerblue", {30, 144, 255}},
    {"firebrick", {178, 34, 34}}, {"floralwhite", {255, 250, 240}},
    {"forestgreen", {34, 139, 34}}, {"fuchsia", {255, 0, 255}},
    {"gainsboro", {220, 220, 220}}, {"ghostwhite", {248, 248, 255}},
    {"gold", {255, 215, 0}}, {"goldenrod", {218, 165, 32}},
    {"gray", {128, 128, 128}}, {"green", {0, 128, 0}},
    {"greenyellow", {173, 255, 47}}, {"grey", {128, 128, 128}},
    {"honeydew", {240, 255, 240}}, {"hotpink", {255, 105, 180}},
    {"indianred", {205, 92, 92}}, {"indigo", {75, 0, 130}},
    {"ivory", {255, 255, 240}}, {"khaki", {240, 230, 140}},
    {"lavender", {230, 230, 250}}, {"lavenderblush", {255, 240, 245}},
    {"lawngreen", {124, 252, 0}}, {"lemonchiffon", {255, 250, 205}},
    {"lightblue", {173, 216, 230}}, {"lightcoral", {240, 128, 128}},
    {"lightcyan", {224, 255, 255}}, {"lightgoldenrodyellow", {250, 250, 210}},
    {"lightgray", {211, 211, 211}}, {"lightgreen", {144, 238, 144}},
    {"lightgrey", {211, 211, 211}}, {"lightpink", {255, 182, 193}},
    {"lightsalmon", {255, 160, 122}}, {"lightseagreen", {32, 178, 170}},
    {"lightskyblue", {135, 206, 250}}, {"lightslategray", {119, 136, 153}},
    {"lightslategrey", {119, 136, 153}}, {"lightsteelblue", {176, 196, 222}},
    {"lightyellow", {255, 255, 224}}, {"lime", {0, 255, 0}},
    {"limegreen", {50, 205, 50}}, {"linen", {250, 240, 230}},
    {"magenta", {255, 0, 255}}, {"maroon", {128, 0, 0}},
    {"mediumaquamarine", {102, 205, 170}}, {"mediumblue", {0, 0, 205}},
    {"mediumorchid", {186, 85, 211}}, {"mediumpurple", {147, 112, 219}},
    {"mediumseagreen", {60, 179, 113}}, {"mediumslateblue", {123, 104, 238}},
    {"mediumspringgreen", {0, 250, 154}}, {"mediumturquoise", {72, 209, 204}},
    {"mediumvioletred", {199, 21, 133}}, {"midnightblue", {25, 25, 112}},
    {"mintcream", {245, 255, 250}}, {"mistyrose", {255, 228, 225}},
    {"moccasin", {255, 228, 181}}, {"navajowhite", {255, 222, 173}},
    {"navy", {0, 0, 128}}, {"oldlace", {253, 245, 230}},
    {"olive", {128, 128, 0}}, {"olivedrab", {107, 142, 35}},
    {"orange", {255, 165, 0}}, {"orangered", {255, 69, 0}},
    {"orchid", {218, 112, 214}}, {"palegoldenrod", {238, 232, 170}},
    {"palegreen", {152, 251, 152}}, {"paleturquoise", {175, 238, 238}},
    {"palevioletred", {219, 112, 147}}, {"papayawhip", {255, 239, 213}},
    {"peachpuff", {255, 218, 185}}, {"peru", {205, 133, 63}},
    {"pink", {255, 192, 203}}, {"plum", {221, 160, 221}},
    {"powderblue", {176, 224, 230}}, {"purple", {128, 0, 128}},
    {"red", {255, 0, 0}}, {"rosybrown", {188, 143, 143}},
    {"royalblue", {65, 105, 225}}, {"saddlebrown", {139, 69, 19}},
    {"salmon", {250, 128, 114}}, {"sandybrown", {244, 164, 96}},
    {"seagreen", {46, 139, 87}}, {"seashell", {255, 245, 238}},
    {"sienna", {160, 82, 45}}, {"silver", {192, 192, 192}},
    {"skyblue", {135, 206, 235}}, {"slateblue", {106, 90, 205}},
    {"slategray", {112, 128, 144}}, {"slategrey", {112, 128, 144}},
    {"snow", {255, 250, 250}}, {"springgreen", {0, 255, 127}},
    {"steelblue", {70, 130, 180}}, {"tan", {210, 180, 140}},
    {"teal", {0, 128, 128}}, {"thistle", {216, 191, 216}},
    {"tomato", {255, 99, 71}}, {"turquoise", {64, 224, 208}},
    {"violet", {238, 130, 238}}, {"wheat", {245, 222, 179}},
    {"white", {255, 255, 255}}, {"whitesmoke", {245, 245, 245}},
    {"yellow", {255, 255, 0}}, {"yellowgreen", {154, 205, 50}}
};

// Clamp to integer 0 .. 255.
template<typename T>
uint8_t clampCssByte(T i) {
  i = ::round(i);  // Seems to be what Chrome does (vs truncation).
  return i < 0 ? 0 : i > 255 ? 255 : uint8_t(i);
}

// Clamp to float 0.0 .. 1.0.
template<typename T>
float clamp_css_float(T f) {
  return f < 0 ? 0 : f > 1 ? 1 : float(f);
}

float parseFloat(const char* str) {
  return ::strtof(str, nullptr);
}

int64_t parseInt(const char* str, uint8_t base = 10) {
  return ::strtoll(str, nullptr, base);
}

// int or percentage.
uint8_t parse_css_int(const std::string& str) {
  if (str.length() && str.back() == '%') {
    return clampCssByte(parseFloat(str.c_str()) / 100.0f * 255.0f);
  } else {
    return clampCssByte(parseInt(str.c_str()));
  }
}

// float or percentage.
float parseCssFloat(const std::string& str) {
  if (str.length() && str.back() == '%') {
    return clamp_css_float(parseFloat(str.c_str()) / 100.0f);
  } else {
    return clamp_css_float(parseFloat(str.c_str()));
  }
}

float cssHueToRgb(float m1, float m2, float h) {
  if (h < 0.0f) {
    h += 1.0f;
  } else if (h > 1.0f) {
    h -= 1.0f;
  }

  if (h * 6.0f < 1.0f) {
    return m1 + (m2 - m1) * h * 6.0f;
  }
  if (h * 2.0f < 1.0f) {
    return m2;
  }
  if (h * 3.0f < 2.0f) {
    return m1 + (m2 - m1) * (2.0f / 3.0f - h) * 6.0f;
  }
  return m1;
}

std::vector<std::string> split(const std::string& s, char delim) {
  std::vector<std::string> elems;
  std::stringstream ss(s);
  std::string item;
  while (std::getline(ss, item, delim)) {
    elems.push_back(item);
  }
  return elems;
}

}  // namespace

QColor parseColor(QString val) {
  // Remove all whitespace, not compliant, but should just be more accepting.
  val.remove(' ');

  // Convert to lowercase.
  std::string str = val.toLower().toStdString();

  for (const auto& namedColor : gNamedColors) {
    if (str == namedColor.name) {
      return namedColor.color;
    }
  }

  constexpr int kAlphaMax = 255;

  // #abc and #abc123 syntax.
  if (str.length() && str.front() == '#') {
    if (str.length() == 4) {
      int64_t iv = parseInt(str.substr(1).c_str(), 16);
      if (!(iv >= 0 && iv <= 0xfff)) {
        return false;
      } else {
        return {
            static_cast<uint8_t>(((iv & 0xf00) >> 4) | ((iv & 0xf00) >> 8)),
            static_cast<uint8_t>((iv & 0xf0) | ((iv & 0xf0) >> 4)),
            static_cast<uint8_t>((iv & 0xf) | ((iv & 0xf) << 4)),
            kAlphaMax,
        };
      }
    } else if (str.length() == 7) {
      int64_t iv = parseInt(str.substr(1).c_str(), 16);
      if (!(iv >= 0 && iv <= 0xffffff)) {
        // Covers NaN.
        return false;
      } else {
        return {
            static_cast<uint8_t>((iv & 0xff0000) >> 16),
            static_cast<uint8_t>((iv & 0xff00) >> 8),
            static_cast<uint8_t>(iv & 0xff),
            kAlphaMax,
        };
      }
    }

    return {};
  }

  size_t op = str.find_first_of('('), ep = str.find_first_of(')');
  if (op != std::string::npos && ep + 1 == str.length()) {
    const std::string fname = str.substr(0, op);
    const std::vector<std::string> params = split(str.substr(op + 1, ep - (op + 1)), ',');

    float alpha = 1.0f;

    if (fname == "rgba" || fname == "rgb") {
      if (fname == "rgba") {
        if (params.size() != 4) {
          return {};
        }
        alpha = parseCssFloat(params.back());
      } else {
        if (params.size() != 3) {
          return {};
        }
      }

      return {
          parse_css_int(params[0]),
          parse_css_int(params[1]),
          parse_css_int(params[2]),
          static_cast<int>(alpha * kAlphaMax),
      };
    } else if (fname == "hsla" || fname == "hsl") {
      if (fname == "hsla") {
        if (params.size() != 4) {
          return {};
        }
        alpha = parseCssFloat(params.back());
      } else {
        if (params.size() != 3) {
          return {};
        }
      }

      float h = parseFloat(params[0].c_str()) / 360.0f;
      float i;
      // Normalize the hue to [0..1[
      h = std::modf(h, &i);

      float s = parseCssFloat(params[1]);
      float l = parseCssFloat(params[2]);

      float m2 = l <= 0.5f ? l * (s + 1.0f) : l + s - l * s;
      float m1 = l * 2.0f - m2;

      return {
          clampCssByte(cssHueToRgb(m1, m2, h + 1.0f / 3.0f) * 255.0f),
          clampCssByte(cssHueToRgb(m1, m2, h) * 255.0f),
          clampCssByte(cssHueToRgb(m1, m2, h - 1.0f / 3.0f) * 255.0f),
          static_cast<int>(alpha * kAlphaMax),
      };
    }
  }

  return {};
}

QColor randomColor() {
  auto* rng = QRandomGenerator::global();
  constexpr int kLowest = 10;
  constexpr int kHighest = 200;
  const int r = rng->bounded(kLowest, kHighest);
  const int g = rng->bounded(kLowest, kHighest);
  const int b = rng->bounded(kLowest, kHighest);
  return QColor(r, g, b);
}

}  // namespace hebo
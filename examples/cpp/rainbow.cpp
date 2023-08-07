#include "bindings.h"
#include <chrono>
#include <cmath>
#include <iostream>
#include <thread>

void color_from_sine(float percentage, uint8_t (&ledValues)[3]) {
  float pi = M_PI;
  float red = std::sin(percentage * 2.0 * pi) * 0.5 + 0.5;
  float green = std::sin((percentage + 0.33) * 2.0 * pi) * 0.5 + 0.5;
  float blue = std::sin((percentage + 0.67) * 2.0 * pi) * 0.5 + 0.5;
  ledValues[0] = static_cast<uint8_t>(red * 255.0);
  ledValues[1] = static_cast<uint8_t>(green * 255.0);
  ledValues[2] = static_cast<uint8_t>(blue * 255.0);
}

int main() {
  std::cout << "Creating rainbow effect!" << std::endl;
  uint8_t rgb_array[3];
  while (true) {
    int steps = 1000;
    for (int i = 0; i <= steps; i++) {
      float ratio = static_cast<float>(i) / static_cast<float>(steps);
      color_from_sine(ratio, rgb_array);
      set_neopixel(&rgb_array, 1);
      std::this_thread::sleep_for(std::chrono::milliseconds(10));
    }
  }
  return 0;
}

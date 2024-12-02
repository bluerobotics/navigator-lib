#include "bindings.h"
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <sys/utsname.h>
#include <unistd.h>

int main() {

  struct utsname uts;
  uname(&uts);
  printf("Navigator C test, system details:\n");
  printf("System is %s on %s hardware\n", uts.sysname, uts.machine);
  printf("OS Release is %s\n", uts.release);
  printf("OS Version is %s\n", uts.version);

  const char *ci_env = std::getenv("CI");
  if (ci_env && std::string(ci_env) == "true") {
    printf("Running from CI\n");
    printf("Not possible to test navigator sensors yet.\n");

    return 0;
  }

  printf("Initiating navigator module.\n");
  init();

  printf("Setting led on!\n");
  set_led(UserLed::Led1, true);

  printf("Temperature: %f\n", read_temp());

  printf("Pressure: %f\n", read_pressure());

  printf("Leak sensor: %s\n", read_leak() ? "true" : "false");

  float adc[4];
  read_adc_all(adc, 4);
  printf("Reading ADC Channels: 1 = %f, 2 = %f, 3 = %f, 4 = %f\n",
         adc[0], adc[1], adc[2], adc[3]);

  printf("Data ADC Channels: 1 = %f\n", read_adc(AdcChannel::Ch1));

  AxisData mag = read_mag();
  printf("Magnetic field: X = %f, Y = %f, Z = %f\n", mag.x, mag.y, mag.z);

  AxisData accel = read_accel();
  printf("Acceleration: X = %f, Y = %f, Z = %f\n", accel.x, accel.y, accel.z);

  AxisData gyro = read_gyro();
  printf("Gyroscope: X = %f, Y = %f, Z = %f\n", gyro.x, gyro.y, gyro.z);

  printf("Setting led off!\n");
  set_led(UserLed::Led1, false);

  return 0;
}

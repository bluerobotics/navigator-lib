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

  ADCData adc = read_adc_all();
  printf("Reading ADC Channels: 1 = %i, 2 = %i, 3 = %i, 4 = %i\n",
         adc.channel[0], adc.channel[1], adc.channel[2], adc.channel[3]);

  printf("Data ADC Channels: 1 = %i\n", read_adc(AdcChannel::Ch1));

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

#include <stdio.h>
#include <stdint.h>
#include "bindings.h"

int main() {
    printf("Tire width: %f\n", create_random_tire().size.width);
    return 0;
}

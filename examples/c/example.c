#include "example.h"
#include <stdio.h>

int main()
{
    printf("Hello from C code!\n");
    printf("Rust code returned: %f\n", pi_approximation(1000000));

    return 0;
}

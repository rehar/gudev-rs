// Generated by gir (https://github.com/gtk-rs/gir @ e0d8d8d645b1)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 3ff4d3275258)
// from gudev-gir
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) G_UDEV_DEVICE_TYPE_BLOCK);
    PRINT_CONSTANT((gint) G_UDEV_DEVICE_TYPE_CHAR);
    PRINT_CONSTANT((gint) G_UDEV_DEVICE_TYPE_NONE);
    return 0;
}

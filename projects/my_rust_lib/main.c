#include <stdio.h>

extern void call_from_c();

int main () {
    call_from_c();

    return 0;
}

#include "random.h"
#include <stdio.h>

int main(int argc, char **argv) {
    char msg[] = "unlock";
    char digest[16] = {0};
    SetEntropyUsing(msg, 6);
    RandomSeed(digest);
    for(int x = 0; x < 16; x++) {
        putchar(digest[x]);
    }
}
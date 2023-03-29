#include <string.h>
#include <stdio.h>

typedef unsigned int uint32_t;
typedef unsigned char uint8_t;

typedef struct {
    uint8_t car_id[8];
    uint8_t num_active;
    uint8_t features[3];
} FEATURE_DATA;

#define CAR_ID "5"

const uint8_t car_id[] = CAR_ID;

int main(int argc, char **argv) {
    FEATURE_DATA f = {0};
    f.car_id[0] = '5';
    f.car_id[1] = 0;
    f.num_active = 3;
    f.features[0] = 1;
    f.features[1] = 2;
    f.features[2] = 3;
    char *buf = &f;
    for(int x = 0; x < sizeof(FEATURE_DATA); x++) {
        putchar(buf[x]);
    }
}
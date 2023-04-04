#include "sha2.h"

#include <stdio.h>
#include <string.h>

#define sha_256_pin "B78C53B964AB04FEFE19BEE2600B38E84A2C4BA735D23BE7600965221E0979B7"

#define DIGEST_BYTES 32

int main(void){
    
    uint64_t pin = 0;
    uint8_t arr_pin[8];
    uint8_t received_hash[DIGEST_BYTES];

    for (int i = 0; i < 64; i=i+2) {
      char hex[2];
      strncpy(hex, &sha_256_pin[i], 2);
      received_hash[i/2] = (uint8_t)strtol(hex, 0, 16);
    }

    for(;pin < 16777217;pin++){
        memcpy(arr_pin,&pin,sizeof(pin));
        sha256_ctx cx[1];
        uint8_t message_hash[DIGEST_BYTES];
        size_t len = 6;

        sha256_begin(cx);
        sha256(message_hash, arr_pin, len, cx);
        
        if (0==(strncmp((char *)message_hash, (char *)received_hash, DIGEST_BYTES))){
            printf("Pin is %lx\n",pin);
            break;
        }
    }
    
    return 0;
}



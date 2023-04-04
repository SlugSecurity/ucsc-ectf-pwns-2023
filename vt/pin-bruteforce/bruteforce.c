#include "sha2.h"

#include <stdio.h>
#include <string.h>


//change this to be the pin hash we get from ROP
#define sha_256_pin "1e6c30ab91d1f1d61996ae17bc9af1ca8bcded8a0414de79258dc0c52d2215ee"

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
    //this should be 16^6+1
    for(;pin < 16777217;pin++){
        memcpy(arr_pin,&pin,sizeof(pin));

        //I hate all of this
        char string_pin[8];
        sprintf(string_pin,"%06lx",pin);

        sha256_ctx cx[1];
        uint8_t message_hash[DIGEST_BYTES];
        size_t len = 6;

        sha256_begin(cx);

        //if this doesnt work, replace with arr_pin
        sha256(message_hash, (uint8_t *)string_pin, len, cx);
        
        if (0==(memcmp((char *)message_hash, (char *)received_hash, DIGEST_BYTES))){
            printf("Pin is %lx\n",pin);
            break;
        }
    }
    
    return 0;
}



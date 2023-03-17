MEMORY
{
    FLASH      (rx)  :  ORIGIN = 0x00000000,  
    LENGTH = 110K
    STACK      (rw)  :  ORIGIN = 0x20000000,                             LENGTH = 28K
    RAM        (rw)  :  ORIGIN = ORIGIN(STACK) + LENGTH(STACK),          LENGTH = 32K - LENGTH(STACK)
}

/*
Add a block of memory for the stack before the RAM block, so that a stack overflow leaks into
reserved space and flash memory, instead of .data and .bss.
*/

_stack_start = ORIGIN(STACK) + LENGTH(STACK);

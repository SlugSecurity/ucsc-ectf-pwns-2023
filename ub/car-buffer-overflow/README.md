UB's car firmware has a potential buffer overflow in the receiveAnswerStartCar() function of car/src/firmware.c when 
receiving the answer to the random challenge sent by the car. The buffer assigned to the ``message`` variable only contains
224 bytes because they assign ``message.buffer`` to ``buffer + sizeof(challenge)`` where ``buffer`` is a 256 byte buffer and 
``sizeof(challenge)`` is 32. The 1 byte unsigned message length is completely user-controlled and can be set to anything
between 0-255 meaning that any buffer assigned to any ``MESSAGE_PACKET`` must be 255 bytes long or more or else a buffer
overflow is possible. By writing particular bytes past the buffer, the r5 register and return address can be overwritten
after returning early, allowing an attacker to return out to the part of the code that reads the unlock flag from the EEPROM
and writes it to UART0 

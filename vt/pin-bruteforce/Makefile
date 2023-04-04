CC = clang
CFLAGS = -Wall -Werror -Wextra -Wpedantic -Ofast $(shell pkg-config --cflags gmp)
LFLAGS = $(shell pkg-config --libs gmp)

all: bruteforce

bruteforce: bruteforce.o sha2.o 
	$(CC) -o $@ $^ $(LFLAGS)

%.o: %.c
	$(CC) $(CFLAGS) -c $<

clean:
	rm -f bruteforce *.o


format:
	clang-format -i -style=file *.[ch]


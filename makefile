CC = gcc
CFLAGS = -O3
LDFLAGS =

.PHONY: main

build: main
	$(CC) $(CFLAGS) $(LDFLAGS) *.o -o numstr

main:
	$(CC) $(CFLAGS) -c main.c

run: main
	./numstr

clean:
	rm *.o numstr

debug_build: CFLAGS = -Wall -Wextra -g
debug_build: build

debug_run: debug_build run
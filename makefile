CC = gcc
CFLAGS = -O3
LDFLAGS =
NAME = numstr
RELEASE_TAG = 0.0

.PHONY: main

build: main
	$(CC) $(CFLAGS) $(LDFLAGS) *.o -o $(NAME)

main:
	$(CC) $(CFLAGS) -c main.c

run: main
	./$(NAME)

clean:
	rm -r *.o $(NAME) $(NAME)_* *.exe

debug_build: CFLAGS = -Wall -Wextra -g
debug_build: build

debug_run: debug_build run

linux_release: build
	mkdir $(NAME)_$(RELEASE_TAG)-linux
	cp -rt $(NAME)_$(RELEASE_TAG)-linux $(NAME) ./langs/
	tar -zcvf $(NAME)_$(RELEASE_TAG)-linux.tar.gz $(NAME)_$(RELEASE_TAG)-linux

windows_release: CC=x86_64-w64-mingw32-gcc
windows_release: build
	mkdir $(NAME)_$(RELEASE_TAG)-win64
	cp -rt $(NAME)_$(RELEASE_TAG)-win64 $(NAME).exe ./langs/
	zip -9r $(NAME)_$(RELEASE_TAG)-win64.zip $(NAME)_$(RELEASE_TAG)-win64

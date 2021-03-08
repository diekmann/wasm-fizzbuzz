CC=clang
AR=llvm-ar-10

CFLAGS=--target=wasm32 -m32 -g -O0 -Wall -nostdlib -ffreestanding -nostdinc -I../musl-1.2.2/built_inc/include/
LIBS=-L../musl-1.2.2/lib -lc

SOURCES = main.c
OBJS = $(addprefix build/,$(SOURCES:.c=.o)) 

libmain.a: main.c
	$(CC) $(CFLAGS) $(LIBS) -c $< -o main.o
	$(AR) rcs $@ main.o

main: main.c
	$(CC) $(CFLAGS) -Wl,--no-entry -Wl,--export-all -Wl,--import-memory -Wl,--allow-undefined-file=wasm.syms $(LIBS) $< -o main.wasm

run:  main
	python3 -m http.server --bind 127.0.0.1

clean:
	rm -rf main.wasm main.o libmain.a
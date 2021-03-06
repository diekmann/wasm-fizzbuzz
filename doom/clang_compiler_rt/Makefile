CC=clang
AR=llvm-ar-10

CFLAGS=--target=wasm32 -m32 -O2 -Wall -nostdlib -ffreestanding -nostdinc -I../musl-1.2.2/include/ -I../musl-1.2.2/arch/wasm32/ -I../musl-1.2.2/arch/generic/

SOURCES = extenddftf2.c extendsftf2.c
OBJS = $(addprefix build/,$(SOURCES:.c=.o)) 

build/%.o: %.c
	mkdir -p build
	$(CC) $(CFLAGS) -c $< -o $@

build/libclang_rt.builtins-wasm32.a: $(OBJS)
	$(AR) rcs $@ $^

clean:
	rm -rf build
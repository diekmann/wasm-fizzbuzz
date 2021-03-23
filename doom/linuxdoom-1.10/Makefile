################################################################
#
# $Id:$
#
# $Log:$
#
CC=clang
AR=llvm-ar-10

# If your get `error: unable to execute command: Executable "wasm-ld" doesn't exist!` on Ubntu 20.04
# apt install lld
# ln -s /usr/bin/wasm-ld-10 /usr/bin/wasm-ld # ugly!
##############TODO:::: use -I musl built_inc/!!!!!
WASM32_CFLAGS=--target=wasm32 -nostdlib -ffreestanding -nostdinc -I../musl-1.2.2/built_inc/include/

# doom was written with the implicit assumption that ints and pointers are 32bit.
# Adding -m32 to make it compile on 64bit archs.
# on Ubuntu 20.04:
# apt install gcc-multilib libx11-dev:i386 libxext-dev:i386
DOOM_CFLAGS=-m32 -O2 -Wall -Wno-unused-const-variable -DNORMALUNIX -DLINUX 

CFLAGS=$(DOOM_CFLAGS) $(WASM32_CFLAGS)
LDFLAGS=-Wl,--no-entry -Wl,--export-all -nostdlib -Wl,--error-limit=0 #-L/usr/X11R6/lib
LIBS=#-lX11

# subdirectory for objects
O=linux

# not too sophisticated dependency
# TODO: $(O)/i_video.o removed, because I don't want to depend on X11 headers. Cleanup needed!
OBJS=				\
		$(O)/doomdef.o		\
		$(O)/doomstat.o		\
		$(O)/dstrings.o		\
		$(O)/i_system.o		\
		$(O)/i_sound.o		\
		$(O)/i_net.o			\
		$(O)/tables.o			\
		$(O)/f_finale.o		\
		$(O)/f_wipe.o 		\
		$(O)/d_main.o			\
		$(O)/d_net.o			\
		$(O)/d_items.o		\
		$(O)/g_game.o			\
		$(O)/m_menu.o			\
		$(O)/m_misc.o			\
		$(O)/m_argv.o  		\
		$(O)/m_bbox.o			\
		$(O)/m_fixed.o		\
		$(O)/m_swap.o			\
		$(O)/m_cheat.o		\
		$(O)/m_random.o		\
		$(O)/am_map.o			\
		$(O)/p_ceilng.o		\
		$(O)/p_doors.o		\
		$(O)/p_enemy.o		\
		$(O)/p_floor.o		\
		$(O)/p_inter.o		\
		$(O)/p_lights.o		\
		$(O)/p_map.o			\
		$(O)/p_maputl.o		\
		$(O)/p_plats.o		\
		$(O)/p_pspr.o			\
		$(O)/p_setup.o		\
		$(O)/p_sight.o		\
		$(O)/p_spec.o			\
		$(O)/p_switch.o		\
		$(O)/p_mobj.o			\
		$(O)/p_telept.o		\
		$(O)/p_tick.o			\
		$(O)/p_saveg.o		\
		$(O)/p_user.o			\
		$(O)/r_bsp.o			\
		$(O)/r_data.o			\
		$(O)/r_draw.o			\
		$(O)/r_main.o			\
		$(O)/r_plane.o		\
		$(O)/r_segs.o			\
		$(O)/r_sky.o			\
		$(O)/r_things.o		\
		$(O)/w_wad.o			\
		$(O)/wi_stuff.o		\
		$(O)/v_video.o		\
		$(O)/st_lib.o			\
		$(O)/st_stuff.o		\
		$(O)/hu_stuff.o		\
		$(O)/hu_lib.o			\
		$(O)/s_sound.o		\
		$(O)/z_zone.o			\
		$(O)/info.o				\
		$(O)/sounds.o

all:	 $(O)/linuxxdoom

clean:
	rm -f *.o *~ *.flc
	rm -f $(O)/*

# link-time optimization only when building the binary, but not when building the *.o files, since we want to link them with rust.
$(O)/linuxxdoom:	$(OBJS) $(O)/i_main.o
	$(CC) -flto $(CFLAGS) $(LDFLAGS) $(OBJS) $(O)/i_main.o \
	-o $(O)/linuxxdoom $(LIBS)

$(O)/%.o:	%.c
	mkdir -p $(O)
	$(CC) $(CFLAGS) -nostdlib  -c $< -o $@

run:	all
	Xephyr :1 -screen 640x400x8 -title DooM &
	sleep 1 # ugly!! waiting for Xephyr to start.
	DISPLAY=:1 ./$(O)/linuxxdoom -2

$(O)/liblinuxxdoom.a: $(OBJS)
	$(AR) rcs $@ $^

#############################################################
#
#############################################################

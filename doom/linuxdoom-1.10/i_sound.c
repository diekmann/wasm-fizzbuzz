// Emacs style mode select   -*- C++ -*- 
//-----------------------------------------------------------------------------
//
// $Id:$
//
// Copyright (C) 1993-1996 by id Software, Inc.
//
// This source is available for distribution and/or modification
// only under the terms of the DOOM Source Code License as
// published by id Software. All rights reserved.
//
// The source is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// FITNESS FOR A PARTICULAR PURPOSE. See the DOOM Source Code License
// for more details.
//
// $Log:$
//
// DESCRIPTION:
//	System interface for sound.
//
//-----------------------------------------------------------------------------

static const char
rcsid[] = "$Id: i_unix.c,v 1.5 1997/02/03 22:45:10 b1 Exp $";

#include "i_sound.h"



// UNIX hack, to be removed.
#ifdef SNDSERV
char*  sndserver_filename = "./sndserver ";
#endif



void I_SetChannels(){}

void I_SetSfxVolume(int volume){}

void I_SetMusicVolume(int volume){}

int I_GetSfxLumpNum(sfxinfo_t* sfx)
{
  return 0;
}

int
I_StartSound
( int		id,
  int		vol,
  int		sep,
  int		pitch,
  int		priority )
{
  return 0;
}

void I_StopSound (int handle){}


int I_SoundIsPlaying(int handle)
{
    // Ouch.
    return 0;
}

void I_UpdateSound( void ){}

void I_SubmitSound(void){}



void
I_UpdateSoundParams
( int	handle,
  int	vol,
  int	sep,
  int	pitch){}

void I_ShutdownSound(void){}

void I_InitSound(){}

void I_InitMusic(void)		{ }
void I_ShutdownMusic(void)	{ }
void I_PlaySong(int handle, int looping){}

void I_PauseSong (int handle){}

void I_ResumeSong (int handle){}

void I_StopSong(int handle){}

void I_UnRegisterSong(int handle){}

int I_RegisterSong(void* data)
{
  return 1;
}

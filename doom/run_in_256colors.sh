#!/bin/sh
Xephyr :1 -screen 640x400x8 -title DooM &
sleep 1 # ugly!! waiting for Xephyr to start.
DISPLAY=:1 "${1}" -2
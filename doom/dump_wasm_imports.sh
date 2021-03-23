#!/bin/sh

# apt install wabt
wasm2wat "${1}" | grep '\(import \|export \)'
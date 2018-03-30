#!/bin/sh

# --generate-inline-functions
CRT="C:\Program Files (x86)\Windows Kits\10\Include\10.0.10240.0\ucrt"
VC="C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\include"
bindgen embree-3.0.0.x64.windows/include/embree3/rtcore.h --no-layout-tests --rust-target 1.25 -o pregenerated_bindings.rs -- -I"$CRT" -I"$VC"
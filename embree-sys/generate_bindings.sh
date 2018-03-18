#!/bin/sh

bindgen embree-3.0.0.x64.windows/include/embree3/rtcore.h -o pregenerated_bindings.rs -- -I"C:\Program Files (x86)\Windows Kits\10\Include\10.0.10240.0\ucrt"
#!/bin/sh

# --generate-inline-functions
CRT="C:\Program Files (x86)\Windows Kits\10\Include\10.0.10240.0\ucrt"
VC="C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\include"
bindgen embree3/include/embree3/rtcore.h -o pregenerated_bindings.rs \
    --no-layout-tests --rust-target 1.25 \
    --whitelist-function "rtc.*" \
    --whitelist-type "RTC.*" \
    --whitelist-var "rtc.*" \
    --whitelist-var "RTC.*" \
    -- -I"$CRT" -I"$VC"
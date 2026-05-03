@echo off
set "PATH=C:\Program Files\gstreamer\1.0\msvc_x86_64\bin;%PATH%"
cargo run --release %*

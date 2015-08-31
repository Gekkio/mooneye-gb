@echo off

cargo rustc --release -- -C link-args="-Wl,--subsystem,windows"

#!/usr/bin/env bash

# Regenerate Rust bindings for the OpenCSD C API using the rust-bindgen CLI tool. This should be run 
# on a *nix system. Bindings for *nix systems are committed to the repo for convenience to reduce 
# dependencies and build time. Windows systems will invoke bindgen at build time to generate 
# bindings.

bindgen wrapper.h -o src/bindings.rs \
  --wrap-unsafe-ops \
  -- \
  -Ivendor/OpenCSD/decoder/include/opencsd/c_api \
  -Ivendor/OpenCSD/decoder/include

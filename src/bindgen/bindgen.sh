#!/bin/sh
set -eu

# Define distribution
DIST="vendored/v0.4.5-dist"

# Go into script dir and call bindgen
#  NOTE: This script requires a valid config.h, so it may be necessary to attempt a build first which creates the config.h
cd "`dirname $0`"
bindgen \
    \
    --allowlist-function="EverCrypt_.*" --allowlist-type="EverCrypt_.*" --allowlist-var="EverCrypt_.*" \
    --allowlist-function="Spec_.*" --allowlist-type="Spec_.*" --allowlist-var="Spec_.*" \
    --allowlist-function="Hacl_.*" --allowlist-type="Hacl_.*" --allowlist-var="Hacl_.*" \
    \
    --blocklist-function="Hacl_Blake2b_32_blake2b_update_multi" \
    --blocklist-function="Hacl_Blake2b_32_blake2b_update_last" \
    --blocklist-function="Hacl_Hash_SHA2_update_last_384" \
    --blocklist-function="Hacl_Hash_SHA2_update_last_512" \
    --blocklist-function="Hacl_Blake2b_256_blake2b_update_multi" \
    --blocklist-function="Hacl_Blake2b_256_blake2b_update_last" \
    \
    --no-layout-tests --output="bindgen.rs" "bindgen.h" \
    -- -I"../../$DIST/c89-compatible" -I"../../$DIST/kremlin/include" -I"../../$DIST/kremlin/kremlib/dist/minimal" 

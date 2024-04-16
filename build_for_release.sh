#!/bin/bash

fnr() {
    sed -i "s|$old|$new|g" Cargo.toml
}

build() {

    # Ensure target directory is clean
    rm -rf target/$target

    # Disable dynamic linking for release builds
    old='bevy = { version = "0.13.2" , features = \["dynamic_linking"\] }'
    new='bevy = { version = "0.13.2" }'
    fnr

    cargo build --target=$target --release

    # Enable dynamic linking for debug builds
    old='bevy = { version = "0.13.2" }'
    new='bevy = { version = "0.13.2" , features = \["dynamic_linking"\] }'
    fnr

    # Copy program into the builds directory
    mkdir builds/$platform_name
    cp target/$target/release/$program_name builds/$platform_name/
    cp -r assets builds/$platform_name/

    # Zip
    cd builds
    zip -r $platform_name.zip $platform_name
    rm -r $platform_name
    cd ..
}

target="x86_64-unknown-linux-gnu"
platform_name="linux"
program_name="hexen_havoc"
build


target="x86_64-pc-windows-gnu"
platform_name="windows"
program_name="hexen_havoc.exe"
build

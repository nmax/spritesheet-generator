#!/bin/sh

rm spritesheet.png
rm spritesheet.css
cargo build --release
target/release/sprite-generator test/imgs/*.png  --name=test --strategy=pack

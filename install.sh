#!/bin/bash
cargo build --release
sudo cp ./target/release/rigos /usr/bin/rigos

#!/bin/sh
cd scripts && python3 copyright.py
cargo clippy --fix --lib -p libmonero --allow-dirty # For fixing clippy warnings
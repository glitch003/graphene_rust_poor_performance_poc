#!/bin/bash

cargo build

cp target/debug/poc .

make clean

SGX=1 make all
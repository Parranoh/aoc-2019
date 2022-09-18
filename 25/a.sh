#!/bin/sh
cd "$(dirname "$0")"
[ -x intcode-ascii ] || rustc -O ../intcode-ascii.rs
{
    cat
    cat /dev/tty
} | ./intcode-ascii

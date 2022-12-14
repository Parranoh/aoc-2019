#!/bin/sh
cd "$(dirname "$0")"
[ -x intcode-ascii ] || rustc -O ../intcode-ascii.rs
{
    cat
    cat <<\EOF
NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
NOT E T
NOT T T
OR H T
AND T J
RUN
EOF
} | ./intcode-ascii | tail -n1

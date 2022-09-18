#!/bin/sh
cd "$(dirname "$0")"
[ -x intcode-ascii ] || rustc -O ../intcode-ascii.rs
cat input - <<\EOF | ./intcode-ascii | tail -n1
NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
EOF

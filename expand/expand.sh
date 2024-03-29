#!/usr/bin/env sh
mkdir -p expanded
RS_SRC="expanded/out.rs"
IGNORE="expanded/.gitignore"
[ ! -f $RS_SRC ] && rm $RS_SRC
[ ! -f $IGNORE ] && echo '*' > $IGNORE
cargo +nightly rustc --profile=test -- -Zunstable-options --pretty=expanded > $RS_SRC
[ -x "$(command -v cargo-fmt)" ] || rustup component add rustfmt --toolchain nightly
cargo +nightly fmt -- $RS_SRC
[ -x "$(command -v bat)" ] && bat $RS_SRC || cat $RS_SRC
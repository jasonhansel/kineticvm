#!/bin/sh

# Generate KineticVM documentation in the "docs" folder.
# We use a rather complex command so that private symbols will also get documented.
rustdoc --no-defaults --passes "collapse-docs" --passes "unindent-comments" ./src/main.rs --crate-name kineticvm -L ./target/debug/deps --cfg doc

# Open the resulting documentation file.
xdg-open ./doc/kineticvm/index.html
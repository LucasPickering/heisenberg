#!/bin/sh

exec watchexec --restart --no-process-group \
    --watch Cargo.toml --watch Cargo.lock --watch src/ \
    -- cargo run

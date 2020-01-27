- Getting started rust
    - https://www.rust-lang.org/learn/get-started
        - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
        - `source ~/.profile`
    - enable clion suggest
        -  `rustup component add rust-src`

- install libs
    - install `cargo install cargo-edit`

- code format
    - rustfmt
        - https://github.com/rust-lang/rustfmt

- diesel
    - create Database
        -  docker run -it --rm --network=example_gooldbook postgres:alpine psql -h db01 -p 26257 -U root -c "CREATE DATABASE gooldbook_example"
    - create migration
        - diesel migration generate {table_name}
    - migration up
        - diesel migration run --database-url postgres://root:@localhost:4000/gooldbook_example
    - migration down
        - diesel migration redo --database-url postgres://root:@localhost:4000/gooldbook_example

-  toolchain XXX does not support components
    - try uninstall and install
        - `rustup toolchain uninstall XXX`
        - `rustup toolchain install XXX`
        - `rust show`
        - `rustup component add rust-src`
        - https://github.com/rust-lang/rustup/issues/1793
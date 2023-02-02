generate:
    #!/usr/bin/env bash

    cargo run -p generate | rustfmt > src/generated.rs

# Get binary of isolated full simulation

    $ cargo test --release --no-run test_simulate_rounds_debug --message-format=json | jq -r 'select(.executable != null) | .executable'


# Heaptrack

    $ heaptrack ./target/release/deps/simulation_tests-...

# Bytehound
## Basic usage (from Bytehound documentation)

    $ export MEMORY_PROFILER_LOG=warn
    $ LD_PRELOAD=./libbytehound.so ./your_application
    $ ./bytehound server memory-profiling_*.dat

Then open your Web browser and point it at `http://localhost:8080` to access the GUI.

# Cargo Flamegraph

    $ cargo install flamegraph
    # Rust projects
    $ cargo flamegraph
    # Arbitrary binaries
    $ flamegraph -- /path/to/binary

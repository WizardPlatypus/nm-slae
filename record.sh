# to build a flamegraph
cargo flamegraph -o $1/flamegraph.svg > /dev/null

# timing
/usr/bin/time --verbose target/release/matrices > /dev/null 2> $1/time.txt

# perf stat
perf stat -d target/release/matrices > /dev/null 2> $1/perf-stat.txt

# perf record
perf record -o $1/perf.data target/release/matrices > /dev/null

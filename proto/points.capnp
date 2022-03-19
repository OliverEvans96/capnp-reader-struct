@0x986b3393db1396c9;
# From the Cap'n Proto Rust readme
# https://github.com/capnproto/capnproto-rust

struct Point {
    x @0 :Float32;
    y @1 :Float32;
}

interface PointTracker {
    addPoint @0 (p :Point) -> (totalPoints :UInt64);
}
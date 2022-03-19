fn main() {
    ::capnpc::CompilerCommand::new()
        .file("proto/points.capnp")
        .run()
        .expect("compiling schema");
}

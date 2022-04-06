fn main() {
    tonic_build::configure()
        .build_server(false)
        .compile(&["../protos/proto/hello_world.proto"], &["../protos/proto"])
        .unwrap();
}

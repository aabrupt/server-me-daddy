use protobuf_codegen;
use protoc_bin_vendored;
fn main() {
    protobuf_codegen::Codegen::new()
        .protoc()
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().expect("missing protoc instalation"))
        .include("protoc")
        .input("protoc/api.proto")
        .cargo_out_dir("protoc")
        .run()
        .expect("failed to run");
}
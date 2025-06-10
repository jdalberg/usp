fn main() {
    prost_build::compile_protos(&["proto/usp-msg-1-1.proto"], &["proto"])
        .expect("Failed to compile Protobufs for USP messages");
    prost_build::compile_protos(&["proto/usp-record-1-1.proto"], &["proto"])
        .expect("Failed to compile Protobufs for USP records");
}
// This build script compiles the Protobuf definitions found in `proto/usp-msg.proto`

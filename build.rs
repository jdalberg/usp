fn main() {
    let mut prost_usp_build = prost_build::Config::new();
    prost_usp_build.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");

    let mut prost_usp_record_build = prost_build::Config::new();
    prost_usp_record_build.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");

    prost_usp_build
        .compile_protos(&["proto/usp-msg-1-1.proto"], &["proto"])
        .expect("Failed to compile Protobufs for USP messages");
    prost_usp_record_build
        .compile_protos(&["proto/usp-record-1-1.proto"], &["proto"])
        .expect("Failed to compile Protobufs for USP records");
}
// This build script compiles the Protobuf definitions found in `proto/usp-msg.proto`

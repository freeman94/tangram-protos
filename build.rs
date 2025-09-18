use glob::glob;
use protobuf_codegen::Codegen;
use std::path::PathBuf;

fn main() {
    let mut protos: Vec<PathBuf> = Vec::new();

    // Add foxglove protos
    let foxglove_pattern = "tangram-protobuf-messages/protos/foxglove/*.proto";
    let foxglove_protos: Vec<PathBuf> = glob(foxglove_pattern).unwrap()
        .filter_map(|e| match e {
            Ok(v) => Some(v),
            Err(_) => None,
        })
        .collect();
    protos.extend(foxglove_protos);

    // Add tangram protos
    let tangram_pattern = "tangram-protobuf-messages/protos/tangram/*.proto";
    let tangram_protos: Vec<PathBuf> = glob(tangram_pattern).unwrap()
        .filter_map(|e| match e {
            Ok(v) => Some(v),
            Err(_) => None,
        })
        .collect();
    protos.extend(tangram_protos);

    Codegen::new()
        .pure()
        .cargo_out_dir("generated_protos")
        .include("tangram-protobuf-messages/protos/")
        .inputs(protos)
        .run_from_script();
}

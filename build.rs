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

    // Add tangram detection protos
    let detections_pattern = "tangram-protobuf-messages/protos/tangram/detections/*.proto";
    let detections_protos: Vec<PathBuf> = glob(detections_pattern).unwrap()
        .filter_map(|e| match e {
            Ok(v) => Some(v),
            Err(_) => None,
        })
        .collect();
    protos.extend(detections_protos);

    // Add ROS protos if the ros feature is enabled
    if cfg!(feature = "ros") {
        let ros_protos: Vec<PathBuf> = glob("tangram-protobuf-messages/protos/ros/**/*.proto").unwrap()
            .filter_map(|e| match e {
                Ok(v) => Some(v),
                Err(_) => None,
            })
            .collect();
        protos.extend(ros_protos);
    }

    Codegen::new()
        .pure()
        .cargo_out_dir("generated_protos")
        .include("tangram-protobuf-messages/protos/")
        .inputs(protos)
        .run_from_script();
}

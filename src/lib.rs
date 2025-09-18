pub mod tangram_protos {
    include!(concat!(env!("OUT_DIR"), "/generated_protos/mod.rs"));
}

pub use tangram_protos::*;
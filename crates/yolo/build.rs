fn main() {
    #[cfg(feature = "backend")]
    backend_build();
}

#[cfg(feature = "backend")]
fn backend_build() {
    use burn_onnx::ModelGen;
    use std::path::PathBuf;

    const OUT_SUBDIR: &str = "model/";

    println!("cargo:rerun-if-changed=src/model");

    const INPUT_ONNX_FILE1: &str = "src/model/yolo26n-pose-nonms.onnx";
    ModelGen::new()
        .input(INPUT_ONNX_FILE1)
        .out_dir(OUT_SUBDIR)
        .embed_states(true)
        .run_from_script();

    // Patch burn-onnx generated code to fix compilation errors.
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let gen_file = out_dir.join(OUT_SUBDIR).join("yolo26n-pose-nonms.rs");
    if let Ok(src) = std::fs::read_to_string(&gen_file) {
        let patched = src.replace("alloc::vec::Vec<_>", "std::vec::Vec<_>");

        let patched = patched.replace(
            "    ) -> (\n        Tensor<B, 3>,\n        Tensor<B, 3>,\n        Tensor<B, 3>,\n        Tensor<B, 4>,\n        Tensor<B, 4>,\n        Tensor<B, 4>,\n        Tensor<B, 3>,\n        Tensor<B, 3>,\n        Tensor<B, 3>,\n        Tensor<B, 4>,\n        Tensor<B, 4>,\n        Tensor<B, 4>,\n        Tensor<B, 3>,\n    ) {",
            "    ) -> Tensor<B, 3> {",
        );

        let patched = patched.replace(
            "        (\n            concat30_out1,\n            concat20_out1,\n            concat21_out1,\n            mul54_out1,\n            mul64_out1,\n            mul71_out1,\n            concat22_out1,\n            concat23_out1,\n            concat24_out1,\n            mul54_out1,\n            mul64_out1,\n            mul71_out1,\n            concat25_out1,\n        )",
            "        concat30_out1",
        );

        std::fs::write(&gen_file, patched).expect("Failed to patch yolo26n-pose-nonms.rs");
    }
}

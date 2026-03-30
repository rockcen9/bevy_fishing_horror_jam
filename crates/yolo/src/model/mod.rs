#[cfg(feature = "backend")]
pub mod hand_model {
    extern crate alloc;
    include!(concat!(env!("OUT_DIR"), "/model/yolo26n-pose-nonms.rs"));
}

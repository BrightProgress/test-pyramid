pub trait Analysis {
    fn efficiency(&self) -> f64;
    fn defect_detection_capacity(&self) -> f64;
    fn defect_localization_capacity(&self) -> f64;
}
use rand::seq::IteratorRandom;
use rand::Rng;
use crate::system::System;
use crate::analysis::Analysis;

pub fn calculate_metrics(
    num_sources: u16,
    num_tests: u16,
    single_source_percentage: f64,
    system_tests_percentage: f64,
) -> Vec<(f64, f64, f64)> {
    let mut rng = rand::thread_rng();
    let mut metrics = Vec::new();
    let mut consecutive_count = 0;
    let mut prev_metrics = ((f64::MAX, f64::MAX, f64::MAX), (0.0, 0.0, 0.0));

    // Precompute the number of single source tests and system tests
    let num_single_source_tests = ((num_tests as f64 * single_source_percentage).ceil()) as u16;
    let num_system_tests = ((num_tests as f64 * system_tests_percentage).ceil()) as u16;

    for i in 0..100 {
        let mut system = System::new(num_sources, num_tests);
        let mut test_index = 0;

        // Add system tests
        for _ in 0..num_system_tests {
            let srcs: Vec<u16> = (0..num_sources).collect();
            system.add_has_test_for(test_index, &srcs);
            test_index += 1;
        }

        // Generate single source tests
        for _ in 0..num_single_source_tests {
            let src = rng.gen_range(0..num_sources);
            system.add_has_test_for(test_index, &[src]);
            test_index += 1;
        }

        // Generate tests that test a randomly selected set of sources
        for _ in test_index..num_tests {
            let num_srcs = rng.gen_range(1..=num_sources);
            let srcs: Vec<u16> = (0..num_sources).choose_multiple(&mut rng, num_srcs as usize);
            system.add_has_test_for(test_index, &srcs);
            test_index += 1;
        }

        // Calculate metrics
        let efficiency = system.efficiency() as f64;
        let defect_detection_capacity = system.defect_detection_capacity() as f64;
        let defect_localization_capacity = system.defect_localization_capacity();

        metrics.push((efficiency, defect_detection_capacity, defect_localization_capacity));

        // Print progress information
        println!(
            "Instance {}: Efficiency = {:.2}, Defect Detection Capacity = {:.2}, Defect Localization Capacity = {:.2}",
            i + 1,
            efficiency,
            defect_detection_capacity,
            defect_localization_capacity
        );

        // Check for saturation
        let (lower_bounds, upper_bounds) = prev_metrics;
        if efficiency >= lower_bounds.0
            && efficiency <= upper_bounds.0
            && defect_detection_capacity >= lower_bounds.1
            && defect_detection_capacity <= upper_bounds.1
            && defect_localization_capacity >= lower_bounds.2
            && defect_localization_capacity <= upper_bounds.2
        {
            consecutive_count += 1;
        } else {
            consecutive_count = 0;
        }

        if consecutive_count >= 10 {
            break;
        }

        // Update bounds
        prev_metrics = (
            (
                lower_bounds.0.min(efficiency),
                lower_bounds.1.min(defect_detection_capacity),
                lower_bounds.2.min(defect_localization_capacity),
            ),
            (
                upper_bounds.0.max(efficiency),
                upper_bounds.1.max(defect_detection_capacity),
                upper_bounds.2.max(defect_localization_capacity),
            ),
        );
    }

    // Print the ranges of the metrics
    let (lower_bounds, upper_bounds) = prev_metrics;
    println!(
        "Efficiency range: {:.2} - {:.2}",
        lower_bounds.0, upper_bounds.0
    );
    println!(
        "Defect Detection Capacity range: {:.2} - {:.2}",
        lower_bounds.1, upper_bounds.1
    );
    println!(
        "Defect Localization Capacity range: {:.2} - {:.2}",
        lower_bounds.2, upper_bounds.2
    );

    metrics
}
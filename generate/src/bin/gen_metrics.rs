use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use test_pyramid::metrics::calculate_metrics;

fn calculate_mean_and_std_dev(values: &[f64]) -> (f64, f64) {
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values.iter().map(|value| {
        let diff = mean - *value;
        diff * diff
    }).sum::<f64>() / values.len() as f64;
    let std_dev = variance.sqrt();
    (mean, std_dev)
}

fn main() {
    // Parse input parameters
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} <num_tests> <num_sources> <single_source_percentage> <system_tests_percentage>", args[0]);
        return;
    }

    let num_tests: u16 = args[1].parse().expect("Invalid number of tests");
    let num_sources: u16 = args[2].parse().expect("Invalid number of sources");
    let single_source_percentage: f64 = args[3].parse().expect("Invalid percentage of single source tests");
    let system_tests_percentage: f64 = args[4].parse().expect("Invalid percentage of system tests");

    // Input validation check
    if single_source_percentage + system_tests_percentage > 1.0 {
        eprintln!("The sum of single source percentage and system tests percentage must be less than or equal to 1.");
        return;
    }

    // Create a directory with a timestamp suffix
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let timestamp = since_the_epoch.as_secs();
    let dir_name = format!("metrics_{}", timestamp);
    fs::create_dir(&dir_name).expect("Unable to create directory");

    // Calculate metrics
    let metrics = calculate_metrics(num_sources, num_tests, single_source_percentage, system_tests_percentage);

    // Calculate mean and standard deviation
    let efficiencies: Vec<f64> = metrics.iter().map(|(efficiency, _, _)| *efficiency).collect();
    let defect_detections: Vec<f64> = metrics.iter().map(|(_, defect_detection, _)| *defect_detection).collect();
    let defect_localizations: Vec<f64> = metrics.iter().map(|(_, _, defect_localization)| *defect_localization).collect();

    let (mean_efficiency, std_dev_efficiency) = calculate_mean_and_std_dev(&efficiencies);
    let (mean_defect_detection, std_dev_defect_detection) = calculate_mean_and_std_dev(&defect_detections);
    let (mean_defect_localization, std_dev_defect_localization) = calculate_mean_and_std_dev(&defect_localizations);

    // Output metrics to CSV file
    let metrics_file_name = format!("{}/metrics.csv", dir_name);
    let mut metrics_file = File::create(&metrics_file_name).expect("Unable to create metrics file");
    writeln!(metrics_file, "num_tests,num_sources,single_source_percentage,system_tests_percentage,efficiency,defect_detection_capacity,defect_localization_capacity").expect("Unable to write to metrics file");

    for (efficiency, defect_detection_capacity, defect_localization_capacity) in metrics {
        writeln!(
            metrics_file,
            "{},{},{},{},{},{},{}",
            num_tests,
            num_sources,
            single_source_percentage,
            system_tests_percentage,
            efficiency,
            defect_detection_capacity,
            defect_localization_capacity
        )
        .expect("Unable to write to metrics file");
    }

    // Output mean and standard deviation to CSV file
    let summary_file_name = format!("{}/summary.csv", dir_name);
    let mut summary_file = File::create(&summary_file_name).expect("Unable to create summary file");
    writeln!(summary_file, "metric,mean,std_dev").expect("Unable to write to summary file");
    writeln!(summary_file, "efficiency,{:.6},{:.6}", mean_efficiency, std_dev_efficiency).expect("Unable to write to summary file");
    writeln!(summary_file, "defect_detection,{:.6},{:.6}", mean_defect_detection, std_dev_defect_detection).expect("Unable to write to summary file");
    writeln!(summary_file, "defect_localization,{:.6},{:.6}", mean_defect_localization, std_dev_defect_localization).expect("Unable to write to summary file");
}

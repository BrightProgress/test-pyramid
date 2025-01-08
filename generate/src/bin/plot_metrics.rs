use std::fs::File;
use std::io::Write;
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
    let mut metrics_file = File::create("plot_metrics.csv").expect("Unable to create metrics file");
    writeln!(metrics_file, "src_size,test_size,unit_test_percentage,system_test_percentage,mean_efficiency,min_efficiency,max_efficiency,std_dev_efficiency,mean_defect_detection,min_defect_detection,max_defect_detection,std_dev_defect_detection,mean_localization,min_localization,max_localization,std_dev_localization").expect("Unable to write to metrics file");

    for src_size in 5..=15 {
        for test_size in (5..=(5 * src_size)).step_by(5) {
            for unit_test_percentage in (0..=80).step_by(2) {
                for system_test_percentage_2 in 0..=40 {
                    let unit_test_percentage = unit_test_percentage as f64 / 100.0;
                    let system_test_percentage = system_test_percentage_2 as f64 / (2.0 * 100.0);

                    // Input validation check
                    if unit_test_percentage + system_test_percentage >= 1.0 {
                        continue;
                    }

                    let metrics = calculate_metrics(src_size, test_size, unit_test_percentage, system_test_percentage);

                    let efficiencies: Vec<f64> = metrics.iter().map(|(efficiency, _, _)| *efficiency).collect();
                    let defect_detections: Vec<f64> = metrics.iter().map(|(_, defect_detection, _)| *defect_detection).collect();
                    let defect_localizations: Vec<f64> = metrics.iter().map(|(_, _, defect_localization)| *defect_localization).collect();

                    let (mean_efficiency, std_dev_efficiency) = calculate_mean_and_std_dev(&efficiencies);
                    let min_efficiency = efficiencies.iter().cloned().fold(f64::INFINITY, f64::min);
                    let max_efficiency = efficiencies.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

                    let (mean_defect_detection, std_dev_defect_detection) = calculate_mean_and_std_dev(&defect_detections);
                    let min_defect_detection = defect_detections.iter().cloned().fold(f64::INFINITY, f64::min);
                    let max_defect_detection = defect_detections.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

                    let (mean_localization, std_dev_localization) = calculate_mean_and_std_dev(&defect_localizations);
                    let min_localization = defect_localizations.iter().cloned().fold(f64::INFINITY, f64::min);
                    let max_localization = defect_localizations.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

                    writeln!(
                        metrics_file,
                        "{},{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2}",
                        src_size,
                        test_size,
                        unit_test_percentage * 100.0,
                        system_test_percentage * 100.0,
                        mean_efficiency,
                        min_efficiency,
                        max_efficiency,
                        std_dev_efficiency,
                        mean_defect_detection,
                        min_defect_detection,
                        max_defect_detection,
                        std_dev_defect_detection,
                        mean_localization,
                        min_localization,
                        max_localization,
                        std_dev_localization
                    ).expect("Unable to write to metrics file");
                }
            }
        }
    }
}
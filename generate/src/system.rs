use std::collections::BTreeSet;
use itertools::Itertools;
use crate::analysis::Analysis;

#[derive(Debug)]
pub struct System {
    pub num_sources: u16,
    pub num_tests: u16,
    test_map: Vec<Option<BTreeSet<u16>>>,
}

impl System {
    pub fn new(num_sources: u16, num_tests: u16) -> System {
        System {
            num_sources,
            num_tests,
            test_map: vec![None; num_tests as usize],
        }
    }

    pub fn add_has_test_for(self: &mut Self, test: u16, srcs: &[u16]) {
        #[cfg(debug_assertions)]
        if !srcs.iter().all(|&x| x < self.num_sources) {
            panic!("Invalid has-test-for mapping: items are not sources");
        }

        #[cfg(debug_assertions)]
        if test >= self.num_tests {
            panic!("Invalid has-test-for mapping: item {} is not a test", test);
        }

        let tested_srcs = self.test_map[test as usize].get_or_insert_with(BTreeSet::new);
        for &new_src in srcs {
            tested_srcs.insert(new_src);
        }
    }

    pub fn is_tested_by(self: &Self, test: u16) -> BTreeSet<u16> {
        #[cfg(debug_assertions)]
        if test >= self.num_tests {
            panic!("Invalid has-test-for mapping: item {} is not a test", test);
        }

        self.test_map[test as usize].clone().unwrap_or_default()
    }
}

impl Analysis for System {
    fn efficiency(&self) -> f64 {
        let total_tests: usize = self.test_map.iter().filter_map(|opt| opt.as_ref().map(|srcs| srcs.len())).sum();
        let max_efficiency = self.num_tests as f64 * self.num_sources as f64;
        1.0 - (total_tests as f64 / max_efficiency)
    }

    fn defect_detection_capacity(&self) -> f64 {
        let mut capacity = 0;
        let sources: Vec<u16> = (0..self.num_sources).collect();

        for subset in (1..=self.num_sources).flat_map(|size| sources.iter().cloned().combinations(size as usize)) {
            let subset_set: BTreeSet<u16> = subset.into_iter().collect();
            let count = self.test_map.iter().filter_map(|opt| opt.as_ref()).filter(|tested_srcs| subset_set.is_subset(tested_srcs)).count();
            capacity += count;
        }

        let max_capacity = ((1 << self.num_sources) - 1) as f64 * self.num_tests as f64;
        capacity as f64 / max_capacity
    }

    fn defect_localization_capacity(&self) -> f64 {
        let mut capacity = 0.0;
        let sources: Vec<u16> = (0..self.num_sources).collect();
        let max_capacity = ((1 << self.num_sources) - 1)as f64;

        // Local function to calculate l_a
        fn l_a(test_map: &[Option<BTreeSet<u16>>], a: &BTreeSet<u16>) -> BTreeSet<u16> {
            let mut l_a = BTreeSet::new();
            let mut first = true;

            for tested_srcs in test_map.iter().filter_map(|opt| opt.as_ref()) {
                if a.is_subset(tested_srcs) {
                    if first {
                        l_a = tested_srcs.clone();
                        first = false;
                    } else {
                        l_a = l_a.intersection(tested_srcs).cloned().collect();
                    }
                }
            }

            l_a
        }

        for subset in (1..=self.num_sources).flat_map(|size| sources.iter().cloned().combinations(size as usize)) {
            let subset_set: BTreeSet<u16> = subset.into_iter().collect();
            let defect_size = subset_set.len();
            let l_a = l_a(&self.test_map, &subset_set);
            let localized_size;
            if !l_a.is_empty() {
                localized_size = l_a.len();
                capacity += (localized_size as f64 - defect_size as f64) / localized_size as f64;
            } else {
                // no tests for this sub-system - assume no localization
                capacity += 1.0;
            }           
        }

        1.0 - (capacity/max_capacity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::Analysis;

    #[test]
    fn test_add_has_tests_for() {
        let mut system = System::new(10, 10);
        system.add_has_test_for(1, &[1, 2, 3]);
        assert_eq!(system.is_tested_by(1), [1, 2, 3].iter().cloned().collect());
    }

    #[test]
    fn test_efficiency() {
        let mut system = System::new(3, 3);
        system.add_has_test_for(0, &[0, 1]);
        system.add_has_test_for(1, &[1, 2]);
        system.add_has_test_for(2, &[0, 2]);

        let max_efficiency = 3.0 * 8.0; // num_tests * 2^num_sources
        let expected_efficiency = 1.0 - (6.0 / max_efficiency);
        assert!((system.efficiency() - expected_efficiency).abs() < 1e-6);
    }

    #[test]
    fn test_defect_detection_capacity() {
        let mut system = System::new(3, 3);
        system.add_has_test_for(0, &[0, 1]);
        system.add_has_test_for(1, &[1, 2]);
        system.add_has_test_for(2, &[0, 2]);
        assert_eq!(system.defect_detection_capacity(), 9.0 / (7.0 * 3.0) as f64);
    }

    #[test]
    fn test_defect_localization_capacity() {
        let mut system = System::new(3, 3);
        system.add_has_test_for(0, &[0, 1]);
        system.add_has_test_for(1, &[1, 2]);
        system.add_has_test_for(2, &[0, 2]);
        assert_eq!(system.defect_localization_capacity(), 1.0 - (1.0 / 7.0) as f64);
    }
}

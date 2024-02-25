use std::collections::HashMap;

use crate::lib::exact_cover::{ExactCoverProblem, ExactCoverSolution};

/**
 * A basic example problem which can be solved with exact cover.
 */
pub(crate) struct BasicExampleProblem<'a> {
    required_items: Vec<&'a str>,
    optional_items: Vec<&'a str>,
    options: Vec<&'a str>,
}

/**
 * A basic example solution.
 */
pub(crate) struct BasicExampleSolution {
    selected_options: Vec<String>,
}

/**
 * Convert a basic example problem to an exact cover problem.
 */
fn convert_to_exact_cover_problem<'a>(basic_example: &'a BasicExampleProblem<'a>) -> ExactCoverProblem {
    let mut covered_by: HashMap<String, Vec<String>> = HashMap::new();

    let mut required_items: Vec<String> = Vec::new();
    for item in &basic_example.required_items {
        covered_by.insert(item.to_string(), Vec::new());
        required_items.push(item.to_string());
    }
    for item in &basic_example.optional_items {
        covered_by.insert(item.to_string(), Vec::new());
    }

    for option_name in &basic_example.options {
        for item_name in get_items_which_can_be_covered_by_option(option_name) {
            covered_by.get_mut(item_name).unwrap().push(option_name.to_string());
        }
    }

    return ExactCoverProblem::new(required_items, vec![], covered_by);
}

fn get_items_which_can_be_covered_by_option(option_name: &str) -> Vec<&str> {
    return option_name.split("").filter(|item_name| item_name.len() > 0).collect();
}

/**
 * Convert an exact cover solution to a basic example solution.
 */
fn convert_to_basic_example_solution(solution: ExactCoverSolution) -> BasicExampleSolution {
    return BasicExampleSolution {
        selected_options: solution.selected_options.iter().map(|option| option.to_string()).collect(),
    };
}

/**
 * Solve a basic example problem with exact cover.
 */
pub(crate) fn solve_basic_example_with_exact_cover<'a>(basic_example_problem: &'a BasicExampleProblem<'a>) -> Option<BasicExampleSolution> {
    let exact_cover_problem = convert_to_exact_cover_problem(basic_example_problem);

    let solution = exact_cover_problem.solve();

    solution.map(convert_to_basic_example_solution)
}

#[cfg(test)]
mod tests {
    use crate::lib::test_utils::assert_eq_ignore_order;

    use super::*;

    fn enable_logging() {
        std::env::set_var("RUST_LOG", "info");
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_zero_items() {
        let basic_example = BasicExampleProblem {
            required_items: vec![],
            optional_items: vec![],
            options: vec![],
        };

        let solution = solve_basic_example_with_exact_cover(&basic_example);

        assert!(solution.is_some());
        let selected_options = solution.unwrap().selected_options;
        let right: Vec<&str> = vec![];
        assert_eq!(selected_options, right);
    }

    #[test]
    fn test_zero_options() {
        let basic_example = BasicExampleProblem {
            required_items: vec!["A", "B", "C", "D", "E", "F", "G"],
            optional_items: vec![],
            options: vec![],
        };

        let solution = solve_basic_example_with_exact_cover(&basic_example);

        assert!(solution.is_none());
    }

    #[test]
    fn test_only_optional_items_and_zero_options() {
        let basic_example = BasicExampleProblem {
            required_items: vec![],
            optional_items: vec!["A", "B", "C", "D", "E", "F", "G"],
            options: vec![],
        };

        let solution = solve_basic_example_with_exact_cover(&basic_example);

        assert!(solution.is_some());
        let selected_options = solution.unwrap().selected_options;
        let right: Vec<&str> = vec![];
        assert_eq!(selected_options, right);
    }

    #[test]
    fn test_one_item() {
        let basic_example = BasicExampleProblem {
            required_items: vec!["A"],
            optional_items: vec![],
            options: vec!["A"],
        };

        let solution = solve_basic_example_with_exact_cover(&basic_example);

        assert!(solution.is_some());
        let selected_options = solution.unwrap().selected_options;
        assert_eq!(selected_options, vec!["A"]);
    }

    #[test]
    fn test_choose_all_two_options() {
        let basic_example = BasicExampleProblem {
            required_items: vec!["A", "B"],
            optional_items: vec![],
            options: vec!["A", "B"],
        };

        let solution = solve_basic_example_with_exact_cover(&basic_example);

        assert!(solution.is_some());
        let selected_options = solution.unwrap().selected_options;
        assert_eq_ignore_order(&selected_options, &vec!["A".to_string(), "B".to_string()]);
    }

    #[test]
    fn test_choose_two_of_three_options_for_three_items() {
        let basic_example = BasicExampleProblem {
            required_items: vec!["A", "B", "C"],
            optional_items: vec![],
            options: vec!["AB", "AC", "C"],
        };

        let solution = solve_basic_example_with_exact_cover(&basic_example);

        assert!(solution.is_some());
        let selected_options = solution.unwrap().selected_options;
        assert_eq_ignore_order(&selected_options, &vec!["AB".to_string(), "C".to_string()])
    }

    #[test]
    fn test_no_solution_for_three_items() {
        let basic_example = BasicExampleProblem {
            required_items: vec!["A", "B", "C"],
            optional_items: vec![],
            options: vec!["AB", "BC", "AC"],
        };

        let solution = solve_basic_example_with_exact_cover(&basic_example);

        assert!(solution.is_none());
    }

    #[test]
    fn test_basic_example() {
        // Example from https://en.wikipedia.org/wiki/Exact_cover#Detailed_example
        let basic_example = BasicExampleProblem {
            required_items: vec!["1", "2", "3", "4", "5", "6", "7"],
            optional_items: vec![],
            options: vec![
                "147",
                "14",
                "457",
                "356",
                "2367",
                "27",
            ],
        };

        let solution = solve_basic_example_with_exact_cover(&basic_example);

        assert!(solution.is_some());
        let selected_options = solution.unwrap().selected_options;
        assert_eq_ignore_order(&selected_options, &vec!["14".to_string(), "356".to_string(), "27".to_string()]);
    }

    #[test]
    fn test_basic_example_no_solution() {
        let basic_example = BasicExampleProblem {
            required_items: vec!["1", "2", "3", "4", "5", "6", "7"],
            optional_items: vec![],
            options: vec![
                "147",
                "14",
                "457",
                "356",
                "2367",
                "26",
            ],
        };

        let solution = solve_basic_example_with_exact_cover(&basic_example);

        assert!(solution.is_none());
    }

    #[test]
    fn test_knuth_basic_example() {
        let basic_example = BasicExampleProblem {
            required_items: vec!["A", "B", "C", "D", "E", "F", "G"],
            optional_items: vec![],
            options: vec![
                "CEF",
                "ADG",
                "BCF",
                "AD",
                "BG",
                "DEG",
            ],
        };

        let solution = solve_basic_example_with_exact_cover(&basic_example);

        assert!(solution.is_some());
        let selected_options = solution.unwrap().selected_options;
        assert_eq_ignore_order(&selected_options, &vec!["CEF".to_string(), "AD".to_string(), "BG".to_string()]);
    }

    #[test]
    fn test_knuth_basic_example_with_optional_h() {
        let basic_example = BasicExampleProblem {
            required_items: vec!["A", "B", "C", "D", "E", "F", "G"],
            optional_items: vec!["H"],
            options: vec![
                "CEF",
                "ADG",
                "BCF",
                "AD",
                "BG",
                "DEG",
            ],
        };

        let solution = solve_basic_example_with_exact_cover(&basic_example);

        assert!(solution.is_some());
        let selected_options = solution.unwrap().selected_options;
        assert_eq_ignore_order(&selected_options, &vec!["CEF".to_string(), "AD".to_string(), "BG".to_string()]);
    }
}


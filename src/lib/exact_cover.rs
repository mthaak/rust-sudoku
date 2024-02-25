use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use log::info;
use priority_queue::PriorityQueue;

/**
 * An exact cover problem. See https://en.wikipedia.org/wiki/Exact_cover.
 */
pub struct ExactCoverProblem<'a> {
    /// Map from item name to option names
    covered_by: HashMap<&'a str, Vec<&'a str>>,
    /// Map from option name to item names
    covers: HashMap<&'a str, Vec<&'a str>>,
    /// The items that must be covered
    required_items: HashSet<&'a str>,
    /// The options that must be selected as part of the solution
    required_options: HashSet<&'a str>,

    // TODO these should probably be passed down to the recursive _solve method instead of being mutating fields
    /// Map from item name to the available options (i.e. those that haven't been removed)
    available_options: RefCell<HashMap<&'a str, RefCell<HashSet<&'a str>>>>,
    /// Priority queue of items, ordered by the smallest number of available options
    items_queue: RefCell<PriorityQueue<&'a str, i32>>,
    /// The selected options
    selected_options: RefCell<Vec<&'a str>>,
}

#[derive(Debug)]
pub struct ExactCoverSolution<'a> {
    /// The selected options
    pub(crate) selected_options: Vec<&'a str>,
}

struct ExactCoverResult<'a> {
    last_solution: Option<ExactCoverSolution<'a>>,
    num_solutions: u64,
}

impl<'a> ExactCoverProblem<'a> {
    /**
     * Create a new exact cover problem.
     */
    pub fn new(
        required_items: Vec<&'a str>,
        required_options: Vec<&'a str>,
        covered_by: HashMap<&'a str, Vec<&'a str>>) -> ExactCoverProblem<'a>
    {
        info!("Covered by: {:?}", covered_by);
        let mut covers: HashMap<&str, Vec<&str>> = HashMap::new();
        for (item_name, option_names) in covered_by.clone() {
            for option_name in option_names.iter() {
                if !covers.contains_key(option_name) {
                    covers.insert(option_name, Vec::new());
                }
                covers.get_mut(option_name).unwrap().push(item_name);
            }
        }

        let mut available_options: HashMap<&str, RefCell<HashSet<&str>>> = HashMap::new();
        for (item_name, option_names) in covered_by.clone() {
            available_options.insert(item_name, RefCell::new(HashSet::from_iter(option_names.clone())));
        }

        let mut items_queue = PriorityQueue::new();
        for item_name in required_items.iter() {
            let option_names = covered_by.get(item_name).unwrap();
            items_queue.push(*item_name, -(option_names.len() as i32));
        }
        let selected_options = Vec::new();

        let required_items = HashSet::from_iter(required_items.iter().cloned());
        let required_options = HashSet::from_iter(required_options.iter().cloned());

        ExactCoverProblem {
            covered_by,
            covers,
            required_items,
            required_options,
            available_options: RefCell::new(available_options),
            items_queue: RefCell::new(items_queue),
            selected_options: RefCell::new(selected_options),
        }
    }

    /**
     * Solve the exact cover problem.
     */
    pub fn solve(&'a self) -> Option<ExactCoverSolution<'a>> {
        self.select_required_options();
        let result = self._solve_until(1);
        return result.last_solution;
    }

    fn select_required_options(&'a self) {
        for option_name in self.required_options.iter() {
            self.select_option(option_name);
        }
    }

    /**
     * Solve the exact cover problem until the given number of solutions are found.
     */
    fn _solve_until(&'a self, remaining_solutions: i32) -> ExactCoverResult<'a> {
        if remaining_solutions <= 0 {
            return ExactCoverResult {
                last_solution: None,
                num_solutions: 0,
            };
        }

        info!("Items queue: {:?}", self.get_items_queue());
        info!("Available options: {:?}", self.get_available_options());
        let item_name_opt = self.select_new_item();
        return match item_name_opt {
            Some(item_name) => {
                info!("Selecting item {}", item_name);

                if self.available_options.borrow().get(item_name).unwrap().borrow().len() == 0 {
                    info!("Contradiction: item {} has no options left", item_name);
                    // Contradiction => return no solution found for selected option
                    self.return_item(item_name);
                    return ExactCoverResult {
                        last_solution: None,
                        num_solutions: 0,
                    };
                }

                let mut result = ExactCoverResult {
                    last_solution: None,
                    num_solutions: 0,
                };

                // TODO this clone might be inefficient but is only way I can think of to allow mutating the available_options
                //   while iterating over it
                let available_options = self.available_options.borrow().get(item_name).unwrap().borrow().clone();
                for option_name in available_options.iter() {
                    info!("Selecting option {}", option_name);
                    let removed_options = self.select_option(option_name);

                    let new_result = self._solve_until(remaining_solutions - result.num_solutions as i32);

                    if new_result.num_solutions == 0 {
                        info!("No solution found for option {}", option_name);
                    } else {
                        result.last_solution = result.last_solution.or(new_result.last_solution);
                        result.num_solutions += new_result.num_solutions;
                    }

                    info!("Unselecting option {}", option_name);
                    self.unselect_option(option_name, removed_options) // backtrack
                }

                if result.num_solutions == 0 {
                    info!("No solution found for item {}", item_name);
                }

                result
            }

            None => {
                // No more item left => solution found
                info!("No more items left. Solution found: {:?}", self.selected_options.borrow());
                ExactCoverResult {
                    last_solution: Some(ExactCoverSolution {
                        selected_options: self.selected_options.clone().into_inner().clone(),
                    }),
                    num_solutions: 1,
                }
            }
        };
    }

    /**
     * Count all solutions to the exact cover problem.
     */
    pub fn count_all_solutions(&'a self) -> u64 {
        self.select_required_options();
        let result = self._solve_until(i32::MAX);
        return result.num_solutions;
    }

    /**
     * Select a new item from the items queue.
     */
    fn select_new_item(&self) -> Option<&str> {
        return self.items_queue.borrow_mut().pop().map(|(item_name, _)| item_name);
    }

    /**
     * Select an option.
     */
    fn select_option(&'a self, option_name: &'a str) -> Vec<&str> {
        self.selected_options.borrow_mut().push(option_name);

        let mut removed_options: Vec<&str> = Vec::new();
        // For each item that this option covers ...
        self.covers.get(option_name).unwrap().iter()
            .for_each(|item_name| {
                // ... remove it from the items queue ...
                info!("Removing item {}", item_name);
                self.remove_item(item_name);

                // ... and make all its options unavailable because only one option can be selected per item
                let available_options = self.available_options.borrow().get(item_name).unwrap().borrow().clone();
                available_options.iter()
                    .for_each(|other_option_name| {
                        info!("Removing option {}", other_option_name);
                        self.remove_option(other_option_name);
                        removed_options.push(other_option_name);
                    });
            });
        return removed_options;
    }

    /**
     * Unselect an option (essentially perform the inverse of select_option).
     */
    fn unselect_option(&'a self, option_name: &'a str, removed_options: Vec<&'a str>) {
        let removed_options_set = removed_options.iter().collect::<HashSet<_>>();

        // For each item that this option covers ...
        self.covers.get(option_name).unwrap().iter()
            .for_each(|item_name| {
                // ... make all its options available again which were removed...
                self.covered_by.get(item_name).unwrap().iter()
                    .for_each(|other_option_name| {
                        if removed_options_set.contains(other_option_name) {
                            info!("Returning option {}", other_option_name);
                            self.return_option(other_option_name);
                        }
                    });

                if self.required_items.contains(item_name) {
                    // ... and return it to the items queue if it's required ...
                    info!("Returning item {}", item_name);
                    self.return_item(item_name);
                }
            });

        self.selected_options.borrow_mut().pop();
    }

    /**
     * Remove an item from the items queue.
     */
    fn remove_item(&'a self, item_name: &'a str) {
        self.items_queue.borrow_mut().remove(item_name);
    }

    /**
     * Remove an option from the available options of all items that it covers.
     */
    fn remove_option(&'a self, option_name: &'a str) {
        // For each item that this option covers ...
        self.covers.get(option_name).unwrap().iter()
            .for_each(|item_name| {
                // ... remove the option from its available options ...
                self.available_options.borrow().get(item_name).unwrap().borrow_mut().remove(option_name);

                // ... and update priority of the item because it has one fewer option
                if self.required_items.contains(item_name) {
                    self.update_priority(item_name)
                }
            });
    }

    /**
     * Add an item to the items queue.
     */
    fn return_item(&'a self, item_name: &'a str) {
        self.items_queue.borrow_mut().push(item_name, -(self.available_options.borrow().get(item_name).unwrap().borrow().len() as i32));
    }

    /**
     * Add an option to the available options of all items that it covers.
     */
    fn return_option(&'a self, option_name: &'a str) {
        // For each item that this option covers ...
        self.covers.get(option_name).unwrap().iter()
            .for_each(|item_name| {
                // ... add the option to its available options ...
                self.available_options.borrow().get(item_name).unwrap().borrow_mut().insert(option_name);

                // ... and update priority of the item because it has one more option
                if self.required_items.contains(item_name) {
                    self.update_priority(item_name)
                }
            });
    }

    /**
     * Update the priority of an item in the items queue.
     */
    fn update_priority(&'a self, item_name: &'a str) {
        self.items_queue.borrow_mut().change_priority(item_name, -(self.available_options.borrow().get(item_name).unwrap().borrow().len() as i32));
    }

    /**
     * Get the items queue.
     */
    fn get_items_queue(&self) -> Vec<&str> {
        return self.items_queue.borrow().clone().into_sorted_vec().iter().map(|item_name| *item_name).collect();
    }

    /**
     * Get the available options for each item.
     */
    fn get_available_options(&self) -> HashMap<&str, HashSet<&str>> {
        let mut available_options: HashMap<&str, HashSet<&str>> = HashMap::new();
        for (item_name, options) in self.available_options.borrow().iter() {
            available_options.insert(item_name, options.borrow().clone());
        }
        return available_options;
    }
}
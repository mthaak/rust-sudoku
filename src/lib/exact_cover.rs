use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use priority_queue::PriorityQueue;

/**
 * An exact cover problem. See https://en.wikipedia.org/wiki/Exact_cover.
 */
pub struct ExactCoverProblem<'a> {
    /// Map from item name to option names
    covered_by: HashMap<&'a str, Vec<&'a str>>,
    /// Map from option name to item names
    covers: HashMap<&'a str, Vec<&'a str>>,

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

impl<'a> ExactCoverProblem<'a> {
    /**
     * Create a new exact cover problem.
     */
    pub fn new(
        required_items: Vec<&'a str>,
        covered_by: HashMap<&'a str, Vec<&'a str>>) -> ExactCoverProblem<'a>
    {
        println!("covered_by: {:?}", covered_by);
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
        let mut items_queue = PriorityQueue::new();
        for item_name in required_items.iter() {
            let option_names = covered_by.get(item_name).unwrap();
            available_options.insert(item_name, RefCell::new(HashSet::from_iter(option_names.clone())));
            items_queue.push(item_name.clone(), -(option_names.len() as i32));
        }
        let selected_options = Vec::new();

        ExactCoverProblem {
            covered_by,
            covers,
            available_options: RefCell::new(available_options),
            items_queue: RefCell::new(items_queue),
            selected_options: RefCell::new(selected_options),
        }
    }

    /**
     * Solve the exact cover problem.
     */
    pub fn solve(&'a self) -> Option<ExactCoverSolution<'a>> {
        println!("Items: {:?}", self.items_queue);
        println!("Options: {:?}", self.available_options);
        let item_name_opt = self.select_new_item();
        return match item_name_opt {
            Some(item_name) => {
                println!("Selecting item {}", item_name);

                if self.available_options.borrow().get(item_name).unwrap().borrow().len() == 0 {
                    println!("Contradiction: item {} has no options left", item_name);
                    // Contradiction => return no solution found for selected option
                    return None;
                }

                // TODO this clone might be inefficient but is only way I can think of to allow mutating the available_options
                //   while iterating over it
                let available_options = self.available_options.borrow().get(item_name).unwrap().borrow().clone();
                for option_name in available_options.iter() {
                    println!("Selecting option {}", option_name);
                    self.select_option(option_name);

                    match self.solve() {
                        Some(solution) => {
                            // Solution found => return it
                            println!("Solution found: {:?}", solution);
                            return Some(solution); // TODO return multiple solutions if desired
                        }
                        None => {
                            println!("No solution found for option {}", option_name);
                            // No solution => backtrack and try next option
                        }
                    }

                    println!("Unselecting option {}", option_name);
                    self.unselect_option(option_name) // backtrack
                }

                // There is no valid option that covers item => return no solution found for selected option
                println!("No solution found for item {}", item_name);
                None
            }

            None => {
                // No more item left => solution found
                println!("No more items left. Solution found: {:?}", self.selected_options);
                Some(ExactCoverSolution {
                    selected_options: self.selected_options.clone().into_inner().clone(),
                })
            }
        };
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
    fn select_option(&'a self, option_name: &'a str) {
        self.selected_options.borrow_mut().push(option_name);

        // For each item that this option covers ...
        self.covers.get(option_name).unwrap().iter()
            .for_each(|item_name| {
                // ... remove it from the items queue ...
                println!("Removing item {}", item_name);
                self.remove_item(item_name);

                // ... and make all its options unavailable because only one option can be selected per item
                self.covered_by.get(item_name).unwrap().iter()
                    .for_each(|other_option_name| {
                        println!("Removing option {}", other_option_name);
                        self.remove_option(other_option_name);
                    });
            });
    }

    /**
     * Unselect an option (essentially perform the inverse of select_option).
     */
    fn unselect_option(&'a self, option_name: &'a str) {
        // For each item that this option covers ...
        self.covers.get(option_name).unwrap().iter()
            .for_each(|item_name| {
                // ... make all its options available again ...
                self.covered_by.get(item_name).unwrap().iter()
                    .for_each(|other_option_name| {
                        self.return_option(other_option_name);
                    });

                // ... and add it to the items queue
                self.return_item(item_name)
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

                // ... and update priority of the item because it has one more option
                self.update_priority(item_name)
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
                self.update_priority(item_name)
            });
    }

    /**
     * Update the priority of an item in the items queue.
     */
    fn update_priority(&'a self, item_name: &'a str) {
        self.items_queue.borrow_mut().change_priority(item_name, -(self.available_options.borrow().get(item_name).unwrap().borrow().len() as i32));
    }
}
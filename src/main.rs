// Define the FilterCondition struct
struct FilterCondition<T> {
    condition: T,
}

impl<T: PartialEq> FilterCondition<T> {
    // Implement the is_match method to check if the item matches the condition
    fn is_match(&self, item: &T) -> bool {
        item == &self.condition
    }
}

// Define the custom_filter function
fn custom_filter<T>(collection: &Vec<T>, filter: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq + Clone,
{
    collection
        .iter()
        .filter(|item| filter.is_match(item))
        .cloned()
        .collect()
}

fn main() {
    // Create a collection (vector) with some elements
    let numbers = vec![1, 2, 3, 4, 5, 2, 6, 7];

    // Initialize a FilterCondition object with the desired value (e.g., filter for 2)
    let filter_condition = FilterCondition { condition: 2 };

    // Call the custom_filter function and store the result
    let filtered_numbers = custom_filter(&numbers, &filter_condition);

    // Print the filtered result to the console
    println!("Filtered Numbers: {:?}", filtered_numbers);
}

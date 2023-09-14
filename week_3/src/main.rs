struct FilterCondition {
    value: i32,
}

impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        *item == self.value
    }
}

fn custom_filter(collection: &[i32], condition: &FilterCondition) -> Vec<i32>
{
    collection
        .iter()
        .filter(|item| condition.is_match(item))
        .cloned()
        .collect()
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let filter_condition = FilterCondition { value: 3 };

    let filtered_data = custom_filter(&data, &filter_condition);

    println!("Filtered data: {:?}", filtered_data);
}

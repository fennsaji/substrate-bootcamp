fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}

fn main() {
    // Initialize two String variables
    let string1 = String::from("Hello, ");
    let string2 = String::from("Rise In!");

    // Call the concatenate_strings function with references
    let concatenated_string = concatenate_strings(&string1, &string2);

    // Print the result
    println!("{}", concatenated_string);
}
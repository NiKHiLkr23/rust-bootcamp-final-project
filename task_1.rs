//  Write the concatenate_strings function signature.
fn concatenate_strings(str1: &str, str2: &str) -> String {
    // Implement the concatenate_strings function.
    let mut result = String::new(); // Creating a new String to store the result.
    result.push_str(str1); // Appending the contents of the first input string slice.
    result.push_str(str2); // Appending the contents of the second input string slice.
    result // Returning the result string.
}

fn main() {
    //  Initialize two String variables in the main function.
    let string1 = String::from("Getting Started with ");
    let string2 = String::from("Rust!");

    //  Call the concatenate_strings function with string slices of the variables.
    let concatenated_string = concatenate_strings(&string1, &string2);

    //  Print the result to the console.
    println!("{}", concatenated_string);
}

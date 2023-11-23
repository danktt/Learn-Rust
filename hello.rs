use std::any::type_name;

// Helper function to get the type name
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}



fn main() {
  println!("First time using rust");


  // So when I want to create a new variable, I need to create with snake_case!!

  let mut change_name = 1; 
    println!("The type of 'change_name' is: {}", type_of(change_name));

    // Attempting to assign a string to a variable originally declared as an integer
    // This will result in a compilation error.
    // change_name = "Danilo Miranda";

    // Uncommenting the line above will result in a compilation error,
    // as you cannot assign a string to a variable that was originally an integer.

    // If you want to create a new variable with a different type, you need to use a new variable name.

    // Example:
    let new_variable = "Danilo Miranda";
    println!("The type of 'new_variable' is: {}", type_of(new_variable));
}


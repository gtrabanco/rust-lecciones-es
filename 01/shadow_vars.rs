fn main() {
    // Shadow var is a redefinition of a variable
    // It will point to a new memory address but
    // the old is not deleted and can not be
    // referenced but the new definition point to
    // it.
    let number = 10;
    let number = number * 2;
    let number = number + 2;
    println!("The number is {}", number);
}
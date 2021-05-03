fn main() {
    let var_number = 10;
    let var_boolean = true;

    println!("The number is {}", var_number);
    println!("The boolean is {}", var_boolean);

    // If you try to re-assign a value to any defined
    // variable, you can't because they are inmutable
    // uncomment to see the error:
    //   error: cannot assign twice to immutable variable `var_number`
    // var_number = 15;
    // To make a function mutable need to define it as mutable
    let mut var_mut_no = 5;
    println!("Mutable value is {}", var_mut_no);

    var_mut_no = 10;
    println!("Mutable value is {}", var_mut_no);
}
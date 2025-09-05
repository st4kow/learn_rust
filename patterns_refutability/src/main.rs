fn main() {
    // Patterns that are match always are irrefutable for example: let x = 5;
    // Patterns that can fail to match for ceration values are refutable for example if let Some(x) = x

    // Function parameters, let statements, for loops can acceptt only  iffefutable patters

    // let Some(x) : Option<i32> = None; // Compile error
    // Let statement can accept only irrefutable patterns

    // Possible solution, be explicit and deal with the other possible values
    let some_option_value: Option<i32> = Some(5);
    let Some(x) = some_option_value else { 
        return;
    };

    // This produces warning, because else never reached. "irrrefutable let .. else pattern"
    let x = 5 else {
        return;
    };
}

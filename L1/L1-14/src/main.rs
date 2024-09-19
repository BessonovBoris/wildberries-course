use std::any::{Any, TypeId};

fn print_type(value: &dyn Any) {
    // get id of given type
    let type_id = value.type_id();

    // check if it is known type
    if type_id == TypeId::of::<i32>() {
        println!("The value is an i32.");
    } else if type_id == TypeId::of::<String>() {
        println!("The value is a String.");
    } else {
        println!("The value is other type - {:?}", type_id);
    }
}

fn main() {
    let x = 5;
    let y = "String".to_string();

    print_type(&x);
    print_type(&y);

    let z = vec![1, 2, 3];
    print_type(&z);
}
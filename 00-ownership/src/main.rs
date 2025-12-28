fn main() {
    let my_name = String::from("Maysam");
    println!("(L-3) my name is {my_name}");

    let new_variable= my_name;
    // my_name.push_str("hi_name"); // can't use my_name here!, value moved in line 5
    // println!("{my_name}");       // can't use my_name here!, value moved in line 5
    println!("(L-8) my name from new variable: {new_variable}");



    let my_name = String::from("Maysam");
    let _new_variable = my_name.clone();

    println!("(L-15) I can use my_name here: {my_name}");
}

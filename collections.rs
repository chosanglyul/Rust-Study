/*
fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(5);
}


fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}
*/

fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}


// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

fn main() {
    println!("Running option 1");
    option1();
    println!("Running option 2");
    option2();
    println!("Running option 3");
    option3();
}

// TODO Make another, separate version of the data that's in `vec0` and pass that to `fill_vec` instead.
// DONT KNOW HOW
fn option1() {
/*
    let vec0 = Vec::new();

    let mut vec1 = option1_fill_vec(vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
*/
}

fn option1_fill_vec(vec: Vec<i32>) -> Vec<i32> {
    /*
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);
    */

    vec
}

// Make `fill_vec` borrow its argument instead of taking ownership of it,
//   and then copy the data within the function in order to return an owned
//   `Vec<i32>`
fn option2() {
    let vec0 = Vec::new();

    let mut vec1 = option2_fill_vec(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn option2_fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec1 = Vec::new();

    vec1.push(22);
    vec1.push(44);
    vec1.push(66);

    vec1
}

// Make `fill_vec` *mutably* borrow a reference to its argument (which will need to be
// mutable), modify it directly, then not return anything. Then you can get rid
// of `vec1` entirely -- note that this will change what gets printed by the
// first `println!`
fn option3() {
    let mut vec0 = Vec::new();
    let vec1 = option3_fill_vec(&mut vec0);
    {
        let vec0 = &vec1;
        println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    }
    vec1.push(88);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn option3_fill_vec(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    let vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

/*
fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
*/
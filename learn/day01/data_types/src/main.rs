fn main() {
    let num = 42;

    let float_num = 42.0;

    let difference = 42 - 2;

    let heart_eyed_cat = '😻';

    println!("{}", heart_eyed_cat);

    let tup = (500, 4.2, '😻');
    let (x, y, z) = tup;
    println!("Z is {}", z);

    println!("Tup one: {}", tup.0);

    let arr = [1, 2, 3, 4, 5];
    println!("The first array element is {}", arr[0]);
}

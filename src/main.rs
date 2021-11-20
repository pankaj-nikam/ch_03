fn main() {
    println!("Compound Data Types");
    //arrays();
    //multidimensional_array();
    tuples();
}

fn arrays() {
    let mut letters = ['a', 'b', 'c'];
    println!("The first letter is {}", letters[0]);
    letters[0] = 'x';
    println!("The first letter is {}", letters[0]);

    let numbers: [i32; 5];
    // numbers = [0, 0, 0, 0, 0];
    numbers = [0; 5];
    println!("The last item is {}", numbers[4]);
}

fn tuples() {
    let mut stuff = (10, 3.14, 'x');
    println!("first item is {}", stuff.0);
    stuff.0 += 3;
    println!("first item is {}", stuff.0);
    println!("entire tuple is {:?}", stuff);
    let (a, b, c) = stuff;
    println!("a is {} b is {} and c is {}", a, b, c);
}

fn multidimensional_array() {
    let parking_lot = [[1, 2, 3], [4, 5, 6]];

    let number = parking_lot[0][1];

    println!("The spot is {}", number);

    //This is a three dimensional array
    let garage = [[[0; 100]; 20]; 5];
    println!("Garage is {:?}", garage);
}

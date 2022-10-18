fn main() {

    let b: bool = 1f32 < 3.0;
    println!("Hello, world!, {}", b);

    let food = "cookie";
    if food == "cookie" {
        println!("yes cookie");
    } else {
        println!("yes NOT cookie");

    }

    let food = "NOT cookie";
    if food == "cookie" {
        println!("yes cookie");
    } else {
        println!("too bad NOT cookie");

    }

}

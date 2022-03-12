#[path = "bubble_sort/bubble_sort.rs"]
mod bubble_sort;

fn main() {
    let mut array = [3, 2, 4, 6, 5, 1];

    bubble_sort::sort(&mut array);

    println!("{:?} ", array);
}

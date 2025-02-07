mod basket;
mod container;
mod stack;

use basket::Basket;
use container::Container;
use stack::Stack;

fn add_string_to_container<T: Container<String>>(container: &mut T, item: String) {
    container.put(item);
}

fn main() {
    let mut b1 = Basket::new(String::from("apple"));
    let b2 = Basket::new(10);
    let b3 = Basket::new(true);

    let mut s1 = Stack::new(vec![String::from("apple")]);
    let s2 = Stack::new(vec![1, 2, 3]);

    add_string_to_container(&mut b1, String::from("banana"));
    add_string_to_container(&mut s1, String::from("banana"));
}

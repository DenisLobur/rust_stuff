pub fn find_biggest_number() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

pub fn biggest_integer(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn biggest_generic<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub struct Point<T, U> {
    pub x: T,
    pub y: U,
}

impl<T, U> Point<T, U> {
    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &U {
        &self.y
    }
}
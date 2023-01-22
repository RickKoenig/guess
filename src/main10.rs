pub fn main10() {
    {
        let number_list = vec![34, 50, 25, 100, 65];
        let mut largest = number_list[0];
        for number in number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);
    }
    {
        fn largest<T>(list: &[T]) -> T {
            let mut largest = list[0];

            for &item in list.iter() {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }
        let number_list = vec![34, 50, 25, 100, 65];
    }
}

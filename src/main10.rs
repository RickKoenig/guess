#![allow(dead_code)]
#![allow(unused_variables)]

fn largest_int(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
/*
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
        //if true {
            largest = item;
        }
    }
    largest
}
*/
pub fn main10() {
    {
        let int_list = vec![34, 50, 25, 100, 65];
        let result = largest_int(&int_list);
        println!("The largest i32 is {}", result);

        let char_list = vec!['h', 'e', 'w', 'w', 'o', 'a'];
        let result = largest_char(&char_list);
        println!("The largest char is {}", result);

        let number_list = vec![33, 55, 44, 22, 11];
        //let result = largest(&number_list);
        //println!("The largest int is {}", result);
    }

    {
        struct Point<T> {
            x: T,
            y: T,
        }

        struct PointG<T, U> {
            x: T,
            y: U,
        }

        impl<T, U> PointG<T, U> {
            fn mixup<V, W>(self, other: PointG<V, W>) -> PointG<T, W> {
                PointG {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        struct PointM<T> {
            x: T,
            y: T,
        }

        impl<T> PointM<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }

        impl PointM<f32> {
            fn dist_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }

        let i32p = Point { x: 5, y: 10 };
        let f64p = Point { x: 5.5, y: 10.0 };
        let sp = Point { x: "hi", y: "ho" };
        let ssp = Point {
            x: String::from("hi"),
            y: String::from("ho"),
        };
        let intp = PointG { x: 5, y: 10.5 };
        let mp = PointM { x: 5f32, y: 12f32 };
        let f32p = Point {
            x: 5.5f32,
            y: 6.6f32,
        };
        let f32pt: Point<f64> = Point { x: 5.5, y: 6.6 };
        println!("p.x = {}", mp.x());
        let dist = mp.dist_from_origin();
        println!("dist from orig = {}", dist);

        let p1 = PointG { x: 5, y: 10.4 };
        let p2 = PointG { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);
        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

        let int = Some(5);
        let float = Some(5.0f32);
        let what: Option<u32> = None;
    }
    {
        trait Summary {
            //fn summarize(&self) -> String;

            fn summarize(&self) -> String {
                String::from("(Read more...)")
            } 
        }

        struct NewsArticle {
            headline: String,
            location: String,
            author: String,
            content: String,
        }

        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }

        struct Tweet {
            username: String,
            content: String,
            reply: bool,
            retweet: bool,
        }

        impl Summary for Tweet {
            /*
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            } 
            */
        }
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
        println!("1 new tweet: {}", tweet.summarize());

        let news_article = NewsArticle {
            headline: String::from("news"),
            location: String::from("loc"),
            author: String::from("auth"),
            content: String::from("contents")
        };
        println!("1 new newsArticle: {}", news_article.summarize());

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
        };
        println!("New article available! {}", article.summarize());
    }
}

#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Debug;
use std::fmt::Display;

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

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
        //if true {
            largest = item;
        }
    }
    largest
}

pub fn main10() {
    {
        let i32_list = vec![34, 50, 25, 100, 65];
        let result = largest_int(&i32_list);
        println!("The largest i32 is {}", result);

        let char_list = vec!['h', 'e', 'w', 'w', 'o', 'a'];
        let result = largest_char(&char_list);
        println!("The largest char is {}", result);

        let int_list = vec![33, 55, 44, 22, 11];
        let result = largest(&int_list);
        println!("The largest int is {}", result);

        
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

            //fn summarize_author(&self) ->String;

            fn summarize(&self) -> String {
                String::from("(Read more...)")
            }
            /*
            fn summarize(&self) -> String {
                format!("(Read more from {}...)",
                    self.summarize_author())
            } */
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

        fn returns_summarizable() -> impl Summary {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            }
        }

        /* won't work, can't return multiple types
        fn returns_summarizable2(switch: bool) -> impl Summary {
            if switch {
                NewsArticle {
                    headline: String::from("Penguins win the Stanley Cup Championship!"),
                    location: String::from("Pittsburgh, PA, USA"),
                    author: String::from("Iceburgh"),
                    content: String::from(
                        "The Pittsburgh Penguins once again are the best
                    hockey team in the NHL.",
                    ),
                }
            } else {
                Tweet {
                    username: String::from("horse_ebooks"),
                    content: String::from("of course, as you probably already know, people"),
                    reply: false,
                    retweet: false,
                }
            }
        }
        */

        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        struct Face {
            user: String,
            what: String,
        }

        impl Summary for Face {
            /*fn summarize(&self) -> String {
                format!("{}: {}", self.user, self.what)
            }*/
        }
        /*
                fn notify(item: impl Summary) {
                    println!("Breaking news! sugar {}", item.summarize());
                }
        */

        fn notify<T: Summary>(item: T) {
            println!("Breaking news! no sugar {}", item.summarize());
        }

        fn notify2(item1: impl Summary, item2: impl Summary) {}
        fn notify3<T: Summary>(item1: T, item2: T) {}

        fn notify4(item: impl Summary + Display) {}

        fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
            34
        }

        fn some_function2<T, U>(t: T, u: U) -> i32
        where
            T: Display + Clone,
            U: Clone + Debug,
        {
            444
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
            content: String::from("contents"),
        };
        println!("1 new newsArticle: {}", news_article.summarize());

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best
            hockey team in the NHL.",
            ),
        };
        println!("New article available! {}", article.summarize());

        let face = Face {
            user: String::from("myself"),
            what: String::from("news from face"),
        };

        println!("New face! {}", face.summarize());

        notify(tweet);
        notify(article);
        notify(face);
    }
}

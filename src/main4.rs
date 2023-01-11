struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rect {
    wid: u32,
    hit: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.wid * self.hit
    }

    fn can_hold(&self, rhs: &Rect) -> bool {
        self.wid >= rhs.wid && self.hit >= rhs.wid
    }

    fn square(size: u32) -> Rect {
        Rect {
            wid: size,
            hit: size,
        }
    }
}

pub fn main4() {
    // structs chapter 5
    println!("\nchapter 5, structs");

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    let user2 = build_user(String::from("a@b.com"), String::from("Rickers"));
    println!("User2 email = {}", user2.email);
    let user3 = User {
        email: String::from("c@d.com"),
        username: String::from("Dave"),
        //active: user2.active,
        //sign_in_count: user2.sign_in_count,
        ..user2
    };
    println!("User3 name = {}", user3.username);

    // tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(3, 4, 12);
    let origint = (13, 15, 17);
    //let (x, y, z) = origin; // doesn't work
    let x = origin.0;
    let y = origin.1;
    let z = origin.2;
    //let w = origin.3;
    let (a, b, c) = origint;

    // rectangle
    {
        let rect1 = Rect { wid: 3, hit: 4 };
        let a = area(&rect1);
        println!("area = {}", a);
        let a2 = rect1.area();
        let a2 = Rect::area(&rect1); // works too
        println!("area2 = {}", a2);
        println!("dim = ({}, {})", rect1.wid, rect1.hit);
        println!("rect1 = '{:#?}'", rect1);
        println!("rect1 = '{:?}'", rect1);
    }
    {
        let rect1 = Rect { wid: 30, hit: 50 };
        let rect2 = Rect { wid: 10, hit: 40 };
        let rect3 = Rect { wid: 60, hit: 45 };

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
        let rsq = Rect::square(11);
        println!("area of sq = {}", rsq.area());
    }

    // methods
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(r: &Rect) -> u32 {
    r.wid * r.hit
}

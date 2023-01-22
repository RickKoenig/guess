#![allow(dead_code)]
#![allow(unused_variables)]
pub fn main6() {
    let mut v: Vec<u32> = Vec::new();
    let v2 = vec![1.2, 2.3, 3.4f32];
    {
        let v3 = vec![1, 26u32, 3];
        v.push(7);
    }
    let snd: &f32 = &v2[1];
    println!("2nd ele = {}", snd);
    match v2.get(3) {
        Some(snd) => println!("3rd ele two = {}", snd),
        None => println!("no 3rd!"),
    }
    let past = v2.get(50);
    println!("past = {:#?}", past);
    let past2 = v2.get(1);
    println!("past = {:#?}", past2);
    {
        let v = vec![100, 32, 57];
        for i in &v {
            print!(" {}", i);
        }
        let mut v = vec![33, 44, 55];
        for i in &mut v {
            *i += 50;
        }
        println!();
        for i in &v {
            print!(" {}", i);
        }
        println!();
    }
    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f32),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Text("brown".to_string()),
            SpreadsheetCell::Float(10.12),
        ];
    }
    {
        let s = String::new();
        let hello = String::from("Dobrý den");
        println!("hello = {}", hello);
        let mut s = "foo".to_string();
        let bar = "bar";
        s.push_str(bar);
        s.push('!');
        println!("s = {}, bar = {}", s, bar);
    }
    {
        let s1 = String::from("hi");
        let s2 = String::from("ho");
        let s3 = s1 + &s2;
        let s4 = format!("{}-{}", s3, s2);
        println!("s4 = {}", s4);
    }
    {
        let hi = "Dobrý den";
        println!("num bytes = {}", hi.len());
        for c in hi.chars() {
            println!("char = {}", c);
        }
        println!();
        for b in hi.bytes() {
            println!("byte = {}", b);
        }
    }

    use std::collections::HashMap;

    {
        //let mut scores: HashMap<String, i32> = HashMap::new();
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 20);
        let teams = vec![String::from("Green"), String::from("Brown")];
        let init_scores = vec![100, 120];
        let scores2: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();
    }
    {
        let name = String::from("color");
        let val = String::from("blue");
        let mut map = HashMap::new();
        map.insert(name, val);
        let colg = map.get("color");
        let cola = &map["color"];
        println!("[] color val = {}", cola);

        match colg {
            Some(val) => {
                println!("get color val = {}", val);
            }
            None => {
                println!("get color val = None");
            }
        }
    }
    let nameg = String::from("color");
    let valg = String::from("blue");
    let mut map = HashMap::new();
    {
        let name = String::from("color");
        let val = String::from("blue");
        //map.insert(&name, &val);
        //map.insert(&name, &val);
        map.insert(&nameg, &valg);
        map.insert(&nameg, &valg);
        println!("{} {}", name, val);
    }
    let hey = String::from("hey");
    let hum = String::from("hum");
    map.insert(&hey, &hum);
    for (key, val) in &map {
        println!("key = {}, val = {}", key, val);
    }
    println!("");
    {
        let mut hm = HashMap::new();
        hm.insert(666, 999);
        for i in 0..10 {
            hm.insert(i * 3 + 4, i * 10 + 1);
        }

        hm.insert(25, 1234);
        hm.insert(25, 5678);
        hm.entry(41).or_insert(1024);
        hm.entry(28).or_insert(65535);

        for (key, val) in &hm {
            println!("key = {}, val = {}", key, val);
        }
        println!("\nlen of map = {}", hm.len());
        //println!("{:#?}", hm);
    }
    {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}

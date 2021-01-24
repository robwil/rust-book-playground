// fn main() {
//     let mut s = String::from("Hello");
//     s.push_str(", world");
//     println!("{}", s);

//     let s1 = String::from("a");
//     let s2 = s1; // at this point, s1 is invalid
//     println!("{}", s2);

//     let s1 = String::from("hello");
//     let s2 = s1.clone(); // explicitly does "deep copy"
//     println!("s1 = {}, s2 = {}", s1, s2);

//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);
//     println!("The length of '{}' is {}.", s1, len);

//     let mut s = String::from("hello");
//     change(&mut s);
//     println!("{}", s);

//     let s = String::from("hello world");
//     let word = first_word(&s);
//     println!("The first word in '{}' is '{}'", s, word);
    
//     let s = "hello world";
//     let word = first_word(s);
//     println!("The first word in '{}' is '{}'", s, word);

//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     println!(
//         "The area of the rectangle {:?} is {} square pixels.",
//         rect1,
//         rect1.area()
//     );
//     let rect2 = Rectangle {
//         width: 10,
//         height: 40,
//     };
//     let rect3 = Rectangle {
//         width: 60,
//         height: 45,
//     };
//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
//     println!("Created 4x4 square: {:?}", Rectangle::square(4));

//     let m = Message::Write(String::from("hello"));
//     m.call();
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
//     fn square(size: u32) -> Rectangle {
//         Rectangle {
//             width: size,
//             height: size,
//         }
//     }
// }

// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// impl Message {
//     fn call(&self) {
//         println!("{:?}", self);
//         match 
//     }
// }


// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }

//     return &s[..]
// }
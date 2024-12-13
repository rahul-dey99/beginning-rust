// struct User {
//     username: String,
//     age: i8,
//     is_active: bool,
//     signin_count: i32
// }

// fn main() {

//     let mut user1 = User{
//         username: String::from("Rahul"),
//         age: 21,
//         is_active: false,
//         signin_count: 0
//     };

//     signin(&mut user1);
//     println!("Your username is {}, age is {}", user1.username, user1.age);
//     println!("{} logins.", user1.signin_count);

//     if user1.is_active {
//         println!("{} is active user.", user1.username);
//     } else {
//         println!("{} is not active user.", user1.username);
//     }
    // update_string();
    // check_pointers();
    // let mut s1 = String::from("Hello");
    // update_word(&mut s1);
    // println!("{}", s1);
// }

// fn signin(user: &mut User) {
//     user.signin_count += 1;
// }

// fn update_word(s1: &mut String) {
//     println!("s1 inside update_word {}", s1);
//     s1.push_str(" World");
//     println!("{}", s1);
// }

// fn check_pointers() {
//     let str = String::from("Hello World!");
//     println!("str pointer -> {:p}", str.as_ptr());
//     let str2 = str;
//     println!("str2 pointer -> {:p}", str2.as_ptr());
//     let str2 = String::from("Nice!");
//     let str3 = String::from("Nice!");

//     println!("str2 ptr -> {:p}, str3 ptr -> {:p}", str2.as_ptr(), str3.as_ptr());
// }



// fn update_string() {
//     let mut my_str = String::from("Hello");
//     println!("{}", my_str);
//     for _ in 0..10 {
//         my_str.push_str(" world ");
//         println!("Length -> {} Capacity -> {} Pointer -> {:p}", my_str.len(), my_str.capacity(), my_str.as_ptr());
//     }
//     println!("{}", my_str)
// }
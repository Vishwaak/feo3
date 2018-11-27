fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    };
    let user1 = User {
        email: String::from("minewine@gmail.com"),
        username: String::from("abi"),
        sign_in_count : 1,
        active: true,
    };
    println!("{}",user1.email);
    let user2 = User {
        email: String:: from("flinghing@gmail.com"),
        username: String:: from("hellokitty"),
        ..user1
    };
    println!("Details of user2 {} {} {}",user2.email,user2.username,user2.active);
}


fn build_user(email: String, username: String) -> User {
    User{
        email,
        username,
        active:true,
        sign_in_count: 1,
        }
}

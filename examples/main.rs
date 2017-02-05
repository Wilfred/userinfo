extern crate userinfo;

fn main() {
    let uid = userinfo::current_user_id();
    println!("Your UID is {}", uid);
    println!("Your username is {:?}", userinfo::login_name(uid));
    println!("Your full name is {:?}", userinfo::user_full_name(uid));
    println!("Your home directory is {:?}", userinfo::user_home_directory(uid));
}

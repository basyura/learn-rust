type UserName = String;
type Id = i64;
type Timestamp = i64;
type User = (Id, UserName, Timestamp);

fn main() {
    let id = 400;
    let now = 4567890123;
    let user = new_user(String::from("mika"), id, now);
    println!("{}", user.0);
    let bad_user = new_user(String::from("kazuki"), now, id);
    println!("{}", bad_user.0);
}

fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
    (id, name, created)
}

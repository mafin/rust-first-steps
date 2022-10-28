use uuid::Uuid;

pub fn greet() {
    let id = Uuid::new_v4();

    println!("Hello ID {}", id)
}

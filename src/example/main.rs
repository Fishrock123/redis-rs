#[crate_id = "redis-example#0.1"];
#[crate_type = "app"];

extern mod redis;

fn main() {
    let mut client = redis::Client::open("redis://127.0.0.1/").unwrap();
    println!("foo get: {:?}", client.get("foo"));
    println!("foo as int: {:?}", client.get_as::<int>("foo"));
    println!("Ping: {:?}", client.ping());
    println!("Keys: {:?}", client.keys("*"));
}

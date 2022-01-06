use memcache::Client;

fn main() {
    println!("Hello, memcachers!");

    let client = Client::connect("memcache://localhost:11211").unwrap();
    client.set("name", "Max", 0).unwrap();
    client.set("occupation", "Dog", 0).unwrap();

    let mut res: Option<String> = client.get("name").unwrap();

    match res {
        Some(value) => println!("name: {}", value),
        None => println!("memcache client failed"),
    } 

    //let res: Option<String> = client.get("occupation").unwrap();
    res = client.get("occupation").unwrap();
    match res {
        Some(value) => println!("occupation: {}", value),
        None => println!("memcache client failed"),
    } 
}

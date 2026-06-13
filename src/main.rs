fn main() {
    let events = vec!["claim:asset1", "verify:asset1", "resolve:asset1"];
    let mut root = String::from("0000000000000000");
    for (i, event) in events.iter().enumerate() {
        root = format!("{:016x}", root.len() + event.len() + i);
        println!("Event {}: {} -> root: {}", i, event, root);
    }
    println!("Final root: {}", root);
}

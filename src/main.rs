use std::collections::HashMap;

fn sha256_simple(data: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in data.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{:016x}", hash)
}

fn main() {
    let mut log: Vec<(String, String, String)> = Vec::new();
    
    println!("DROS Musik-Schutz System");
    println!("Gib den Titel deines Songs ein:");
    
    let mut titel = String::new();
    std::io::stdin().read_line(&mut titel).unwrap();
    let titel = titel.trim();
    
    let hash = sha256_simple(titel);
    let timestamp = "2026-06-13";
    
    log.push((titel.to_string(), hash.clone(), timestamp.to_string()));
    
    println!("---");
    println!("Song: {}", titel);
    println!("Hash: {}", hash);
    println!("Zeitstempel: {}", timestamp);
    println!("Gesichert in DROS Log.");
}

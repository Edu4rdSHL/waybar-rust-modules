use std::path::Path;

fn main() {
    if Path::new("/proc/sys/net/ipv4/conf/tun0").exists() {
        println!("{{\"text\": \" ON\"}}");
    } else {
        println!("{{\"text\": \" OFF\", \"class\": \"down\"}}");
    }
}

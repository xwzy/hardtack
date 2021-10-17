use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

pub struct HardTack {
    timestamp: u64,
    salt: u64,
    data: Vec<Tack>,
}

impl HardTack {
    fn get_uuid(&self) -> String { 
        self.timestamp.to_string() + &self.salt.to_string()
    }
}
pub struct Tack {
    id: i64,
    salt: i64,
    flags: i32,
    size: i32,
    data: Box<[u8]>,
    hash: i64,
}

impl Tack {

}

#[test]
fn test() {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_secs();

    let random_salt = rand::thread_rng().gen_range(0..u64::MAX);

    let h = HardTack{
        timestamp: in_ms,
        salt: random_salt,
        data: vec!()};

    print!(h.get_uuid());
}

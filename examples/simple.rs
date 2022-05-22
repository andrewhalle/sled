use std::str;

fn main() {
    let db: sled::Db = sled::open("my_db").unwrap();

    db.insert(b"hello", b"world").unwrap();
    let value = db.get(b"hello").unwrap().unwrap();
    println!("{}", str::from_utf8(&value).unwrap());
}

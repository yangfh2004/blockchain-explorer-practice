use lmdb::{DatabaseFlags, Environment, Transaction, WriteFlags};
use serde::{Deserialize, Serialize};
use tempdir::TempDir;

#[derive(Serialize, Deserialize, PartialEq)]
struct Value {
    s: String,
}

pub fn main() {
    // open a db, write a value, then read it back, using serde_json
    let tmp = TempDir::new("blockchain-explorer").expect("failed to open tmpdir");
    let path = tmp.path();

    let mut builder = Environment::new();
    builder.set_max_dbs(16);

    let env = builder.open(path).expect("failed to open env");
    let db = env
        .create_db(Some("mydb"), DatabaseFlags::empty())
        .expect("failed to open db");

    let mut rwtxn = env.begin_rw_txn().expect("can't begin rw txn");
    let key = "key";
    let val = Value {
        s: "val".to_string(),
    };

    let wbytes = serde_json::to_string(&val).expect("failed to serialize");

    rwtxn
        .put(db, &key, &wbytes, WriteFlags::empty())
        .expect("put failed");
    rwtxn.commit().expect("commit failed for rwtxn");

    let rotxn = env.begin_ro_txn().expect("can't begin ro txn");
    let rbytes = rotxn.get(db, &key).expect("failed to get key");
    let rstr = std::str::from_utf8(rbytes).expect("failed to parse read bytes");
    let rval: Value = serde_json::from_str(rstr).expect("failed to deserialize");

    assert!(val == rval);

    // convert string to/from JSON, read/write some values
    let json_str = r#"{ "string_key": "foo", "int_key": 42, "vec_key": [ 1, 2, 3 ] }"#;

    let json = json::parse(json_str).expect("failed to parse block as json");
    let foo = json["string_key"].as_str().expect("no string_key");
    let forty2 = json["int_key"].as_i64().expect("no int_key");

    println!(r#"json["string_key"] {:?}"#, foo);
    println!(r#"json["int_key"] {:?}"#, forty2);

    for x in json["vec_key"].members() {
        println!("{}", x.as_i64().expect("wasn't an int"));
    }
}

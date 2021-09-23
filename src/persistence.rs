use probabilistic_collections::cuckoo::ScalableCuckooFilter;
use bincode;
use std::fs::File;
use std::io::BufReader;

pub fn serialize_to(path: &str, filter: &ScalableCuckooFilter<String>) {
    let buffer = File::create(path).unwrap();
    bincode::serialize_into(buffer, filter).unwrap();
}

pub fn deserialize_from(path: &str) -> ScalableCuckooFilter<String> {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    bincode::deserialize_from(reader).unwrap()
}
fn main() {
    electrs_rocksdb::DB::open_default("db").unwrap();
}

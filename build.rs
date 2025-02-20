fn main() {
    tonic_build::compile_protos("proto/transaction.proto").unwrap();
}
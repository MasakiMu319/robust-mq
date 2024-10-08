#[cfg(test)]
mod tests {
    #[test]
    fn build_pb() {
        tonic_build::configure()
            .build_server(true)
            .out_dir("src/")
            .compile_protos(&["src/kv.proto"], &["src/"])
            .unwrap();
    }
}

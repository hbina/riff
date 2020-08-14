extern crate riff;

#[test]
fn read_as_riff() {
    let file =
        riff::riff_ram::Riff::from_file(std::path::PathBuf::from("test_assets/minimal.riff"))
            .unwrap();
    let root_chunk = file.get_chunk();
    root_chunk
        .iter_type()
        .inspect(|_| {})
        .map(riff::riff_ram::ChunkContents::from)
        .for_each(|_| {});
}

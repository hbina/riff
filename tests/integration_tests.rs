extern crate riff;

#[test]
fn test_minimal() {
    let file = riff::riff_ram::Riff::from_file(std::path::PathBuf::from("test_assets/set_1.riff"))
        .unwrap();
    assert_eq!(file.payload_len(), 14);
    assert_eq!(riff::riff_ram::Chunk::from(&file).id().as_str(), "RIFF");
    assert_eq!(
        riff::riff_ram::Chunk::from(&file).get_chunk_type().as_str(),
        "smpl"
    );
    let expected_content = vec![vec![255]];
    assert_eq!(
        file.iter().fold(0, |acc, _| acc + 1),
        expected_content.len()
    );
    file.iter()
        .zip(expected_content)
        .for_each(|(chunk, expected)| {
            assert_eq!(chunk.get_raw_child().len(), expected.len());
            assert_eq!(chunk.get_raw_child(), expected);
        });
    assert_eq!(file.iter().skip(1).next(), None);
}

#[test]
fn test_minimal_2() {
    let file = riff::riff_ram::Riff::from_file(std::path::PathBuf::from("test_assets/set_2.riff"))
        .unwrap();
    assert_eq!(file.payload_len(), 24);
    assert_eq!(riff::riff_ram::Chunk::from(&file).id().as_str(), "RIFF");
    assert_eq!(
        riff::riff_ram::Chunk::from(&file).get_chunk_type().as_str(),
        "smpl"
    );
    let expected_content = vec![("tst1", vec![255]), ("tst2", vec![238])];
    assert_eq!(
        file.iter().fold(0, |acc, _| acc + 1),
        expected_content.len()
    );
    file.iter()
        .zip(expected_content)
        .for_each(|(chunk, (name, data))| {
            assert_eq!(chunk.id().as_str(), name);
            assert_eq!(chunk.get_raw_child().len(), data.len());
            assert_eq!(chunk.get_raw_child(), data);
        });
    assert_eq!(file.iter().skip(2).next(), None);
}

#[test]
fn test_test() {
    let file = riff::riff_ram::Riff::from_file(std::path::PathBuf::from("test_assets/set_3.riff"))
        .unwrap();
    {
        assert_eq!(file.payload_len(), 100);
        assert_eq!(
            riff::riff_ram::Chunk::from(&file).id().as_str(),
            riff::chunk_id::RIFF_ID
        );
        assert_eq!(
            riff::riff_ram::Chunk::from(&file).get_chunk_type().as_str(),
            "smpl"
        );
        assert_eq!(file.iter().fold(0, |acc, _| acc + 1), 2);
    }
    {
        let list_1 = file.iter().next().unwrap();
        assert_eq!(list_1.id().as_str(), riff::chunk_id::LIST_ID);
        assert_eq!(list_1.get_chunk_type().as_str(), "tst1");
        assert_eq!(list_1.iter().fold(0, |acc, _| acc + 1), 2);
        {
            let test = list_1.iter().next().unwrap();
            assert_eq!(test.id().as_str(), "test");
            assert_eq!(test.get_raw_child(), "hey this is a test".as_bytes());
        }
        {
            let test = list_1.iter().skip(1).next().unwrap();
            assert_eq!(test.id().as_str(), "test");
            assert_eq!(test.get_raw_child(), "hey this is another test".as_bytes());
        }
    }
    {
        let list_1 = file.iter().skip(1).next().unwrap();
        assert_eq!(list_1.id().as_str(), "seqt");
        assert_eq!(list_1.iter().fold(0, |acc, _| acc + 1), 1);
        assert_eq!(list_1.iter().next().unwrap().id().as_str(), "test");
        assert_eq!(
            list_1.iter().next().unwrap().get_raw_child(),
            "final test".as_bytes()
        );
    }
}

#[test]
fn test_test_2() {
    let file = riff::riff_ram::Riff::from_file(std::path::PathBuf::from("test_assets/set_4.riff"))
        .unwrap();
    {
        assert_eq!(file.payload_len(), 102);
        assert_eq!(
            riff::riff_ram::Chunk::from(&file).id().as_str(),
            riff::chunk_id::RIFF_ID
        );
        assert_eq!(
            riff::riff_ram::Chunk::from(&file).get_chunk_type().as_str(),
            "smpl"
        );
        assert_eq!(file.iter().fold(0, |acc, _| acc + 1), 2);
    }
    {
        let list_1 = file.iter().next().unwrap();
        assert_eq!(list_1.id().as_str(), riff::chunk_id::LIST_ID);
        assert_eq!(list_1.get_chunk_type().as_str(), "tst1");
        assert_eq!(list_1.iter().fold(0, |acc, _| acc + 1), 2);
        {
            let test = list_1.iter().next().unwrap();
            assert_eq!(test.id().as_str(), "test");
            assert_eq!(test.get_raw_child(), "hey this is a test".as_bytes());
        }
        {
            let test = list_1.iter().skip(1).next().unwrap();
            assert_eq!(test.id().as_str(), "test");
            assert_eq!(test.get_raw_child(), "hey this is another test!".as_bytes());
        }
    }
    {
        let list_1 = file.iter().skip(1).next().unwrap();
        assert_eq!(list_1.id().as_str(), "seqt");
        assert_eq!(list_1.iter().fold(0, |acc, _| acc + 1), 1);
        assert_eq!(list_1.iter().next().unwrap().id().as_str(), "test");
        assert_eq!(
            list_1.iter().next().unwrap().get_raw_child(),
            "final test".as_bytes()
        );
    }
}

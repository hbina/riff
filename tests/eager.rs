extern crate riffu;

use riffu::{error::RiffResult, Riff};

#[test]
fn test_set_1() -> RiffResult<()> {
    let file = Riff::from_path("test_assets/set_1.riff").unwrap();
    let chunk = dbg!(file.as_chunk().unwrap());
    assert_eq!(chunk.payload_len().unwrap(), 14);
    assert_eq!(chunk.id().unwrap().as_bytes(), b"RIFF");
    assert_eq!(chunk.chunk_type().unwrap().as_bytes(), b"smpl");
    let expected_content = vec![vec![255]];
    assert_eq!(chunk.iter().unwrap().count(), expected_content.len());
    for (chunk, expected) in chunk.iter().unwrap().zip(expected_content) {
        let chunk = chunk.unwrap();
        assert_eq!(chunk.content().unwrap().len(), expected.len());
        assert_eq!(chunk.content().unwrap(), expected);
    }
    match chunk.iter().unwrap().skip(1).next() {
        None => assert!(true),
        _ => assert!(false),
    }
    Ok(())
}

#[test]
fn test_set_2() -> RiffResult<()> {
    let file = Riff::from_path("test_assets/set_2.riff").unwrap();
    let chunk = file.as_chunk().unwrap();
    assert_eq!(chunk.payload_len().unwrap(), 24);
    assert_eq!(chunk.id().unwrap().as_bytes(), b"RIFF");
    assert_eq!(chunk.chunk_type().unwrap().as_bytes(), b"smpl");
    let expected_content = vec![(b"tst1", vec![255]), (b"tst2", vec![238])];
    assert_eq!(chunk.iter().unwrap().count(), expected_content.len());
    for (chunk, (name, data)) in chunk.iter().unwrap().zip(expected_content) {
        let chunk = chunk.unwrap();
        assert_eq!(chunk.id().unwrap().as_bytes(), name);
        assert_eq!(chunk.content().unwrap().len(), data.len());
        assert_eq!(chunk.content().unwrap(), data);
    }
    match chunk.iter().unwrap().skip(2).next() {
        None => assert!(true),
        _ => assert!(false),
    }
    Ok(())
}

#[test]
fn test_set_3() -> RiffResult<()> {
    let file = Riff::from_path("test_assets/set_3.riff").unwrap();
    let chunk = file.as_chunk().unwrap();
    {
        assert_eq!(chunk.payload_len().unwrap(), 100);
        assert_eq!(chunk.id().unwrap().as_bytes(), riffu::constants::RIFF_ID);
        assert_eq!(chunk.chunk_type().unwrap().as_bytes(), b"smpl");
        assert_eq!(chunk.iter().unwrap().count(), 2);
    }
    {
        let list_1 = chunk.iter().unwrap().next().unwrap().unwrap();
        assert_eq!(list_1.id().unwrap().as_bytes(), riffu::constants::LIST_ID);
        assert_eq!(list_1.chunk_type().unwrap().as_bytes(), b"tst1");
        assert_eq!(list_1.iter().unwrap().count(), 2);
        {
            let test = list_1.iter().unwrap().next().unwrap().unwrap();
            assert_eq!(test.id().unwrap().as_bytes(), b"test");
            assert_eq!(
                test.content().unwrap(),
                "hey this is a test".as_bytes()
            );
        }
        {
            let test = list_1.iter().unwrap().skip(1).next().unwrap().unwrap();
            assert_eq!(test.id().unwrap().as_bytes(), b"test");
            assert_eq!(
                test.content().unwrap(),
                "hey this is another test".as_bytes()
            );
        }
    }
    {
        let list_1 = chunk.iter().unwrap().skip(1).next().unwrap().unwrap();
        assert_eq!(list_1.id().unwrap().as_bytes(), b"seqt");
        assert_eq!(list_1.iter().unwrap().count(), 1);
        assert_eq!(
            list_1
                .iter()
                .unwrap()
                .next()
                .unwrap()
                .unwrap()
                .id()
                .unwrap()
                .as_bytes(),
            b"test"
        );
        assert_eq!(
            list_1
                .iter()
                .unwrap()
                .next()
                .unwrap()
                .unwrap()
                .content()
                .unwrap(),
            b"final test"
        );
    }
    Ok(())
}

#[test]
fn test_set_4() -> RiffResult<()> {
    let file = Riff::from_path("test_assets/set_4.riff").unwrap();
    let chunk = file.as_chunk().unwrap();
    {
        assert_eq!(chunk.payload_len().unwrap(), 102);
        assert_eq!(chunk.id().unwrap().as_bytes(), riffu::constants::RIFF_ID);
        assert_eq!(chunk.chunk_type().unwrap().as_bytes(), b"smpl");
        assert_eq!(chunk.iter().unwrap().count(), 2);
    }
    {
        let list_1 = chunk.iter().unwrap().next().unwrap().unwrap();
        assert_eq!(list_1.id().unwrap().as_bytes(), riffu::constants::LIST_ID);
        assert_eq!(list_1.chunk_type().unwrap().as_bytes(), b"tst1");
        assert_eq!(list_1.iter().unwrap().count(), 2);
        {
            let test = list_1.iter().unwrap().next().unwrap().unwrap();
            assert_eq!(test.id().unwrap().as_bytes(), b"test");
            assert_eq!(test.content().unwrap(), b"hey this is a test");
        }
        {
            let test = list_1.iter().unwrap().skip(1).next().unwrap().unwrap();
            assert_eq!(test.id().unwrap().as_bytes(), b"test");
            assert_eq!(test.content().unwrap(), b"hey this is another test!");
        }
    }
    {
        let list_1 = chunk.iter().unwrap().skip(1).next().unwrap().unwrap();
        assert_eq!(list_1.id().unwrap().as_bytes(), b"seqt");
        assert_eq!(list_1.iter().unwrap().count(), 1);
        assert_eq!(
            list_1
                .iter()
                .unwrap()
                .next()
                .unwrap()
                .unwrap()
                .id()
                .unwrap()
                .as_bytes(),
            b"test"
        );
        assert_eq!(
            list_1
                .iter()
                .unwrap()
                .next()
                .unwrap()
                .unwrap()
                .content()
                .unwrap(),
            b"final test"
        );
    }
    Ok(())
}

#[test]
fn test_chimes_wav() -> RiffResult<()> {
    let file = Riff::from_path("test_assets/Chimes.wav").unwrap();
    let chunk = file.as_chunk().unwrap();
    assert_eq!(b"RIFF", chunk.id().unwrap().as_bytes());
    assert_eq!(15924, chunk.payload_len().unwrap());
    let expected = vec![(b"fmt ", 16), (b"fact", 4), (b"data", 15876)];
    for (chunk, (expected_name, expected_payload)) in chunk.iter().unwrap().zip(expected.iter()) {
        let chunk = chunk.unwrap();
        assert_eq!(*expected_name, chunk.id().unwrap().as_bytes());
        assert_eq!(*expected_payload, chunk.payload_len().unwrap());
    }
    Ok(())
}

#[test]
fn test_canimate_avi() -> RiffResult<()> {
    let file = Riff::from_path("test_assets/Canimate.avi").unwrap();
    let chunk = file.as_chunk().unwrap();
    assert_eq!(b"RIFF", chunk.id().unwrap().as_bytes());
    assert_eq!(91952, chunk.payload_len().unwrap());
    let expected = vec![
        (b"LIST", 1216),
        (b"JUNK", 2840),
        (b"LIST", 87620),
        (b"idx1", 240),
    ];
    for (chunk, (expected_name, expected_payload)) in chunk.iter().unwrap().zip(expected.iter()) {
        let chunk = chunk.unwrap();
        assert_eq!(*expected_name, chunk.id().unwrap().as_bytes());
        assert_eq!(*expected_payload, chunk.payload_len().unwrap());
    }
    Ok(())
}

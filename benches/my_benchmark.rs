extern crate riffu;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use riffu::{
    constants::{LIST_ID, RIFF_ID},
    error::RiffResult,
    Riff,
};

fn test_set_4(_: ()) -> RiffResult<()> {
    let file = Riff::from_path("test_assets/set_4.riff")?;
    let chunk = file.as_chunk()?;
    {
        assert_eq!(chunk.payload_len()?, 102);
        assert_eq!(chunk.id()?.as_bytes(), RIFF_ID);
        assert_eq!(chunk.chunk_type()?.as_bytes(), b"smpl");
        assert_eq!(chunk.iter()?.fold(0, |acc, _| acc + 1), 2);
    }
    {
        let list_1 = chunk.iter()?.next().unwrap()?;
        assert_eq!(list_1.id()?.as_bytes(), LIST_ID);
        assert_eq!(list_1.chunk_type()?.as_bytes(), b"tst1");
        assert_eq!(list_1.iter()?.fold(0, |acc, _| acc + 1), 2);
        {
            let test = list_1.iter()?.next().unwrap()?;
            assert_eq!(test.id()?.as_bytes(), b"test");
            assert_eq!(test.content()?, "hey this is a test".as_bytes());
        }
        {
            let test = list_1.iter()?.skip(1).next().unwrap()?;
            assert_eq!(test.id()?.as_bytes(), b"test");
            assert_eq!(test.content()?, "hey this is another test!".as_bytes());
        }
    }
    {
        let list_1 = chunk.iter()?.skip(1).next().unwrap()?;
        assert_eq!(list_1.id()?.as_bytes(), b"seqt");
        assert_eq!(list_1.iter()?.fold(0, |acc, _| acc + 1), 1);
        assert_eq!(list_1.iter()?.next().unwrap()?.id()?.as_bytes(), b"test");
        assert_eq!(list_1.iter()?.next().unwrap()?.content()?, b"final test");
    }
    Ok(())
}

fn test_set_3(_: ()) -> RiffResult<()> {
    let file = Riff::from_path("test_assets/set_3.riff")?;
    let chunk = file.as_chunk()?;
    {
        assert_eq!(chunk.payload_len()?, 100);
        assert_eq!(chunk.id()?.as_bytes(), RIFF_ID);
        assert_eq!(chunk.chunk_type()?.as_bytes(), b"smpl");
        assert_eq!(chunk.iter()?.fold(0, |acc, _| acc + 1), 2);
    }
    {
        let list_1 = chunk.iter()?.next().unwrap()?;
        assert_eq!(list_1.id()?.as_bytes(), LIST_ID);
        assert_eq!(list_1.chunk_type()?.as_bytes(), b"tst1");
        assert_eq!(list_1.iter()?.fold(0, |acc, _| acc + 1), 2);
        {
            let test = list_1.iter()?.next().unwrap()?;
            assert_eq!(test.id()?.as_bytes(), b"test");
            assert_eq!(test.content()?, "hey this is a test".as_bytes());
        }
        {
            let test = list_1.iter()?.skip(1).next().unwrap()?;
            assert_eq!(test.id()?.as_bytes(), b"test");
            assert_eq!(test.content()?, "hey this is another test".as_bytes());
        }
    }
    {
        let list_1 = chunk.iter()?.skip(1).next().unwrap()?;
        assert_eq!(list_1.id()?.as_bytes(), b"seqt");
        assert_eq!(list_1.iter()?.fold(0, |acc, _| acc + 1), 1);
        assert_eq!(list_1.iter()?.next().unwrap()?.id()?.as_bytes(), b"test");
        assert_eq!(list_1.iter()?.next().unwrap()?.content()?, b"final test");
    }
    Ok(())
}

fn test_set_2(_: ()) -> RiffResult<()> {
    let file = Riff::from_path("test_assets/set_2.riff")?;
    let chunk = file.as_chunk()?;
    assert_eq!(chunk.payload_len()?, 24);
    assert_eq!(chunk.id()?.as_bytes(), b"RIFF");
    assert_eq!(chunk.chunk_type()?.as_bytes(), b"smpl");
    let expected_content = vec![(b"tst1", vec![255]), (b"tst2", vec![238])];
    assert_eq!(
        chunk.iter()?.fold(0, |acc, _| acc + 1),
        expected_content.len()
    );
    for (chunk, (name, data)) in chunk.iter()?.zip(expected_content) {
        let chunk = chunk?;
        assert_eq!(chunk.id()?.as_bytes(), name);
        assert_eq!(chunk.content()?.len(), data.len());
        assert_eq!(chunk.content()?, data);
    }
    Ok(())
}

fn test_set_1(_: ()) -> RiffResult<()> {
    let file = Riff::from_path("test_assets/set_1.riff")?;
    let chunk = file.as_chunk()?;
    assert_eq!(chunk.payload_len()?, 14);
    assert_eq!(chunk.id()?.as_bytes(), b"RIFF");
    assert_eq!(chunk.chunk_type()?.as_bytes(), b"smpl");
    let expected_content = vec![vec![255]];
    assert_eq!(
        chunk.iter()?.fold(0, |acc, _| acc + 1),
        expected_content.len()
    );
    for (chunk, expected) in chunk.iter()?.zip(expected_content) {
        let chunk = chunk?;
        assert_eq!(chunk.content()?.len(), expected.len());
        assert_eq!(chunk.content()?, expected);
    }
    Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lazy sets 1 => ", |b| b.iter(|| test_set_1(black_box(()))));
    c.bench_function("lazy sets 2 => ", |b| b.iter(|| test_set_2(black_box(()))));
    c.bench_function("lazy sets 3 => ", |b| b.iter(|| test_set_3(black_box(()))));
    c.bench_function("lazy sets 4 => ", |b| b.iter(|| test_set_4(black_box(()))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

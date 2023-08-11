use criterion::{black_box, Criterion};
use protokit::*;

pub trait Serialize {
    type Message: Default + for<'a> BinProto<'a>;

    fn serialize_pb(&self) -> Self::Message;
}

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
    where
        T: Serialize,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/protokit", name));

    let mut serialize_buffer = Vec::with_capacity(BUFFER_LEN);

    group.bench_function("serialize (populate + encode)", |b| {
        b.iter(|| {
            serialize_buffer.clear();
            let inbuf = std::mem::replace(&mut serialize_buffer, vec![]);
            serialize_buffer = black_box(protokit::binformat::encode_to(&data.serialize_pb(), inbuf).unwrap());
            // black_box(&mut serialize_buffer).clear();
            // data.serialize_pb().encode(&mut serialize_buffer).unwrap();
            // black_box(());
        })
    });

    let message = data.serialize_pb();
    group.bench_function("serialize (encode)", |b| {
        b.iter(|| {
            serialize_buffer.clear();
            let inbuf = std::mem::replace(&mut serialize_buffer, vec![]);

            serialize_buffer = black_box(protokit::binformat::encode_to(&message, inbuf).unwrap());
            // black_box(&mut serialize_buffer).clear();
            // message.encode(&mut serialize_buffer).unwrap();
            // black_box(());
        })
    });

    // let mut deserialize_buffer = Vec::new();
    // data.serialize_pb().encode(&mut deserialize_buffer).unwrap();

    let deserialize_buffer = protokit::binformat::encode(&data.serialize_pb()).unwrap();

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(protokit::binformat::decode::<T::Message>(black_box(&deserialize_buffer).as_slice()).unwrap());
        })
    });

    crate::bench_size(name, "protokit", deserialize_buffer.as_slice());

    group.finish();
}

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn validate_data_with_checksum<T>(data: T) -> bool
where T: AsRef<[u8]> {
    let data = data.as_ref();
    let checksum_index = data.len() - 1;
    fast_ihex::checksum::checksum(&data[..checksum_index]) == data[checksum_index]
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("checksum", |b| {
        b.iter(|| {
            fast_ihex::checksum::checksum(black_box([
                0x04, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0xCD,
            ]))
        })
    });

    let mut validate_group = c.benchmark_group("validate_data");
    validate_group.bench_function("checksum == last_byte", |b| {
        b.iter(|| {
            validate_data_with_checksum(black_box([
                0x04, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0xCD, 0x2A,
            ]))
        })
    });
    validate_group.bench_function("validate_data", |b| {
        b.iter(|| {
            fast_ihex::checksum::is_data_valid(black_box([
                0x04, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0xCD, 0x2A,
            ]))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

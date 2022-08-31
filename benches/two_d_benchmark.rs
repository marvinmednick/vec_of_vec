use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use std::time::Duration;
use two_d_array::{FlattendArray};

pub fn flat_array_init(val: u32) -> u32 {
    let size = 2000;
    let mut data : FlattendArray<u32> = FlattendArray::new(size,size);
    data.set(black_box(3),black_box(0),black_box(val));
    black_box(data.get(black_box(3),black_box(0)).unwrap())
}

fn flat_array_loop_benchmark(c: &mut Criterion) {

    static KB: usize = 1024;
    let mut group = c.benchmark_group("flat_array_loop");
    let mut sum = 0;
    for size in [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut data : FlattendArray<u32> = FlattendArray::new(size,size);
                data.set(black_box(3),black_box(0),black_box(size as u32));
                sum += black_box(data.get(black_box(3),black_box(0)).unwrap());

                }
            );
        });
    }
    group.finish();
}


fn flat_array_init_benchmark(c: &mut Criterion) {
    c.bench_function("flat_array_init", |b| b.iter(|| flat_array_init(black_box(2))));
}


criterion_group!(benches, flat_array_init_benchmark,flat_array_loop_benchmark);
criterion_main!(benches);

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use std::time::Duration;
use vec_of_vec::{VecOfVec,FlattendArray};

fn fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}


pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn vec_init(val: u32) -> u32 {
    let size = 2000;
    let mut data : VecOfVec<u32> = VecOfVec::new(size,size);
    data.set(black_box(3),black_box(0),black_box(val));
    black_box(data.get(black_box(3),black_box(0)).unwrap())
}

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

fn add2_benchmark(c: &mut Criterion) {
    c.bench_function("add 2", |b| b.iter(|| add_two(black_box(2))));
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-example");
    
    group.sample_size(10);
    group.warm_up_time(Duration::new(1, 500000));
    group.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    group.finish();
}

fn vec_of_vec_init_benchmark(c: &mut Criterion) {
    c.bench_function("vec_init", |b| b.iter(|| vec_init(black_box(2))));
}

fn flat_array_init_benchmark(c: &mut Criterion) {
    c.bench_function("flat_array_init", |b| b.iter(|| flat_array_init(black_box(2))));
}


criterion_group!(benches, criterion_benchmark, add2_benchmark,vec_of_vec_init_benchmark,flat_array_init_benchmark,flat_array_loop_benchmark);
criterion_main!(benches);

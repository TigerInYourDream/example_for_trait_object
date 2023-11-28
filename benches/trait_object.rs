use criterion::{black_box, criterion_group, criterion_main, Criterion};
use example_for_trait_object::{
    execute_boxed_trait_object, execute_genrics, execute_trait_object, Shell,
};

pub fn genericis_benchmark(c: &mut Criterion) {
    c.bench_function("generics", |b| {
        b.iter(|| {
            let cmd = Shell::new("ls", &[]);
            execute_genrics(black_box(&cmd)).unwrap();
        })
    });
}

pub fn trait_object_benchmark(c: &mut Criterion) {
    c.bench_function("trait_object", |b| {
        b.iter(|| {
            let cmd = Shell::new("ls", &[]);
            execute_trait_object(black_box(&cmd)).unwrap();
        })
    });
}

pub fn boxed_object_benchmark(c: &mut Criterion) {
    c.bench_function("boxed", |b| {
        b.iter(|| {
            let cmd = Box::new(Shell::new("ls", &[]));
            execute_boxed_trait_object(black_box(cmd)).unwrap();
        })
    });
}

criterion_group!(
    benches,
    genericis_benchmark,
    trait_object_benchmark,
    boxed_object_benchmark
);

criterion_main!(benches);

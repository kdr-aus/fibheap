use criterion::*;
use fibheap::*;
use rand::prelude::*;
use std::collections::BinaryHeap;
use std::iter::*;

fn random(n: usize) -> Vec<u32> {
    let mut rng = StdRng::seed_from_u64(314);
    repeat_with(|| rng.gen_range(0..1_000u32)).take(n).collect()
}

fn peeking(c: &mut Criterion) {
    c.bench_function("std::BinaryHeap::peek", |b| {
        let heap = BinaryHeap::from_iter(0..100_000u32);
        b.iter(|| black_box(heap.peek()));
    });

    c.bench_function("v1::FibonacciHeap::peek", |b| {
        let heap = v1::FibonacciHeap::from_iter(0..100_000u32);
        b.iter(|| black_box(heap.peek()));
    });
}

fn pushing(c: &mut Criterion) {
    // be careful of destructors
    let lrg = random(10_000_000);

    c.bench_function("std::BinaryHeap::push one-el n100", |b| {
        b.iter_batched_ref(
            || BinaryHeap::from_iter(lrg.iter().take(100).copied()),
            |heap| heap.push(500),
            BatchSize::SmallInput,
        );
    });

    c.bench_function("std::BinaryHeap::push one-el n10_000", |b| {
        b.iter_batched_ref(
            || BinaryHeap::from_iter(lrg.iter().take(10_000).copied()),
            |heap| heap.push(500),
            BatchSize::SmallInput,
        );
    });

    c.bench_function("std::BinaryHeap::push one-el n10_000_000", |b| {
        b.iter_batched_ref(
            || BinaryHeap::from_iter(lrg.iter().take(10_000_000).copied()),
            |heap| heap.push(500),
            BatchSize::SmallInput,
        );
    });

    c.bench_function("v1::FibonacciHeap::push one-el n100", |b| {
        b.iter_batched_ref(
            || v1::FibonacciHeap::from_iter(lrg.iter().take(100).copied()),
            |heap| heap.push(500),
            BatchSize::SmallInput,
        );
    });

    c.bench_function("v1::FibonacciHeap::push one-el n10_000", |b| {
        b.iter_batched_ref(
            || v1::FibonacciHeap::from_iter(lrg.iter().take(10_000).copied()),
            |heap| heap.push(500),
            BatchSize::SmallInput,
        );
    });

    c.bench_function("v1::FibonacciHeap::push one-el n10_000_000", |b| {
        b.iter_batched_ref(
            || v1::FibonacciHeap::from_iter(lrg.iter().take(10_000_000).copied()),
            |heap| heap.push(500),
            BatchSize::SmallInput,
        );
    });

    c.bench_function("v2::FibonacciHeap::push one-el n100", |b| {
        b.iter_batched_ref(
            || v2::FibonacciHeap::from_iter(lrg.iter().take(100).copied()),
            |heap| heap.push(500),
            BatchSize::SmallInput,
        );
    });

    c.bench_function("v2::FibonacciHeap::push one-el n10_000", |b| {
        b.iter_batched_ref(
            || v2::FibonacciHeap::from_iter(lrg.iter().take(10_000).copied()),
            |heap| heap.push(500),
            BatchSize::SmallInput,
        );
    });

    c.bench_function("v2::FibonacciHeap::push one-el n10_000_000", |b| {
        b.iter_batched_ref(
            || v2::FibonacciHeap::from_iter(lrg.iter().take(10_000_000).copied()),
            |heap| heap.push(500),
            BatchSize::SmallInput,
        );
    });
}

fn construction(c: &mut Criterion) {
    // be careful of destructors
    let lrg = random(10_000);

    c.bench_function("std::BinaryHeap::from_iter n10_000", |b| {
        b.iter_with_large_drop(|| {
            black_box(BinaryHeap::from_iter(lrg.iter().take(10_000).copied()))
        })
    });

    c.bench_function("v1::FibonacciHeap::from_iter n10_000", |b| {
        b.iter_with_large_drop(|| {
            black_box(v1::FibonacciHeap::from_iter(
                lrg.iter().take(10_000).copied(),
            ))
        })
    });

    c.bench_function("v2::FibonacciHeap::from_iter n10_000", |b| {
        b.iter_with_large_drop(|| {
            black_box(v2::FibonacciHeap::from_iter(
                lrg.iter().take(10_000).copied(),
            ))
        })
    });
}

fn draining(c: &mut Criterion) {
    let lrg = random(100_000);

    c.bench_function("std::BinaryHeap drain 1000", |b| {
        b.iter_batched(
            || BinaryHeap::from_iter(lrg.iter().take(1000).copied()),
            |mut heap| loop {
                if heap.pop().is_none() {
                    break;
                }
            },
            BatchSize::SmallInput,
        );
    });

    c.bench_function("std::BinaryHeap drain 100_000", |b| {
        b.iter_batched(
            || BinaryHeap::from_iter(lrg.iter().copied()),
            |mut heap| loop {
                if heap.pop().is_none() {
                    break;
                }
            },
            BatchSize::SmallInput,
        );
    });

    c.bench_function("v1::FibonacciHeap drain 1000", |b| {
        b.iter_batched(
            || v1::FibonacciHeap::from_iter(lrg.iter().take(1000).copied()),
            |mut heap| loop {
                if heap.pop().is_none() {
                    break;
                }
            },
            BatchSize::SmallInput,
        );
    });

    //     c.bench_function("v1::FibonacciHeap drain 100_000", |b| {
    //         b.iter_batched(
    //             || v1::FibonacciHeap::from_iter(lrg.iter().copied()),
    //             |mut heap| loop {
    //                 if heap.pop().is_none() {
    //                     break;
    //                 }
    //             },
    //             BatchSize::SmallInput,
    //         );
    //     });

    c.bench_function("v2::FibonacciHeap drain 1000", |b| {
        b.iter_batched(
            || v2::FibonacciHeap::from_iter(lrg.iter().take(1000).copied()),
            |mut heap| loop {
                if heap.pop().is_none() {
                    break;
                }
            },
            BatchSize::SmallInput,
        );
    });

    c.bench_function("v2::FibonacciHeap drain 100_000", |b| {
        b.iter_batched(
            || v2::FibonacciHeap::from_iter(lrg.iter().copied()),
            |mut heap| loop {
                if heap.pop().is_none() {
                    break;
                }
            },
            BatchSize::SmallInput,
        );
    });
}

fn use_case(c: &mut Criterion) {
    enum Op {
        Pop,
        Push(u32),
    }
    let mut rng = StdRng::seed_from_u64(314);
    let ops = repeat_with(|| match rng.gen_range(0..10u8) == 0 {
        true => Op::Pop,
        false => Op::Push(rng.gen_range(0..1000u32)),
    })
    .take(10_000)
    .collect::<Vec<_>>();

    c.bench_function("std::BinaryHeap randomops 10_000", |b| {
        b.iter_with_large_drop(|| {
            let mut heap = BinaryHeap::new();
            for op in &ops {
                match op {
                    Op::Pop => {
                        heap.pop();
                    }
                    Op::Push(x) => heap.push(*x),
                }
            }

            heap
        });
    });

    c.bench_function("v1::FibonacciHeap randomops 10_000", |b| {
        b.iter_with_large_drop(|| {
            let mut heap = v1::FibonacciHeap::new();
            for op in &ops {
                match op {
                    Op::Pop => {
                        heap.pop();
                    }
                    Op::Push(x) => heap.push(*x),
                }
            }

            heap
        });
    });

    c.bench_function("v2::FibonacciHeap randomops 10_000", |b| {
        b.iter_with_large_drop(|| {
            let mut heap = v2::FibonacciHeap::new();
            for op in &ops {
                match op {
                    Op::Pop => {
                        heap.pop();
                    }
                    Op::Push(x) => heap.push(*x),
                }
            }

            heap
        });
    });
}

criterion_group!(benches, peeking, pushing, construction, draining, use_case);
criterion_main!(benches);

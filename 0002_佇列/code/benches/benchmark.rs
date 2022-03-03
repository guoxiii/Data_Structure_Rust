use criterion::{ criterion_group, criterion_main, Criterion };
use code::Personal;
use code::PersonalAry;

fn fun() -> u64 {
    let mut personal_ary = PersonalAry {
        personal: Vec::new()
    };

    for _ in 0 .. 10000 {
        personal_ary.personal.push(Personal { name: String::from("A"), age: 52 });
    }

    for _ in 0 .. 10000 {
        personal_ary.next();
    }
    
    0
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("benchmark", |b| b.iter(|| fun()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
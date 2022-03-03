use std::rc::Rc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rox::{RoxMap, RoxNumber, RoxString, Table};

    pub fn criterion_benchmark(c: &mut Criterion) {
        c.bench_function("basic table", |b| {
            b.iter(|| {
                let mut t = Table::new();
                t.set(
                    Rc::new(RoxString::new("abc")),
                    &rox::Value::Number(RoxNumber(123.0)),
                );
                t.set(
                    Rc::new(RoxString::new("def")),
                    &rox::Value::Number(RoxNumber(123.0)),
                );
                t.set(
                    Rc::new(RoxString::new("ghi")),
                    &rox::Value::Number(RoxNumber(123.0)),
                );
                t.set(
                    Rc::new(RoxString::new("123")),
                    &rox::Value::Number(RoxNumber(123.0)),
                );
                t.set(
                    Rc::new(RoxString::new("456")),
                    &rox::Value::Number(RoxNumber(123.0)),
                );
                t.set(
                    Rc::new(RoxString::new("789")),
                    &rox::Value::Number(RoxNumber(123.0)),
                );

                t.remove(Rc::new(RoxString::new("abc")));
                t.remove(Rc::new(RoxString::new("def")));
                t.remove(Rc::new(RoxString::new("ghi")));
                t.remove(Rc::new(RoxString::new("123")));
                t.remove(Rc::new(RoxString::new("456")));
                t.remove(Rc::new(RoxString::new("789")));
            })
        });
    }

    criterion_group!(benches, criterion_benchmark);
    criterion_main!(benches);

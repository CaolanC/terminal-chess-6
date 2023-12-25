use chess::io::ansi::{Colours, Modifiers, Style};
use chess::styled;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("ansi: static str impl", |b| {
        b.iter(|| {
            let mut style = Style::from(black_box("This is a test string to be styled."));

            style
                .colour(black_box(Colours::RED))
                .colour(black_box(Colours::BG_BLUE))
                .bold()
                .italic()
                .underlined();

            format!("{}", style);
        })
    });

    c.bench_function("ansi: with macro", |b| {
        b.iter(|| {
            format!(
                "{}",
                styled!(
                    "This is a test string to be styled",
                    "red",
                    "bg-blue",
                    "bold",
                    "italic",
                    "underlined"
                )
            );
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use terminal_chess::io::ansi::{BgColour, FgColour, Style};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("ansi: static str impl", |b| {
        b.iter(|| {
            let mut style = Style::from(black_box("This is a test string to be styled."));

            style
                .fg(black_box(FgColour::RED))
                .bg(black_box(BgColour::BLUE))
                .bold()
                .italic()
                .underline();

            format!("{}", style);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

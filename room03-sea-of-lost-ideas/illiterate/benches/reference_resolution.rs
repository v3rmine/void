use std::collections::HashMap;

use criterion::{
    Criterion, criterion_group, criterion_main,
};
use illiterate::{
    resolve_references,
    types::{BlockData, IlliterateRef},
};

fn gen_ref(name: &str) -> IlliterateRef {
    IlliterateRef {
        base_indent_match: String::new(),
        is_inline: false,
        name: name.to_string(),
        ref_text: format!("<<{name}>>"),
    }
}

fn gen_block(content: &str, deps: &[&str]) -> BlockData {
    BlockData {
        code_content: content.to_string(),
        refs_in_code: deps
            .iter()
            .map(|d| gen_ref(d))
            .collect(),
    }
}

fn bench_resolve_no_deps(c: &mut Criterion) {
    let mut blocks = HashMap::new();
    for i in 0..100 {
        blocks.insert(
            format!("block_{i}"),
            gen_block(&format!("content_{i}"), &[]),
        );
    }
    c.bench_function("resolve_no_deps_100", |b| {
        b.iter(|| resolve_references(blocks.clone()))
    });
}

fn bench_resolve_linear_chain(c: &mut Criterion) {
    let mut blocks = HashMap::new();
    blocks.insert(
        "block_0".to_string(),
        gen_block("base_content", &[]),
    );
    for i in 1..100 {
        blocks.insert(
            format!("block_{i}"),
            gen_block(
                &format!("<<block_{}>>", i - 1),
                &[&format!("block_{}", i - 1)],
            ),
        );
    }
    c.bench_function("resolve_linear_chain_100", |b| {
        b.iter(|| resolve_references(blocks.clone()))
    });
}

fn bench_resolve_wide_fanout(c: &mut Criterion) {
    let mut blocks = HashMap::new();
    blocks.insert(
        "base".to_string(),
        gen_block("base_content", &[]),
    );
    for i in 0..99 {
        blocks.insert(
            format!("dep_{i}"),
            gen_block(
                &format!("<<base>> content_{i}"),
                &["base"],
            ),
        );
    }
    c.bench_function("resolve_wide_fanout_100", |b| {
        b.iter(|| resolve_references(blocks.clone()))
    });
}

fn bench_resolve_deep_dag(c: &mut Criterion) {
    let mut blocks = HashMap::new();
    blocks.insert(
        "root".to_string(),
        gen_block("root_content", &[]),
    );
    for i in 1..50 {
        let prev = format!("block_{}", i - 1);
        let name = format!("block_{i}");
        blocks.insert(
            name.clone(),
            gen_block(
                &format!("<<{prev}>> + content"),
                &[&prev],
            ),
        );
    }
    for i in 1..50 {
        let name = format!("block_{i}");
        let leaf = format!("leaf_{i}");
        blocks.insert(
            leaf.clone(),
            gen_block(
                &format!("<<{name}>> leaf"),
                &[&name],
            ),
        );
    }
    c.bench_function("resolve_deep_dag_100", |b| {
        b.iter(|| resolve_references(blocks.clone()))
    });
}

fn bench_resolve_complex_dag(c: &mut Criterion) {
    let mut blocks = HashMap::new();
    blocks.insert(
        "base_0".to_string(),
        gen_block("base", &[]),
    );
    blocks.insert(
        "base_1".to_string(),
        gen_block("base", &[]),
    );
    for i in 2..100 {
        let deps: Vec<String> = if i < 10 {
            vec!["base_0".to_string(), "base_1".to_string()]
        } else {
            vec![
                format!("block_{}", i - 3),
                format!("block_{}", i - 2),
                format!("block_{}", i - 1),
            ]
        };
        let dep_refs: Vec<&str> =
            deps.iter().map(|s| s.as_str()).collect();
        blocks.insert(
            format!("block_{i}"),
            gen_block(
                &format!("deps: {}", deps.join(", ")),
                &dep_refs,
            ),
        );
    }
    c.bench_function("resolve_complex_dag_100", |b| {
        b.iter(|| resolve_references(blocks.clone()))
    });
}

fn bench_resolve_circular(c: &mut Criterion) {
    let mut blocks = HashMap::new();
    for i in 0..10 {
        let next = format!("block_{}", (i + 1) % 10);
        blocks.insert(
            format!("block_{i}"),
            gen_block(&format!("<<{next}>>"), &[&next]),
        );
    }
    c.bench_function("resolve_circular_10", |b| {
        b.iter(|| resolve_references(blocks.clone()))
    });
}

fn bench_resolve_missing_refs(c: &mut Criterion) {
    let mut blocks = HashMap::new();
    for i in 0..50 {
        blocks.insert(
            format!("block_{i}"),
            gen_block(
                &format!("<<missing_{i}>>"),
                &[&format!("missing_{i}")],
            ),
        );
    }
    c.bench_function("resolve_missing_refs_50", |b| {
        b.iter(|| resolve_references(blocks.clone()))
    });
}

criterion_group!(
    benches,
    bench_resolve_no_deps,
    bench_resolve_linear_chain,
    bench_resolve_wide_fanout,
    bench_resolve_deep_dag,
    bench_resolve_complex_dag,
    bench_resolve_circular,
    bench_resolve_missing_refs,
);
criterion_main!(benches);

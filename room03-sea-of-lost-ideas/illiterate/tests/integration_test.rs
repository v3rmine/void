use std::process::Command;

use fancy_regex::Regex;

struct IlliterateOutput {
    stdout: String,
    exit_code: Option<i32>,
}

fn strip_ansi(s: &str) -> String {
    let re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    re.replace_all(s, "").to_string()
}

fn run_illiterate(
    source: &str,
    config: Option<&str>,
    command: &str,
    extra_args: &[&str],
) -> IlliterateOutput {
    let mut cmd =
        Command::new(env!("CARGO_BIN_EXE_illiterate"));
    cmd.arg("--source")
        .arg(source)
        .env("RUST_LOG", "illiterate=trace");
    if let Some(config) = config {
        cmd.arg("--config").arg(config);
    }
    cmd.arg(command);
    for arg in extra_args {
        cmd.arg(arg);
    }
    let output =
        cmd.output().expect("failed to run illiterate");
    IlliterateOutput {
        stdout: strip_ansi(&String::from_utf8_lossy(
            &output.stdout,
        )),
        exit_code: output.status.code(),
    }
}

impl IlliterateOutput {
    fn get_dry_run_content(
        &self,
        path: &str,
    ) -> Option<String> {
        let pattern = format!(
            r#"would emit file.*?path="{}" content="((?:[^"\\]|\\.)*)""#,
            fancy_regex::escape(path)
        );
        let re = Regex::new(&pattern).unwrap();
        re.captures(&self.stdout).ok().flatten().and_then(
            |caps| {
                let raw = caps.get(1)?.as_str();
                Some(
                    raw.replace("\\n", "\n")
                        .replace("\\\"", "\""),
                )
            },
        )
    }
}

#[test]
fn test_named_block_resolution() {
    let output = run_illiterate(
        "examples/basic",
        None,
        "check",
        &[],
    );

    assert!(
        output.stdout.contains("resolved named block"),
        "stdout should contain 'resolved named block', got:\n{}",
        output.stdout
    );
    assert!(
        output.stdout.contains("greeting"),
        "stdout should contain 'greeting' block, got:\n{}",
        output.stdout
    );
    assert!(
        output.stdout.contains("farewell"),
        "stdout should contain 'farewell' block, got:\n{}",
        output.stdout
    );
    assert_eq!(
        output.exit_code,
        Some(0),
        "exit code should be 0 for clean resolution"
    );
}

#[test]
fn test_file_block_resolution() {
    let output = run_illiterate(
        "examples/file-output",
        None,
        "tangle",
        &["--dry-run"],
    );

    let lib_content =
        output.get_dry_run_content("src/lib.rs").expect(
            "should have dry-run content for src/lib.rs",
        );
    let main_content =
        output.get_dry_run_content("src/main.rs").expect(
            "should have dry-run content for src/main.rs",
        );

    assert!(
        lib_content.contains("APP_NAME"),
        "src/lib.rs should contain APP_NAME, got:\n{lib_content}"
    );
    assert!(
        main_content.contains("my_app::run()"),
        "src/main.rs should contain my_app::run(), got:\n{main_content}"
    );
    assert_eq!(
        output.exit_code,
        Some(0),
        "exit code should be 0 for clean tangle"
    );
}

#[test]
fn test_circular_dependency_detected() {
    let output = run_illiterate(
        "examples/circular-deps",
        None,
        "check",
        &[],
    );

    assert!(
        output
            .stdout
            .contains("circular dependency detected"),
        "stdout should contain circular dependency error, got:\n{}",
        output.stdout
    );
    assert!(
        output.stdout.contains("cycle/a"),
        "circular dependency error should mention cycle/a, got:\n{}",
        output.stdout
    );
    assert!(
        output.stdout.contains("cycle/b"),
        "circular dependency error should mention cycle/b, got:\n{}",
        output.stdout
    );
    assert_eq!(
        output.exit_code,
        Some(1),
        "exit code should be 1 for circular dependencies"
    );
}

#[test]
fn test_duplicate_named_blocks_joined() {
    let output = run_illiterate(
        "examples/duplicate-blocks",
        Some("examples/illiterate.toml"),
        "check",
        &[],
    );

    // Check that the warning message appears in the output
    assert!(
        output.stdout.contains("duplicate named block 'test', joining with newline"),
        "stdout should contain duplicate block warning, got:\n{}", output.stdout
    );
    assert_eq!(
        output.exit_code,
        Some(0),
        "exit code should be 0 for clean resolution"
    );
}

#[test]
fn test_missing_reference_warning() {
    let output = run_illiterate(
        "examples/missing-refs",
        None,
        "check",
        &[],
    );

    assert!(
        output.stdout.contains("references missing block"),
        "stdout should contain missing reference warning, got:\n{}",
        output.stdout
    );
    assert!(
        output.stdout.contains("nonexistent"),
        "missing reference warning should mention 'nonexistent', got:\n{}",
        output.stdout
    );
    assert!(
        output.stdout.contains("also-missing"),
        "missing reference warning should mention 'also-missing', got:\n{}",
        output.stdout
    );
    assert_eq!(
        output.exit_code,
        Some(2),
        "exit code should be 2 for missing references"
    );
}

#[test]
fn test_comment_blocks_ignored() {
    let output = run_illiterate(
        "examples/comment-blocks",
        None,
        "check",
        &[],
    );

    assert!(
        output.stdout.contains("resolved named block"),
        "stdout should contain resolved blocks, got:\n{}",
        output.stdout
    );
    assert!(
        output.stdout.contains("normal"),
        "stdout should contain 'normal' block, got:\n{}",
        output.stdout
    );
    assert!(
        !output.stdout.contains(r#"name="hidden""#),
        "stdout should NOT contain 'hidden' block from comment, got:\n{}",
        output.stdout
    );
    assert_eq!(
        output.exit_code,
        Some(0),
        "exit code should be 0 for clean resolution"
    );
}

#[test]
fn test_deep_reference_chain() {
    let output = run_illiterate(
        "examples/nested-refs",
        None,
        "tangle",
        &["--dry-run"],
    );

    let main_content =
        output.get_dry_run_content("src/main.rs").expect(
            "should have dry-run content for src/main.rs",
        );

    assert!(
        main_content.contains("40 + 2"),
        "src/main.rs should contain resolved value '40 + 2', got:\n{main_content}"
    );
    assert!(
        main_content.contains("let y = 40 + 2"),
        "src/main.rs should contain 'let y = 40 + 2' (base ref resolved), got:\n{main_content}"
    );
    assert!(
        main_content.contains("println!(\"y = {}\", y)"),
        "src/main.rs should contain println statement, got:\n{main_content}"
    );
    assert_eq!(
        output.exit_code,
        Some(0),
        "exit code should be 0 for clean tangle"
    );
}

#[test]
fn test_entangled_syntax_compatibility() {
    let output = run_illiterate(
        "examples/entangled",
        Some("examples/entangled.toml"),
        "tangle",
        &["--dry-run"],
    );

    let main_content =
        output.get_dry_run_content("hello_world.cc").expect(
            "should have dry-run content for hello_world.cc",
        );

    assert!(
        main_content.contains("include <iostream>"),
        "hello_world.cc should contain 'include <iostream>', got:\n{main_content}"
    );
    assert!(
        main_content.contains("Hello, World!"),
        "hello_world.cc should contain 'Hello, World!' (first hello-world ref resolved), got:\n{main_content}"
    );
    assert!(
        main_content
            .contains("int main(int argc, char **argv)"),
        "hello_world.cc should contain main function definition, got:\n{main_content}"
    );
    assert!(
        main_content.contains("EXIT_SUCCESS"),
        "hello_world.cc should contain 'EXIT_SUCCESS' (second hello-world ref resolved), got:\n{main_content}"
    );
    assert_eq!(
        output.exit_code,
        Some(0),
        "exit code should be 0 for clean tangle"
    );
}

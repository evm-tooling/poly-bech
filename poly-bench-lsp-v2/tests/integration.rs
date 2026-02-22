//! Integration tests for poly-bench LSP v2
//!
//! These tests verify the LSP behavior for various scenarios.

use poly_bench_syntax::{parse, parse_with_tree, Lang, Node};

mod formatting {
    use super::*;

    #[test]
    #[ignore]
    fn test_formatting_valid_document() {
        let source = r#"suite test{description:"A test"
bench foo{go:run()}}"#;

        let file = parse(source);
        assert!(!file.has_errors());
        assert_eq!(file.suites.len(), 1);
    }

    #[test]
    #[ignore]
    fn test_formatting_with_errors_preserves_structure() {
        let source = r#"suite test {
    bench incomplete {
        go:
"#;
        let file = parse(source);

        // Should still have parsed the suite structure
        assert_eq!(file.suites.len(), 1);

        // The suite should be valid even though it contains errors
        let suite = &file.suites[0];
        assert!(suite.is_valid());
    }

    #[test]
    fn test_formatting_empty_document() {
        let file = parse("");
        assert!(file.suites.is_empty());
        assert!(!file.has_errors());
    }
}

mod semantic_tokens {
    use super::*;

    #[test]
    fn test_semantic_tokens_simple() {
        let source = r#"
suite test {
    description: "A test"
    bench foo {
        go: run()
    }
}
"#;
        let (tree, file) = parse_with_tree(source);

        // Tree should be valid
        assert!(!tree.root_node().has_error());

        // File should have content
        assert_eq!(file.suites.len(), 1);
    }

    #[test]
    fn test_semantic_tokens_with_setup() {
        let source = r#"
suite test {
    setup go {
        import (
            "fmt"
        )

        helpers {
            func helper() {}
        }
    }

    bench foo {
        go: helper()
    }
}
"#;
        let (tree, file) = parse_with_tree(source);

        // Tree should be valid
        assert!(!tree.root_node().has_error());

        // Should have setup block
        let suite = file.suites[0].as_valid().unwrap();
        assert!(suite.setups.contains_key(&Lang::Go));
    }

    #[test]
    fn test_semantic_tokens_incomplete_code() {
        let source = r#"
suite test {
    bench incomplete {
        go:
"#;
        let (tree, _file) = parse_with_tree(source);

        // Tree should have errors but still be parseable
        assert!(tree.root_node().has_error());

        // Root should still be source_file
        assert_eq!(tree.root_node().kind(), "source_file");
    }
}

mod diagnostics {
    use super::*;

    #[test]
    fn test_diagnostics_empty_suite() {
        let source = r#"
suite empty {
    description: "Empty suite"
}
"#;
        let file = parse(source);

        // Should parse successfully
        assert_eq!(file.suites.len(), 1);

        // Suite should be valid but empty
        let suite = file.suites[0].as_valid().unwrap();
        assert!(suite.benchmarks.is_empty());
        assert!(suite.fixtures.is_empty());
    }

    #[test]
    fn test_diagnostics_empty_benchmark() {
        let source = r#"
suite test {
    bench empty {
    }
}
"#;
        let file = parse(source);

        // Should parse successfully
        assert_eq!(file.suites.len(), 1);

        // Benchmark should be valid but have no implementations
        let suite = file.suites[0].as_valid().unwrap();
        assert_eq!(suite.benchmarks.len(), 1);

        let bench = suite.benchmarks[0].as_valid().unwrap();
        assert!(bench.implementations.is_empty());
    }

    #[test]
    #[ignore]
    fn test_diagnostics_syntax_error() {
        let source = r#"
suite test {
    bench foo {
        go: {
            incomplete
"#;
        let (tree, file) = parse_with_tree(source);

        // Tree should have errors
        assert!(tree.root_node().has_error());

        // Should still have parsed the suite
        assert_eq!(file.suites.len(), 1);
    }
}

mod incremental_parsing {
    use super::*;
    use poly_bench_syntax::IncrementalParser;

    #[test]
    fn test_incremental_parse_simple_edit() {
        let mut parser = IncrementalParser::new();

        let source1 = r#"
suite test {
    bench foo {
        go: run()
    }
}
"#;
        let tree1 = parser.parse(source1, None);
        assert!(!tree1.root_node().has_error());

        // Make a small edit - change "foo" to "bar"
        let source2 = r#"
suite test {
    bench bar {
        go: run()
    }
}
"#;
        let tree2 = parser.parse(source2, None);
        assert!(!tree2.root_node().has_error());
    }

    #[test]
    fn test_incremental_parse_add_benchmark() {
        let mut parser = IncrementalParser::new();

        let source1 = r#"
suite test {
    bench foo {
        go: run()
    }
}
"#;
        let tree1 = parser.parse(source1, None);
        assert!(!tree1.root_node().has_error());

        // Add another benchmark
        let source2 = r#"
suite test {
    bench foo {
        go: run()
    }

    bench bar {
        go: test()
    }
}
"#;
        let tree2 = parser.parse(source2, None);
        assert!(!tree2.root_node().has_error());
    }
}

mod partial_ast {
    use super::*;

    #[test]
    fn test_partial_ast_with_multiple_suites() {
        let source = r#"
suite first {
    bench a {
        go: runA()
    }
}

suite second {
    bench b {
        go: runB()
    }
}
"#;
        let file = parse(source);

        assert_eq!(file.suites.len(), 2);

        let first = file.suites[0].as_valid().unwrap();
        assert_eq!(first.name, "first");

        let second = file.suites[1].as_valid().unwrap();
        assert_eq!(second.name, "second");
    }

    #[test]
    fn test_partial_ast_with_fixtures() {
        let source = r#"
suite test {
    fixture data {
        hex: "deadbeef"
    }

    bench foo {
        go: process(data)
    }
}
"#;
        let file = parse(source);

        let suite = file.suites[0].as_valid().unwrap();
        assert_eq!(suite.fixtures.len(), 1);

        let fixture = suite.fixtures[0].as_valid().unwrap();
        assert_eq!(fixture.name, "data");
    }

    #[test]
    fn test_partial_ast_with_charting() {
        let source = r#"
use std::charting

suite test {
    bench foo {
        go: run()
    }

    after {
        charting.drawTable(
            title: "Results"
        )
    }
}
"#;
        let file = parse(source);

        assert_eq!(file.use_stds.len(), 1);

        let suite = file.suites[0].as_valid().unwrap();
        assert!(suite.after_block.is_some());
    }
}

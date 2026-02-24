//! CST to partial AST conversion
//!
//! This module converts Tree-sitter's concrete syntax tree (CST)
//! to our typed partial AST. The conversion is error-tolerant,
//! meaning it will produce a partial AST even when the source
//! contains syntax errors.

use crate::{partial_ast::*, tree::NodeExt};
use std::collections::HashMap;
use tree_sitter::{Node as TsNode, Tree};

/// Convert a Tree-sitter tree to a partial AST
///
/// This function always succeeds, returning a PartialFile that
/// may contain error nodes if the source had syntax errors.
pub fn convert_file(tree: &Tree, source: &str) -> PartialFile {
    let root = tree.root_node();
    let mut file = PartialFile::default();

    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        match child.kind() {
            "use_statement" => {
                file.use_stds.push(convert_use_statement(child, source));
            }
            "global_setup" => {
                file.global_setup = Some(convert_global_setup(child, source));
            }
            "suite" => {
                file.suites.push(convert_suite(child, source));
            }
            "ERROR" => {
                file.errors.push(extract_error(child, source));
            }
            "comment" => {
                // Skip comments at file level
            }
            _ => {
                // Unknown node type - could be whitespace or other extras
            }
        }
    }

    file
}

fn convert_use_statement(node: TsNode, source: &str) -> Node<UseStd> {
    let span = Span::from_node(&node);

    if node.has_error() {
        return Node::Error { span, message: "Invalid use statement".to_string() };
    }

    let module = node.field("module").map(|n| n.text(source).to_string()).unwrap_or_default();

    Node::Valid(UseStd { module, span })
}

fn convert_global_setup(node: TsNode, source: &str) -> Node<GlobalSetup> {
    let span = Span::from_node(&node);

    if node.has_error() {
        return Node::Error { span, message: "Invalid global setup".to_string() };
    }

    let mut statements = Vec::new();

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "global_setup_body" {
            let mut body_cursor = child.walk();
            for stmt in child.children(&mut body_cursor) {
                if stmt.kind() == "global_setup_statement" {
                    // Look inside global_setup_statement for the actual call
                    let mut stmt_cursor = stmt.walk();
                    for inner in stmt.children(&mut stmt_cursor) {
                        match inner.kind() {
                            "anvil_call" => {
                                statements.push(convert_anvil_call(inner, source));
                            }
                            "function_call" => {
                                statements.push(convert_function_call(inner, source));
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    Node::Valid(GlobalSetup { statements, span })
}

fn convert_anvil_call(node: TsNode, source: &str) -> Node<GlobalSetupStatement> {
    let span = Span::from_node(&node);

    let mut fork_url = None;

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "anvil_args" {
            let mut args_cursor = child.walk();
            for arg_child in child.children(&mut args_cursor) {
                if arg_child.kind() == "string" {
                    fork_url = Some(extract_string_value(arg_child, source));
                }
            }
        }
    }

    Node::Valid(GlobalSetupStatement::AnvilSpawn { fork_url, span })
}

fn convert_function_call(node: TsNode, source: &str) -> Node<GlobalSetupStatement> {
    let span = Span::from_node(&node);

    let name = node.child(0).map(|n| n.text(source).to_string()).unwrap_or_default();

    let args = Vec::new(); // TODO: Parse arguments if needed

    Node::Valid(GlobalSetupStatement::FunctionCall { name, args, span })
}

fn convert_suite(node: TsNode, source: &str) -> Node<PartialSuite> {
    let span = Span::from_node(&node);

    let name = node
        .field("name")
        .map(|n| n.text(source).to_string())
        .unwrap_or_else(|| "<unnamed>".to_string());

    let mut suite = PartialSuite::new(name, span);

    if let Some(suite_type) = node.field("suite_type") {
        suite.suite_type = Some(suite_type.text(source).to_string());
    }
    if let Some(run_mode) = node.field("run_mode") {
        suite.run_mode = Some(run_mode.text(source).to_string());
    }
    if let Some(same_dataset) = node.field("same_dataset") {
        suite.same_dataset = Some(same_dataset.text(source) == "true");
    }

    // Find suite_body
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "suite_body" {
            convert_suite_body(child, source, &mut suite);
        }
    }

    if node.has_error() {
        // Still return what we parsed, but note the error
        suite
            .properties
            .push(Node::Error { span, message: "Suite contains syntax errors".to_string() });
    }

    Node::Valid(suite)
}

fn convert_suite_body(node: TsNode, source: &str, suite: &mut PartialSuite) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "property" => {
                suite.properties.push(convert_property(child, source));
            }
            "setup_block" => {
                if let Some((lang, setup)) = convert_setup_block(child, source) {
                    if !suite.setups.contains_key(&lang) {
                        suite.setup_order.push(lang);
                    }
                    suite.setups.insert(lang, setup);
                }
            }
            "fixture" => {
                suite.fixtures.push(convert_fixture(child, source));
            }
            "benchmark" => {
                suite.benchmarks.push(convert_benchmark(child, source));
            }
            "after_block" => {
                suite.after_block = Some(convert_after_block(child, source));
            }
            "global_setup" => {
                suite.global_setup = Some(convert_global_setup(child, source));
            }
            "ERROR" => {
                // Add error but continue parsing
            }
            _ => {}
        }
    }
}

fn convert_property(node: TsNode, source: &str) -> Node<Property> {
    let span = Span::from_node(&node);

    if node.has_error() {
        return Node::Error { span, message: "Invalid property".to_string() };
    }

    let name = node.field("name").map(|n| n.text(source).to_string()).unwrap_or_default();

    let value = node
        .field("value")
        .map(|n| convert_value(n, source))
        .unwrap_or(PropertyValue::String(String::new()));

    Node::Valid(Property { name, value, span })
}

fn convert_value(node: TsNode, source: &str) -> PropertyValue {
    match node.kind() {
        "string" => PropertyValue::String(extract_string_value(node, source)),
        "number" => {
            let text = node.text(source);
            PropertyValue::Number(text.parse().unwrap_or(0))
        }
        "float" => {
            let text = node.text(source);
            PropertyValue::Float(text.parse().unwrap_or(0.0))
        }
        "duration" => {
            let mut num = 0i64;
            let mut unit = DurationUnit::Milliseconds;

            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                match child.kind() {
                    "number" => {
                        num = child.text(source).parse().unwrap_or(0);
                    }
                    "duration_unit" => {
                        unit = DurationUnit::from_str(child.text(source))
                            .unwrap_or(DurationUnit::Milliseconds);
                    }
                    _ => {}
                }
            }

            PropertyValue::Duration(num, unit)
        }
        "boolean" => {
            let text = node.text(source);
            PropertyValue::Boolean(text == "true")
        }
        "identifier" => PropertyValue::Identifier(node.text(source).to_string()),
        "string_array" | "lang_array" => {
            let mut strings = Vec::new();
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "string" {
                    strings.push(extract_string_value(child, source));
                }
            }
            PropertyValue::StringArray(strings)
        }
        _ => PropertyValue::String(node.text(source).to_string()),
    }
}

fn extract_string_value(node: TsNode, source: &str) -> String {
    let text = node.text(source);
    // Remove quotes
    if (text.starts_with('"') && text.ends_with('"')) ||
        (text.starts_with('\'') && text.ends_with('\''))
    {
        text[1..text.len() - 1].to_string()
    } else {
        text.to_string()
    }
}

fn convert_setup_block(node: TsNode, source: &str) -> Option<(Lang, Node<StructuredSetup>)> {
    let span = Span::from_node(&node);

    let lang = node.field("language").and_then(|n| Lang::from_str(n.text(source)))?;

    let mut setup = StructuredSetup::new(lang, span);

    // Find setup_body
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "setup_body" {
            convert_setup_body(child, source, &mut setup);
        }
    }

    Some((lang, Node::Valid(setup)))
}

fn convert_setup_body(node: TsNode, source: &str, setup: &mut StructuredSetup) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "import_section" => {
                setup.imports = extract_code_block_from_section(child, source);
            }
            "declare_section" => {
                setup.declare = extract_code_block_from_section(child, source);
            }
            "init_section" => {
                // Check for async
                let mut init_cursor = child.walk();
                for init_child in child.children(&mut init_cursor) {
                    if init_child.kind() == "async" {
                        setup.is_async_init = true;
                    }
                }
                setup.init = extract_code_block_from_section(child, source);
            }
            "helpers_section" => {
                setup.helpers = extract_code_block_from_section(child, source);
            }
            _ => {}
        }
    }
}

fn extract_code_block_from_section(node: TsNode, source: &str) -> Option<CodeBlock> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "code_block" || child.kind() == "paren_code_block" {
            return extract_code_block(child, source);
        }
    }
    None
}

fn extract_code_block(node: TsNode, source: &str) -> Option<CodeBlock> {
    let span = Span::from_node(&node);

    // Find embedded_code child
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "embedded_code" {
            return Some(CodeBlock { code: child.text(source).to_string(), span });
        }
    }

    // If no embedded_code, the block might be empty or inline
    let text = node.text(source);
    let code = if text.starts_with('{') && text.ends_with('}') {
        text[1..text.len() - 1].trim().to_string()
    } else if text.starts_with('(') && text.ends_with(')') {
        text[1..text.len() - 1].trim().to_string()
    } else {
        text.to_string()
    };

    if code.is_empty() {
        None
    } else {
        Some(CodeBlock { code, span })
    }
}

fn convert_fixture(node: TsNode, source: &str) -> Node<PartialFixture> {
    let span = Span::from_node(&node);

    let name = node
        .field("name")
        .map(|n| n.text(source).to_string())
        .unwrap_or_else(|| "<unnamed>".to_string());

    let mut fixture = PartialFixture::new(name, span);

    // Parse params
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "fixture_params" => {
                fixture.params = convert_fixture_params(child, source);
            }
            "fixture_body" => {
                convert_fixture_body(child, source, &mut fixture);
            }
            _ => {}
        }
    }

    if node.has_error() {
        return Node::Error { span, message: "Fixture contains syntax errors".to_string() };
    }

    Node::Valid(fixture)
}

fn convert_fixture_params(node: TsNode, source: &str) -> Vec<FixtureParam> {
    let mut params = Vec::new();

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "fixture_param" {
            let span = Span::from_node(&child);
            let name = child.field("name").map(|n| n.text(source).to_string()).unwrap_or_default();
            let type_name =
                child.field("type").map(|n| n.text(source).to_string()).unwrap_or_default();

            params.push(FixtureParam { name, type_name, span });
        }
    }

    params
}

fn convert_fixture_body(node: TsNode, source: &str, fixture: &mut PartialFixture) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "property" => {
                fixture.properties.push(convert_property(child, source));
            }
            "hex_property" => {
                fixture.hex = convert_hex_property(child, source);
            }
            "shape_property" => {
                fixture.shape = extract_code_block_from_section(child, source);
            }
            "language_implementation" => {
                if let Some((lang, code)) = convert_language_implementation(child, source) {
                    if !fixture.implementations.contains_key(&lang) {
                        fixture.impl_order.push(lang);
                    }
                    fixture.implementations.insert(lang, code);
                }
            }
            _ => {}
        }
    }
}

fn convert_hex_property(node: TsNode, source: &str) -> Option<HexData> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "string" => {
                return Some(HexData::Inline(extract_string_value(child, source)));
            }
            "file_ref" => {
                let mut file_cursor = child.walk();
                for file_child in child.children(&mut file_cursor) {
                    if file_child.kind() == "string" {
                        return Some(HexData::File(extract_string_value(file_child, source)));
                    }
                }
            }
            _ => {}
        }
    }
    None
}

fn convert_benchmark(node: TsNode, source: &str) -> Node<PartialBenchmark> {
    let span = Span::from_node(&node);

    let name = node
        .field("name")
        .map(|n| n.text(source).to_string())
        .unwrap_or_else(|| "<unnamed>".to_string());

    let kind = match node.text(source).trim_start().starts_with("benchAsync") {
        true => BenchmarkKind::Async,
        false => BenchmarkKind::Sync,
    };
    let mut benchmark = PartialBenchmark::new(name, kind, span);

    // Find benchmark_body
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "benchmark_body" {
            convert_benchmark_body(child, source, &mut benchmark);
        }
    }

    if node.has_error() {
        return Node::Error { span, message: "Benchmark contains syntax errors".to_string() };
    }

    Node::Valid(benchmark)
}

fn convert_benchmark_body(node: TsNode, source: &str, benchmark: &mut PartialBenchmark) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "property" => {
                benchmark.properties.push(convert_property(child, source));
            }
            "tags_property" => {
                benchmark.tags = convert_tags(child, source);
            }
            "skip_hook" => {
                convert_hook(child, source, &mut benchmark.skip);
            }
            "validate_hook" => {
                convert_hook(child, source, &mut benchmark.validate);
            }
            "before_hook" => {
                convert_hook(child, source, &mut benchmark.before);
            }
            "after_hook" => {
                convert_hook(child, source, &mut benchmark.after);
            }
            "each_hook" => {
                convert_hook(child, source, &mut benchmark.each);
            }
            "language_implementation" => {
                if let Some((lang, code)) = convert_language_implementation(child, source) {
                    if !benchmark.implementations.contains_key(&lang) {
                        benchmark.impl_order.push(lang);
                    }
                    benchmark.implementations.insert(lang, code);
                }
            }
            _ => {}
        }
    }
}

fn convert_tags(node: TsNode, source: &str) -> Vec<String> {
    let mut tags = Vec::new();

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "string_array" {
            let mut array_cursor = child.walk();
            for array_child in child.children(&mut array_cursor) {
                if array_child.kind() == "string" {
                    tags.push(extract_string_value(array_child, source));
                }
            }
        }
    }

    tags
}

fn convert_hook(node: TsNode, source: &str, hooks: &mut HashMap<Lang, Node<CodeBlock>>) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "hook_flat" => {
                if let Some((lang, code)) = convert_hook_flat(child, source) {
                    hooks.insert(lang, code);
                }
            }
            "hook_grouped" => {
                convert_hook_grouped(child, source, hooks);
            }
            _ => {}
        }
    }
}

fn convert_hook_flat(node: TsNode, source: &str) -> Option<(Lang, Node<CodeBlock>)> {
    let lang = node.field("language").and_then(|n| Lang::from_str(n.text(source)))?;

    let span = Span::from_node(&node);

    // Find code_block or inline_code
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "code_block" => {
                if let Some(code) = extract_code_block(child, source) {
                    return Some((lang, Node::Valid(code)));
                }
            }
            "inline_code" => {
                let code = CodeBlock {
                    code: child.text(source).trim().to_string(),
                    span: Span::from_node(&child),
                };
                return Some((lang, Node::Valid(code)));
            }
            _ => {}
        }
    }

    Some((lang, Node::Error { span, message: "Missing code in hook".to_string() }))
}

fn convert_hook_grouped(node: TsNode, source: &str, hooks: &mut HashMap<Lang, Node<CodeBlock>>) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "language_implementation" {
            if let Some((lang, code)) = convert_language_implementation(child, source) {
                hooks.insert(lang, code);
            }
        }
    }
}

fn convert_language_implementation(node: TsNode, source: &str) -> Option<(Lang, Node<CodeBlock>)> {
    let lang = node.field("language").and_then(|n| Lang::from_str(n.text(source)))?;

    let span = Span::from_node(&node);

    // Find code_block or inline_code
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "code_block" => {
                if let Some(code) = extract_code_block(child, source) {
                    return Some((lang, Node::Valid(code)));
                }
            }
            "inline_code" => {
                let code = CodeBlock {
                    code: child.text(source).trim().to_string(),
                    span: Span::from_node(&child),
                };
                return Some((lang, Node::Valid(code)));
            }
            _ => {}
        }
    }

    Some((lang, Node::Error { span, message: "Missing code in implementation".to_string() }))
}

fn convert_after_block(node: TsNode, source: &str) -> Node<AfterBlock> {
    let span = Span::from_node(&node);
    let mut directives = Vec::new();

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "after_body" {
            let mut body_cursor = child.walk();
            for body_child in child.children(&mut body_cursor) {
                if body_child.kind() == "chart_directive" {
                    directives.push(convert_chart_directive(body_child, source));
                }
            }
        }
    }

    Node::Valid(AfterBlock { directives, span })
}

fn convert_chart_directive(node: TsNode, source: &str) -> Node<ChartDirective> {
    let span = Span::from_node(&node);

    let function = node.field("function").map(|n| n.text(source).to_string()).unwrap_or_default();

    let mut params = HashMap::new();
    let mut param_order = Vec::new();

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "chart_params" {
            let mut params_cursor = child.walk();
            for param_child in child.children(&mut params_cursor) {
                if param_child.kind() == "chart_param" {
                    let name = param_child
                        .field("name")
                        .map(|n| n.text(source).to_string())
                        .unwrap_or_default();
                    let value = param_child
                        .field("value")
                        .map(|n| convert_value(n, source))
                        .unwrap_or(PropertyValue::String(String::new()));
                    // Track parameter order before inserting
                    if !params.contains_key(&name) {
                        param_order.push(name.clone());
                    }
                    params.insert(name, value);
                }
            }
        }
    }

    Node::Valid(ChartDirective { function, params, param_order, span })
}

fn extract_error(node: TsNode, source: &str) -> ParseError {
    let span = Span::from_node(&node);
    let text = node.text(source);

    let message = if text.len() > 50 {
        format!("Syntax error near: {}...", &text[..50])
    } else {
        format!("Syntax error near: {}", text)
    };

    ParseError::new(span, message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::IncrementalParser;

    fn parse_source(source: &str) -> PartialFile {
        let mut parser = IncrementalParser::new();
        let tree = parser.parse(source, None);
        convert_file(&tree, source)
    }

    #[test]
    fn test_convert_simple_suite() {
        let source = r#"
declare suite test performance timeBased sameDataset: true {
    description: "A test"
    iterations: 100
    
    bench foo {
        go: run()
    }
}
"#;
        let file = parse_source(source);
        assert_eq!(file.suites.len(), 1);

        let suite = file.suites[0].as_valid().unwrap();
        assert_eq!(suite.name, "test");
        assert_eq!(suite.benchmarks.len(), 1);
    }

    #[test]
    fn test_convert_with_setup() {
        let source = r#"
declare suite test performance timeBased sameDataset: true {
    setup go {
        helpers {
            func helper() {}
        }
    }
    
    bench foo {
        go: helper()
    }
}
"#;
        let file = parse_source(source);
        let suite = file.suites[0].as_valid().unwrap();
        assert!(suite.setups.contains_key(&Lang::Go));
    }

    #[test]
    #[ignore]
    fn test_convert_with_errors() {
        let source = r#"
declare suite test performance timeBased sameDataset: true {
    bench incomplete {
        go:
"#;
        let file = parse_source(source);

        // Should still have parsed the suite
        assert_eq!(file.suites.len(), 1);
    }

    #[test]
    fn test_convert_with_global_setup() {
        let source = r#"
use std::anvil

globalSetup {
    anvil.spawnAnvil()
}

declare suite test performance timeBased sameDataset: true {
    globalSetup {
        anvil.spawnAnvil(fork: "https://mainnet.infura.io")
    }
    
    bench foo {
        go: run()
    }
}
"#;
        let file = parse_source(source);

        // Check file-level globalSetup
        assert!(file.global_setup.is_some(), "File-level globalSetup should exist");
        let gs = file.global_setup.as_ref().unwrap().as_valid().unwrap();
        assert_eq!(gs.statements.len(), 1, "File-level globalSetup should have 1 statement");

        // Check suite-level globalSetup
        let suite = file.suites[0].as_valid().unwrap();
        assert!(suite.global_setup.is_some(), "Suite-level globalSetup should exist");
        let suite_gs = suite.global_setup.as_ref().unwrap().as_valid().unwrap();
        assert_eq!(suite_gs.statements.len(), 1, "Suite-level globalSetup should have 1 statement");
    }
}

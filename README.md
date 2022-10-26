# pulldown-cmark-minghu6

<!-- [![Docs](https://docs.rs/pulldown-cmark/badge.svg)](https://docs.rs/pulldown-cmark)
[![Crates.io](https://img.shields.io/crates/v/pulldown-cmark.svg?maxAge=2592000)](https://crates.io/crates/pulldown-cmark) -->

## Brief

It's a pulldown-cmark fork by minghu6.
Its goal is to enable more flexiable coding and feature.


## Example

Example usage:

```rust
// Create parser with example Markdown text.
let markdown_input = "hello world";
let parser = pulldown_cmark::Parser::new(markdown_input);

// Write to a new String buffer.
let mut html_output = String::new();
pulldown_cmark::html::push_html(&mut html_output, parser);
assert_eq!(&html_output, "<p>hello world</p>\n");
```


## Authors

The main author is Raph Levien. The implementation of the new design (v0.3+) was completed by Marcus Klaas de Vries.

The markdown writer is from [pulldown-cmark-to-cmark](https://github.com/Byron/pulldown-cmark-to-cmark) with its author Byron and contributors associated.

This fork author is minghu6.

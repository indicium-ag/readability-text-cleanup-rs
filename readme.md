# Readability Text Cleanup

This projectis a Rust library designed to enhance the readability of text by cleaning up HTML content, unescaping HTML entities, replacing abbreviations, and splitting text into paragraphs. It leverages the power of Rust for efficient text processing and aims to provide a comprehensive solution for preparing text for further analysis or display. Additionally, it is compiled to WebAssembly (Wasm) and published to npm, making it accessible for use in Node.js environments.

## Features

- **HTML Cleanup**: Removes HTML tags and entities, ensuring that the text is free from any HTML formatting.
- **HTML Entity Unescaping**: Converts HTML entities (e.g., `&amp;` to `&`) to their corresponding characters.
- **Abbreviation Replacement**: Replaces common abbreviations with their full forms for better readability.
- **Text Segmentation**: Splits the text into paragraphs, making it easier to process and analyze.
- **Customizable**: Offers a range of functions for specific text processing needs, from simple HTML tag removal to more complex sentence splitting and repair.
- **WebAssembly Compatibility**: Compiled to Wasm for use in web and Node.js environments.

## Getting Started

To use this library in your Rust project, add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
readability-text-cleanup = "0.1.0" # Use the latest version
```

For Node.js projects, you can install the npm package:

```bash
npm install readability-text-cleanup
```

## Usage

### Rust

Here's a basic example of how to use the library to prepare text in Rust:

```rust
use readability_text_cleanup_rs::prepare_text;

fn main() {
    let html_content = r#"<p>This is a <strong>sample</strong> HTML content.</p>"#;
    let cleaned_text = prepare_text(html_content);
    println!("{}", cleaned_text);
}
```

This will output:

```
This is a sample HTML content.
```

### Node.js

To use the library in a Node.js project, you can import it and use it as follows:

```javascript
const { prepareText } = require('readability-text-cleanup');

const htmlContent = '<p>This is a <strong>sample</strong> HTML content.</p>';
const cleanedText = prepareText(htmlContent);
console.log(cleanedText);
```

This will output:

```
This is a sample HTML content.
```

## License

This project is licensed under the MIT license.

## Acknowledgments

- The `html2md` library for HTML to Markdown conversion.
- The `regex` library for powerful text processing capabilities.
- The `katana` module for advanced text segmentation.
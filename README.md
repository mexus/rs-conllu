# rs-conllu

This project aims to provide a parser for the CoNLL-U format of the Universal Dependencies project: https://universaldependencies.org/format.html.

## Basic Usage

Parse a file in ConLL-U format and iterate over the containing sentences.

```rust
let file = File::open("example.conllu").unwrap();

let doc = parse_file(file);

for sentence in doc {
    for token in sentence.unwrap() {
        println!("{}", token.form);
    }
}
```

## Limitations

Parsing happens in a "flat" manner, relations between tokens are not respected.

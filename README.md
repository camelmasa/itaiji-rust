# itaiji-rust

[![Build Status](https://travis-ci.org/camelmasa/itaiji-rust.svg?branch=master)](https://travis-ci.org/camelmasa/itaiji-rust)

Convert japanese from itaiji(異体字) to seijitai(正字体) for Rust.


## Install

```
[dependencies]
itaiji = "0.1.0"
```


## Usage

```rust
use itaiji::Converter;

fn main() {
    let converter = Converter::new();
    converter.seijitai("齊藤"); // 斉藤
    converter.itaiji("斉藤"); // 齊藤
}
```
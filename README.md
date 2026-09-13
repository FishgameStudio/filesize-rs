# filesize-rs

![Stars](https://img.shields.io/github/stars/FishgameStudio/filesize-rs)
![Issues](https://img.shields.io/github/issues/FishgameStudio/filesize-rs)
![PRs](https://img.shields.io/github/issues-pr/FishgameStudio/filesize-rs)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange?logo=rust)](rust-lang.org)
[![License](https://img.shields.io/github/license/FishgameStudio/filesize-rs)](LICENSE)

**filesize-rs** is a **extremely lightweight** library to get file and directory sizes, with **zero dependencies**. The core implementation is only **~200 lines**, helping you avoid reinventing this small wheel.

## 🚀 Getting Started

### 📦 Installation
```sh
# Download from crates.io
cargo add filesize-rs
# Or install from forked repository
cargo add --path .
```

### 💡 Usage

You can import this crate and use it in your daily development. Here's a simple example to use it.
```rust
use filesize_rs::{get_size_pretty, Result};

use std::io::stdin;

fn main() -> Result<()> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;
    let path = buf.trim();
    println!("Size: {}", get_size_pretty(path)?);
    Ok(())
}
```

## 🤝 Contributing

Contributions are **greatly appreciated**! 
If you have a suggestion that would make this better, please fork the repo and create a pull request. 

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### 🌟 Top contributors:

<a href="https://github.com/FishgameStudio/filesize-rs/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=FishgameStudio/filesize-rs" alt="contrib.rocks image" />
</a>

## 📃 License

[MIT](https://mit-license.org) Licensed. See [LICENSE](LICENSE) for more information.

## 📬 Contact

Nicola Grey - [popxh@outlook.com](mailto:popxh@outlook.com)

Project Link: [https://github.com/FishgameStudio/filesize-rs](https://github.com/FishgameStudio/filesize-rs)

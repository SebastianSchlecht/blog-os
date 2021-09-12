# blog-os
Blog:OS that is heavily influenced by https://os.phil-opp.com/  
See also his git repository: https://github.com/phil-opp/blog_os

# Building
To build the project a nightly version of Rust is required. `rustup update nightly --force`  

The project can be build by running:
```
cargo build
```
To create a bootable disk image `bootimage` is used
```
cargo install bootimage
cargo bootimage
```
# Running
This project runs via QEMU on your development pc
```
cargo run
```
QEMU and `bootimage` have to be installed.

# License
Licensed under [LICENSE-MIT](LICENSE-MIT)  

This project uses third party code:
* original code by phill-opp: [LICENSE-MIT](LICENSE-MIT) 

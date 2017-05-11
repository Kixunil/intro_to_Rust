Notes on unsafe
===============

Rust is kind of magical but lot of it's magic is hidden in unsafe blocks. These blocks allow programmer to break some rules. This differentiates Rust from C. In C everything is super-unsafe. In Rust almost everything is safe with small exceptions clearly marked as `unsafe`. We aren't going to learn this. Just wanted to let you know this important difference. If you ever encounter a memory bug in Rust, you can be sure the bad code is clearly marked as `unsafe`.

Before we dive into code, let's discuss error handling in extra-error_handling.

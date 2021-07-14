# WebAssembly from Scratch: From FizzBuzz to DooM

Exploring WebAssembly from scratch from a backend-person-point-of-view. A story in four acts.

---

Welcome to my journey where I will explore some [WebAssembly](https://webassembly.org/).
We will start from scratch.
My goal is to look at everything bottom-up, without much magic tooling or frameworks on our way.
In this article, I prefer to build everything as much from scratch as possible.

Non-goal: We will *not* implement our own WebAssembly interpreter or runtime, but use existing ones, e.g., [wabt](https://github.com/WebAssembly/wabt) or the Chrome and Firefox browsers.

---

* Part 1) [Introduction to WebAssembly](intro_examples/)
* Part 2) [Implementing FizzBuzz with WebAssembly by Hand](wat/)
* Part 3) [Implementing FizzBuzz in Rust and Compiling to WebAssembly](rust/)
* Part 4) [Porting 1997 DooM to WebAssembly from Scratch](doom/)

---

Demo at <https://diekmann.github.io/wasm-fizzbuzz/rust/>, deployed from (Part 3) via GitHub Actions to GitHub Pages.

Play doom at <https://diekmann.github.io/wasm-fizzbuzz/doom/>, deployed from (Part 4) via GitHub Actions to GitHub Pages..


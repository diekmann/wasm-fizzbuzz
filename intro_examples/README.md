# WebAssembly Crash Course

Looking at WebAssembly.

![A small WebAssembly program in the WebAssembly Text Format which adds two numbers and can be called from the command-line](../imgs/wasm1.png)

src: [addtwo.wat](addtwo.wat)

---

No jumps to random locations! :heart_eyes:
Basically, the call graph is known at load time.

![A small WebAssembly loop](../imgs/wasm2.png)

src: [blocks.wat](blocks.wat)

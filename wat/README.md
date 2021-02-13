# Writing WebAssembly By Hand

This doc is based on <https://developer.mozilla.org/en-US/docs/WebAssembly>, in particular, <https://developer.mozilla.org/en-US/docs/WebAssembly/Understanding_the_text_format>.



---

Let's see if we can implement FizzBuzz. Starting with a loop to print the numbers 0..99.

![A small WebAssembly loop and a Makefile](../imgs/wasm_fizzbuzz1.png)

---

Here is the fizz buzz logic. The next step will be printing strings instead of the `i32` placeholders. This will be challenging, since WebAssembly does not have a native string type, nor does `wasm-interp` provide a way to print strings AFAICT.

![FizzBuzz logic, with placeholders for strings](../imgs/wasm_fizzbuzz2.png)

---

To get strings out of wasm, looks like state of the art (non-representative survey) is to export the complete wasm memory (or import memory from the host environment) and share pointers into wasm memory as `i32` with the host environment.
I don't want to work with shared memory,...

---

I need more support from the host environment to print strings. Let's switch from `wasm-interp` to a browser. No change to the wasm code.
The fizzbuzz (still with numbers instead of strings) shows up in the JavaScript console.

![wat, Makefile, and index.html](../imgs/wasm_fizzbuzz3.png)
![browser](../imgs/wasm_fizzbuzz4.png)

---

For reasons I don't understand, the .wasm file is executed, but the Chrome (version 87.0.4280.88 64bit Linux) dev tools debugger refuses to load the file.
![Chrome where wasm file does not load](../imgs/wasm_fizzbuzz5.png)

---

When I ask Chrome to `"Save as..."` the `fizzbuzz.wasm` and base64 decode it (wtf? Why is it base64 encoded? The python webserver definitely serves it as binary with `Content-type: application/wasm`) I get the same `fizzbuzz.wasm` I expect.
Let's ignore this, no wasm debugger for now.

Narrator: firefox will be my debugger

---

Calling the main function from JavaScript instead of `(start $main)` does not seem to make a difference. So let's not do this.
![wat, Makefile, and index.html](../imgs/wasm_fizzbuzz6.png)

---

Let's get the FizzBuzz running! I'm cheating somewhat, since I moved the string processing to JavaScript. No changes to the wasm code.

![wat code](../imgs/wasm_fizzbuzz7.png)
![browser showing FizzBuzz in the JavaScript console](../imgs/wasm_fizzbuzz8.png)

---

Since I don't want to communicate via shared memory, I'll write my own message-passing API :woman_shrugging:. Probably `putchar` and `getchar` will serve as API between browser and wasm.

Since I don't want to share memory with the host environment, WebAssembly only supports a limited set of types we can share with the host environment.
Basically only `i32`, `i64` and floats (both 32bit and 64bit).
To send a string from WebAssembly to the browser, we will transmit one character as `i32` at a time from WebAssembly to the browser.
(This is a from-scratch demo, you probably don't want to implement this API in production)

BRB, coding an ugly terminal. Let's start by getting rid of `console.log`. No change to wasm code.

![wat code](../imgs/wasm_fizzbuzz9.png)
![browser](../imgs/wasm_fizzbuzz10.png)

---

By providing a `putchar` function from javascript, the WebAssembly code can directly write chars to the HTML. One char at a time, but I guess I gave up on performance for this demo long ago.
Now, the string functions can be moved to WebAssembly.

![wat code and index.html](../imgs/wasm_fizzbuzz11.png)

---

These may be the worst string functions you will see today. But I know that you will take a look :stuck_out_tongue_winking_eye:

![wat code and index.html](../imgs/wasm_fizzbuzz12.png)
![FizzBuzz in the browser](../imgs/wasm_fizzbuzz13.png)

---

Time to switch to a real terminal emulator.
Thanks to [@larsr_h](http://twitter.com/larsr_h) for teaching me how to avoid `npm` with unpkg.

![wat code and index.html](../imgs/wasm_fizzbuzz14.png)
![FizzBuzz in the browser](../imgs/wasm_fizzbuzz15.png)

---

Terminal emulator in the browser, this is how the web was meant to be, right?
Maybe I should wire this wasm to a real serial terminal via WebUSB?
I hope nobody notices that I'm a backend person. I have no frontend skills.

:impostor:

Demo at <https://diekmann.github.io/wasm-fizzbuzz/wat/>.

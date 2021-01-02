;; Using the WebAssembly Binary Toolkit: github.com/WebAssembly/wabt
;; wat2wasm addtwo.wat && wasm-interp --host-print addtwo.wasm
(module
  ;; WebAssembly does not provide any IO functions, nor functions to interact with the environment in general. Those functions need to be provided externally.
  ;; The wabt wasm-interp can provide a print function for us. The host.print function takes an i32 (which will be printed) and returns an i32 (which we will ignore).
  ;; We give the function the internal index $hprint, which we will use to call it.
  (import "host" "print" (func $hprint (param i32) (result i32)))

  ;; Defining an internal (non-exported) function, identified by index $addTwo.
  ;; In the resulting binary, the identifier "$addTwo" is compiled away and this function is only known as "function number 0".
  (func $addTwo (param i32 i32) (result i32)
    ;; wasm execution is defined as stack machine.
    local.get 0 ;; push function parameter 0 (an i32) to the stack
    local.get 1 ;; push function parameter 1 (an i32) to the stack
    i32.add     ;; add the two numbers on top of the stack, pushing the sum of those back on the stack.
    ;; Only one i32 (the sum of the two parameters) is left on the stack. This is what we return. wasm type checks and validates this claim!
  )
  ;; wasm is statically type checked and validated at load time.
  ;; For example, the function signature of $addTwo promises (param i32 i32) (result i32), i.e. two i32 must be on the stack and the function will return one i32.
  ;; The wasm interpreter will reject the code (before executing) if there is any chance that the stack may look different than promised by the type signature.

  ;; $main is an alias for "function number 1".
  ;; For the sake of example, we make this function publicly available as "main", too. Literally, the string "main", not some numeric index.
  ;; This allows to export the function externally. Naming and exporting the main function is not required and just done here as example.
  (func $main (export "main")
    i32.const 40
    i32.const 2
    call $addTwo
    call $hprint ;; should print i32:42 (and return 0, which we ignore)
    drop ;; ignore the return value of the $hprint function
    )

  ;; The first function to be called is $main.
  (start $main)
)

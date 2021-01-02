(module
  (import "host" "print" (func $hprint (param i32) (result i32)))

  (func $main
    (local $i i32) ;; local zero-initialized variable, referenceable by index $i.

    ;; Remember that wasm is statically type-checked and that the stack must always look as expected?
    ;; This is also the reason why it is not possible in wasm to jump to arbitrary locations
    ;; and all basic blocks (e.g., `func`, `block`, `loop`, ...) need to be explicitly declared
    ;; and have a type signature. Here, since we don't specify a type signature for `block` and `loop`,
    ;; from the caller's point of view, the `block` and `loop` must basically leave the stack unmodified.
    block $outerLoop
    loop $theLoop
      local.get $i
      call $hprint ;; print $i, i.e., 0,1,2,...
      drop ;; ignore $hprint's return value

      ;; Instead of explicitly pushing parameters on the stack, wasm text format also allows for a syntax which looks more like calling functions with parameters.
      ;; The following increments $i by one.
      (local.set $i (i32.add (local.get $i) (i32.const 1)))

      local.get $i
      i32.const 3
      i32.ge_u
      br_if $outerLoop ;; break depth 1, i.e. breaking out of the loop and finishing if $i >= 3.

      br $theLoop ;; break depth 0, i.e. continuing the loop.
    end
    end
    )

  (start $main)
)

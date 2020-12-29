(module

;; debugging
(import "host" "print" (func $hprint (param i32)))

(func $fizzbuzz (param i32)
)

(func $main
  (local $i i32) ;; local zero-initialized variable

  loop $countToHundred
    local.get $i
    call $hprint

    (local.set $i (i32.add (local.get $i) (i32.const 1))) ;; increment $i

    (i32.lt_s (local.get $i) (i32.const 100))
    br_if $countToHundred
  end
)
(start $main)
)

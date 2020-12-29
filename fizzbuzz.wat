(module

;; debugging
(import "host" "print" (func $hprint (param i32)))

;; TODO: print strings instead of numbers
(func $fizzbuzz (param $n i32)
    (i32.eqz (i32.rem_u (local.get $n) (i32.const 15))) ;; $n % 15 == 0
    if
      (call $hprint (i32.const -3)) ;; place holder for "fizzbuzz"
      return
    end

    (i32.eqz (i32.rem_u (local.get $n) (i32.const 5))) ;; $n % 5 == 0
    if
      (call $hprint (i32.const -2)) ;; place holder for "buzz"
      return
    end

    (i32.eqz (i32.rem_u (local.get $n) (i32.const 3))) ;; $n % 3 == 0
    if
      (call $hprint (i32.const -1)) ;; place holder for "fizz"
      return
    end

    (call $hprint (local.get $n))
)

(func $main
  (local $i i32) ;; local zero-initialized variable

  loop $countToHundred
    local.get $i
    call $fizzbuzz

    (local.set $i (i32.add (local.get $i) (i32.const 1))) ;; increment $i

    (br_if $countToHundred (i32.lt_s (local.get $i) (i32.const 100))) ;; continue loop if $i < 100
  end
)
(start $main)
)

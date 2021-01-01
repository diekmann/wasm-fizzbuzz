(module
(import "host" "putchar" (func $putchar (param i32)))

(func $puts (param $ptr i32) (param $size i32)
  ;; no empty string plz. I.e. $size must be > 0.
  loop $nextChar
    (i32.load8_u (local.get $ptr))
    call $putchar

    (local.set $ptr (i32.add (local.get $ptr) (i32.const 1)))

    (local.tee $size (i32.sub (local.get $size) (i32.const 1)))
    (i32.gt_s (i32.const 0))
    br_if $nextChar
  end
  (call $putchar (i32.const 10)) ;; ASCII '\n'
)

;; converts $i (from 0..9) to the corresponding single ASCII character.
(func $digitToASCII (param $i i32) (result i32)
    (i32.ge_u (local.get $i) (i32.const 10)) ;; (unsigned)$i >= 10
    if
      i32.const 63 ;; ASCII '?'
      return
    end
    local.get $i
    i32.const 48 ;; ASCII '0'
    i32.add
)

(memory 1)
(data (i32.const 0) "ABCDEFGHIJKLMN") ;; scratch space for intToASCII
(data (i32.const 42) "fizzbuzz") ;; hardcoded at memory address 42.

;; converts $i to ASCII, placing the string at memory location 0, returning the length
;; of the string.
(func $intToASCII (param $i i32) (result i32)
  (local $length i32) ;; length of ASCII string.
  (local $div i32)
  (local.set $div (i32.const 1000000000)) ;; yolo.
  (i32.lt_s (local.get $i) (i32.const 0)) ;; $i < 0
  if
    (i32.store8 (i32.const 0) (i32.const 45)) ;; ASCII '-'
    (local.set $i (i32.mul (local.get $i) (i32.const -1))) ;; $i *= -1 (large $i bug!)
    (local.set $length (i32.const 1))
  end
  loop $nextDigit
      (local.get $length) ;; param0, i.e. pointer, for i32.store8
      (call $digitToASCII (i32.div_u (local.get $i) (local.get $div)))
      i32.store8 ;; write digitToASCII($i/$div) to memory at location $length
      (local.set $i (i32.rem_u (local.get $i) (local.get $div))) ;; $i %= $div
      (local.set $div (i32.div_u (local.get $div) (i32.const 10))) ;; $div /= 10
      (local.set $length (i32.add (local.get $length) (i32.const 1))) ;; ++$length
      (i32.ge_s (local.get $div) (i32.const 1)) ;; $div >= 1
      br_if $nextDigit
  end
  local.get $length
)

(func $fizzbuzz (param $n i32)
    (i32.eqz (i32.rem_u (local.get $n) (i32.const 15))) ;; $n % 15 == 0
    if
      (call $puts (i32.const 42) (i32.const 8)) ;; "fizzbuzz"
      return
    end
    (i32.eqz (i32.rem_u (local.get $n) (i32.const 5))) ;; $n % 5 == 0
    if
      (call $puts (i32.const 46) (i32.const 4)) ;; skip "fizz", point to "buzz"
      return
    end
    (i32.eqz (i32.rem_u (local.get $n) (i32.const 3))) ;; $n % 3 == 0
    if
      (call $puts (i32.const 42) (i32.const 4)) ;; "fizz"
      return
    end

    (call $puts (i32.const 0) (call $intToASCII (local.get $n)))
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

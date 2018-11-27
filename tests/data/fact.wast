(func (export "fac") (param $x i64) (result i64)
  (if (result i64) (i64.eq (get_local $x) (i64.const 0))
    (then (i64.const 1))
    (else
      (i64.mul (get_local $x) (call 0 (i64.sub (get_local $x) (i64.const 1))))
    )
  )
)
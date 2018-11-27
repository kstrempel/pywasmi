(module
  (func $add (param $lhs f64) (param $rhs f64) (result f64)
    get_local $lhs
    get_local $rhs
    f64.add)
  (export "add" (func $add))
)
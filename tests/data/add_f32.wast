(module
  (func $add (param $lhs f32) (param $rhs f32) (result f32)
    get_local $lhs
    get_local $rhs
    f32.add)
  (export "add" (func $add))
)
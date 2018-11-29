use wasmi::{ModuleRef, NopExternals, RuntimeValue, ExternVal, ValueType};
use wasmi::nan_preserving_float::{F32, F64};

use pyo3::types::PyTuple;
use pyo3::{Python, ToPyObject};
use pyo3::prelude::*;


#[pyclass]
pub struct WASMInstance {
    pub instance: ModuleRef
}

#[pymethods]
impl WASMInstance {
    fn invoke(&self, method: String, py_args: Option<&PyTuple>) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();        

        let mut args = Vec::new();
        if let Some(tuple) = py_args {
            let function = self.instance.export_by_name(&method);
            let signature = get_params(&function);
            args = create_args(tuple, signature);
        }

        let result = self.instance.invoke_export(
            &method,
            &args,
            &mut NopExternals).expect("Failed to execute export");
                
        if let Some(result) = result {
            match result {
                RuntimeValue::I32(i) => return Ok(i.to_object(py).into_object(py)),
                RuntimeValue::I64(l) => return Ok(l.to_object(py).into_object(py)),
                RuntimeValue::F32(f) => return Ok(f.to_float().to_object(py).into_object(py)),
                RuntimeValue::F64(d) => return Ok(d.to_float().to_object(py).into_object(py))
            }            
        }

        Ok("hello".to_object(py).into_object(py))
    }
}


fn create_args(args: &PyTuple, signature: &[ValueType]) -> Vec<RuntimeValue> {
    let mut result = Vec::new();
    for (arg, wasm_type) in args.iter().zip(signature) {
        let py_type = arg.get_type();
        let type_name = py_type.name().into_owned();
        if type_name == "int" {
            match wasm_type {
                ValueType::I32 => {
                    let i32_ = i32::extract(arg).expect("Conversion failed");
                    result.push(RuntimeValue::from(i32_));
                },
                ValueType::I64 => {
                    let i64_ = i64::extract(arg).expect("Conversion failed");
                    result.push(RuntimeValue::from(i64_));
                }
                _ => println!("int type given but not wanted")
            }
        }        
        if type_name == "float" {
            match wasm_type {
                ValueType::F32 => {
                    let f32_ = f32::extract(arg).expect("Conversion failed");
                    result.push(RuntimeValue::from(F32::from_float(f32_)));
                },
                ValueType::F64 => {
                    let f64_ = f64::extract(arg).expect("Conversion failed");
                    result.push(RuntimeValue::from(F64::from_float(f64_)));
                }
                _ => println!("int type given but not wanted")
            }
        }        
    }

    result
}

fn get_params<'a>(signature: &'a Option<ExternVal>) -> &'a [ValueType] {
    if let Some(signature) = signature {
        if let ExternVal::Func(signature) = signature {
            return signature.signature().params();
        }
    }
    &[]
}


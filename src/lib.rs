#![feature(custom_attribute)]
extern crate wasmi;
extern crate uuid;

#[macro_use]
extern crate pyo3;

use std::collections::HashMap;

use uuid::Uuid;
use wasmi::{Module, ModuleInstance, ImportsBuilder, ModuleRef, NopExternals, RuntimeValue, ExternVal, ValueType};
use wasmi::nan_preserving_float::{F32, F64};

use pyo3::types::PyTuple;
use pyo3::FromPyObject;
use pyo3::prelude::*;

type ModulesMap = HashMap<String, Module>;
type OptionModulesMap = Option<ModulesMap>;

type InstanceMap = HashMap<String, ModuleRef>;
type OptionInstanceMap = Option<InstanceMap>;

static mut MODULES : OptionModulesMap = None;
static mut INSTANCES : OptionInstanceMap = None;


#[pymodinit]
fn pywasmi_lib(_py: Python, m: &PyModule) -> PyResult<()> {

    m.add_function(wrap_function!(module_from_buffer))?;
    m.add_function(wrap_function!(create_instance))?;
    m.add_function(wrap_function!(invoke_export))?;

    unsafe {
        MODULES = Some(ModulesMap::new());
        INSTANCES = Some(InstanceMap::new());
    }

    Ok(())
}

#[pyfunction]
fn module_from_buffer(buffer: Vec<u8>) -> PyResult<String> {
    let module = Module::from_buffer(&buffer).expect("Failed to load wasm");
    let uuid = Uuid::new_v4().to_hyphenated().to_string();
    unsafe {
        if let Some(ref mut m) = MODULES {
            m.insert(uuid.clone(), module);
        }
    }
    Ok(uuid)
}

#[pyfunction]
fn create_instance(module_id: String) -> PyResult<String> {
    let uuid = Uuid::new_v4().to_hyphenated().to_string();    
    unsafe {
        if let Some(ref m) = MODULES {
            let module = m.get(&module_id).expect("Module not found");
            let instance = ModuleInstance::new(
                module, 
                &ImportsBuilder::default()
            )
            .expect("Failing to create instance")
            .assert_no_start();

            if let Some(ref mut i) = INSTANCES {
                i.insert(uuid.clone(), instance);
            }
        }
    }

    Ok(uuid)
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

#[pyfunction]
fn invoke_export(_py: Python, instance_id: String, method: String, py_args: Option<&PyTuple>) -> PyResult<PyObject> {
    unsafe {
        if let Some(ref i) = INSTANCES {
            let instance = i.get(&instance_id).expect("Unknonw instance id");

            let mut args = Vec::new();
            if let Some(tuple) = py_args {
                let function = instance.export_by_name(&method);
                let signature = get_params(&function);
                args = create_args(tuple, signature);
            }

            let result = instance.invoke_export(
                &method,
                &args,
                &mut NopExternals).expect("Failed to execute export");
            
            if let Some(result) = result {
                match result {
                    RuntimeValue::I32(i) => return Ok(i.to_object(_py).into_object(_py)),
                    RuntimeValue::I64(l) => return Ok(l.to_object(_py).into_object(_py)),
                    RuntimeValue::F32(f) => return Ok(f.to_float().to_object(_py).into_object(_py)),
                    RuntimeValue::F64(d) => return Ok(d.to_float().to_object(_py).into_object(_py))
                }            
            }
        }
    }
    Ok("hello".to_object(_py).into_object(_py))
}


extern crate wasmi;
extern crate uuid;
#[macro_use]
extern crate cpython;

use std::collections::HashMap;

use uuid::Uuid;
use wasmi::{Module, ModuleInstance, ImportsBuilder, ModuleRef, NopExternals, RuntimeValue};
use wasmi::nan_preserving_float::{F32, F64};
use cpython::{Python, PyResult, PythonObject, PyObject, ToPyObject, FromPyObject, PyTuple};

type ModulesMap = HashMap<String, Module>;
type OptionModulesMap = Option<ModulesMap>;

type InstanceMap = HashMap<String, ModuleRef>;
type OptionInstanceMap = Option<InstanceMap>;

static mut MODULES : OptionModulesMap = None;
static mut INSTANCES : OptionInstanceMap = None;


py_module_initializer!(pywasmi_lib, initpywasmi_lib, PyInit_pywasmi_lib, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "module_from_buffer", py_fn!(py, module_from_buffer(buffer: Vec<u8>)))?;
    m.add(py, "create_instance", py_fn!(py, create_instance(module_id: String)))?;
    m.add(py, "invoke_export", py_fn!(py, invoke_export(instance_id: String, method: String, args: PyTuple)))?;
    unsafe {
        MODULES = Some(ModulesMap::new());
        INSTANCES = Some(InstanceMap::new());
    }
    Ok(())
});

fn module_from_buffer(_: Python, buffer: Vec<u8>) -> PyResult<String> {
    let module = Module::from_buffer(&buffer).expect("Failed to load wasm");
    let uuid = Uuid::new_v4().to_hyphenated().to_string();
    unsafe {
        if let Some(ref mut m) = MODULES {
            m.insert(uuid.clone(), module);
        }
    }
    Ok(uuid)
}

fn create_instance(_: Python, module_id: String) -> PyResult<String> {
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

fn create_args(py: Python, args: PyTuple) -> Vec<RuntimeValue> {
    let mut result = Vec::new();
    for arg in args.iter(py) {
        let py_type = arg.get_type(py);
        let type_name = py_type.name(py).into_owned();

        if type_name == "int" {
            let i32_ = i32::extract(py, arg).expect("Conversion failed");
            result.push(RuntimeValue::from(i32_));
        }        
        if type_name == "float" {
            let f32_ = f32::extract(py, arg).expect("Conversion failed");
            result.push(RuntimeValue::from(F32::from_float(f32_)));
        }
    }

    result
}

fn invoke_export(py: Python, instance_id: String, method: String, args: PyTuple) -> PyResult<PyObject> {
    unsafe {
        if let Some(ref i) = INSTANCES {
            let instance = i.get(&instance_id).expect("Unknonw instance id");
            let args = create_args(py, args);
            let result = instance.invoke_export(
                &method,
                &args,
                &mut NopExternals).expect("Failed to execute export");
            
            if let Some(result) = result {
                match result {
                    RuntimeValue::I32(i) => return Ok(i.to_py_object(py).into_object()),
                    RuntimeValue::I64(l) => return Ok(l.to_py_object(py).into_object()),
                    RuntimeValue::F32(f) => return Ok(f.to_float().to_py_object(py).into_object()),
                    RuntimeValue::F64(d) => return Ok(d.to_float().to_py_object(py).into_object())
                }            
            }
        }
    }
    Ok("hello".to_py_object(py).into_object())
}


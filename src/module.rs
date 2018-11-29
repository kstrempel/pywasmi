use std::io::prelude::*;
use std::fs::File;

use wasmi::{Module, ModuleInstance, ImportsBuilder};
use pyo3::types::PyType;
use pyo3::prelude::*;

use super::instance::WASMInstance;


#[pyclass]
pub struct WASMModule {
    module: Module
}

#[pymethods]
impl WASMModule {
    #[new]
    fn __new__(obj: &PyRawObject, buffer: Vec<u8>) -> PyResult<()> {
        let module = Module::from_buffer(&buffer).expect("Failed to load wasm");
        obj.init(|_| WASMModule{module}) 
    }


    #[classmethod]
    fn from_file(_cls: &PyType, path: &str) -> PyResult<WASMModule> {
        let mut file = File::open(path).expect("Can't open file");
        let mut content : Vec<u8> = Vec::new();
        file.read_to_end(&mut content).expect("Can't read file");

        WASMModule::from_buffer(_cls, content)
    }

    #[classmethod]
    fn from_buffer(_cls: &PyType, buffer: Vec<u8>) -> PyResult<WASMModule> {
        let module = Module::from_buffer(&buffer).expect("Failed to load wasm");
        Ok(WASMModule{module})
    }

    fn create_instance(&self) -> PyResult<WASMInstance> {
        let instance = ModuleInstance::new(
            &self.module, 
            &ImportsBuilder::default()
        )
        .expect("Failing to create instance")
        .assert_no_start();


        Ok(WASMInstance{instance})
    }
}


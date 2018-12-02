#![feature(specialization)]

extern crate pyo3;
extern crate wasmi;

mod instance;
mod module;

use pyo3::prelude::*;

use self::instance::WASMInstance;
use self::module::WASMModule;

#[pymodinit]
fn pywasmi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WASMModule>()?;
    m.add_class::<WASMInstance>()?;

    Ok(())
}

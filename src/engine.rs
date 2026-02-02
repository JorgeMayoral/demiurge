use std::{ffi::CString, fs, path::PathBuf};

use pyo3::{ffi::c_str, prelude::*};

use crate::config::Config;

pub fn run(python_file: PathBuf) -> PyResult<Config> {
    Python::attach(|py| {
        let code = c_str!(include_str!("./py-lib/demiurge.py"));
        let module = PyModule::from_code(py, &code, c_str!("demiurge.py"), c_str!("demiurge"))?;

        let sys = py.import("sys")?;
        let modules = sys.getattr("modules")?;
        modules.set_item("demiurge", module)?;

        let user_config_code = fs::read_to_string(python_file).unwrap();
        let user_code_cstring = CString::new(user_config_code.as_str()).unwrap();
        let user_code_cstr = user_code_cstring.as_c_str();

        py.run(user_code_cstr, None, None)?;
        py.run(c_str!("from demiurge import STATE"), None, None)?;
        let result = py.eval(c_str!("STATE"), None, None)?;
        let state: Config = result.extract()?;

        Ok(state)
    })
}

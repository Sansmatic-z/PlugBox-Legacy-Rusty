// Copyright 2026 Raj Mitra
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

/// WireData is the rich structured type passed across the wire
/// between Rust and Python. Supports any Python type (dict, list, etc).
#[pyclass]
pub struct WireData {
    #[pyo3(get, set)]
    pub key: String,
    #[pyo3(get, set)]
    pub data: PyObject,
    #[pyo3(get, set)]
    pub metadata: PyObject,
}

#[pymethods]
impl WireData {
    #[new]
    fn new(key: String, data: PyObject, metadata: PyObject) -> Self {
        WireData { key, data, metadata }
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        let data_repr: String = self.data.bind(py).repr()?.to_string();
        let meta_repr: String = self.metadata.bind(py).repr()?.to_string();
        Ok(format!("WireData(key='{}', data={}, metadata={})",
            self.key, data_repr, meta_repr))
    }
}

impl WireData {
    fn clone_ref(&self, py: Python<'_>) -> Self {
        WireData {
            key: self.key.clone(),
            data: self.data.clone_ref(py),
            metadata: self.metadata.clone_ref(py),
        }
    }
}

/// RustBox is the sealed execution core.
/// Users plug Python functions into named wire points,
/// seal the box, then run it.
#[pyclass]
pub struct RustBox {
    wires: HashMap<String, PyObject>,
    sealed: bool,
    name: String,
}

#[pymethods]
impl RustBox {

    /// Create a new named RustBox instance
    #[staticmethod]
    fn load(name: String) -> Self {
        println!("[PlugBox] '{}' initializing...", name);
        RustBox {
            wires: HashMap::new(),
            sealed: false,
            name,
        }
    }

    /// Attach a Python function to a named wire point.
    /// Must be called before seal().
    fn wire(&mut self, py: Python<'_>, wire_name: String, func: PyObject) -> PyResult<()> {
        if self.sealed {
            return Err(pyo3::exceptions::PyRuntimeError::new_err(
                format!(
                    "FAILED TO CONNECT WIRE: Box '{}' is already sealed.\n\n\
                    HINT: You must attach all wires BEFORE calling seal().\n      \
                    Move your box.wire('{}', ...) call above box.seal().",
                    self.name, wire_name
                )
            ));
        }
        
        if !func.bind(py).is_callable() {
             return Err(pyo3::exceptions::PyTypeError::new_err(
                format!(
                    "INVALID WIRE: The object provided for wire '{}' is not callable.\n\n\
                    HINT: Ensure you are passing a function or a method, not the result of a function call.\n      \
                    CORRECT: box.wire('{}', my_func)\n      \
                    WRONG:   box.wire('{}', my_func())",
                    wire_name, wire_name, wire_name
                )
            ));
        }

        println!("[PlugBox] Wire '{}' connected.", wire_name);
        self.wires.insert(wire_name, func);
        Ok(())
    }

    /// Seal the box. No more wires can be attached after this.
    fn seal(&mut self) -> PyResult<()> {
        if self.sealed {
            println!("[PlugBox] Warning: Box '{}' is already sealed.", self.name);
            return Ok(());
        }

        if self.wires.is_empty() {
            return Err(pyo3::exceptions::PyRuntimeError::new_err(
                format!(
                    "CANNOT SEAL BOX: Box '{}' has no wires connected.\n\n\
                    HINT: A PlugBox needs at least one wire to do anything useful.\n      \
                    Add a wire using box.wire('name', your_function) before sealing.",
                    self.name
                )
            ));
        }

        println!("[PlugBox] Box '{}' sealed with {} wire(s): {:?}",
            self.name,
            self.wires.len(),
            self.wires.keys().collect::<Vec<_>>()
        );
        self.sealed = true;
        Ok(())
    }

    /// List all wires currently attached to this box.
    fn list_wires(&self) -> Vec<String> {
        self.wires.keys().cloned().collect()
    }

    /// Check if a specific wire is attached.
    fn has_wire(&self, wire_name: String) -> bool {
        self.wires.contains_key(&wire_name)
    }

    /// Run the box. Executes Rust-side computation,
    /// calls all wired Python functions with structured data,
    /// and returns all results as a dict.
    fn run(&self, py: Python<'_>) -> PyResult<PyObject> {
        if !self.sealed {
            return Err(pyo3::exceptions::PyRuntimeError::new_err(
                format!(
                    "CANNOT RUN BOX: Box '{}' is not sealed.\n\n\
                    HINT: You must call box.seal() after attaching all wires and before calling run().\n      \
                    Example: box.wire(...); box.seal(); box.run()",
                    self.name
                )
            ));
        }

        println!("\n[PlugBox] '{}' execution started.", self.name);

        // === RUST HEAVY WORK HAPPENS HERE ===
        // Simulate real Rust computation
        let mut computed: u64 = 0;
        for i in 0u64..1_000_000 {
            computed = computed.wrapping_add(i.wrapping_mul(i));
        }
        println!("[PlugBox] Rust core computed: {}", computed);

        // Build structured data to send across the wire
        // Now using RICH data (a dict)
        let data_dict = PyDict::new_bound(py);
        data_dict.set_item("raw_result", computed)?;
        data_dict.set_item("is_even", computed % 2 == 0)?;
        data_dict.set_item("cycle_count", 1_000_000)?;

        let meta_dict = PyDict::new_bound(py);
        meta_dict.set_item("box_name", &self.name)?;
        meta_dict.set_item("engine_version", "0.1.0")?;

        let payload = WireData::new(
            "core_result".to_string(),
            data_dict.into_py(py),
            meta_dict.into_py(py),
        );

        // === CALL ALL WIRES AND COLLECT RESULTS ===
        let results = PyDict::new_bound(py);

        for (wire_name, func) in &self.wires {
            println!("[PlugBox] Calling wire '{}'...", wire_name);
            let wire_result = func.call1(py, (payload.clone_ref(py),));
            match wire_result {
                Ok(val) => {
                    println!("[PlugBox] Wire '{}' returned successfully.", wire_name);
                    results.set_item(wire_name, val)?;
                }
                Err(e) => {
                    println!("[PlugBox] Wire '{}' raised an error: {}", wire_name, e);
                    results.set_item(wire_name, format!("ERROR: {}", e))?;
                }
            }
        }

        println!("[PlugBox] '{}' execution complete.\n", self.name);
        Ok(results.into())
    }

    /// Call a single specific wire by name with custom data.
    fn call_wire(
        &self,
        py: Python<'_>,
        wire_name: String,
        data: Py<WireData>,
    ) -> PyResult<PyObject> {
        if !self.sealed {
            return Err(pyo3::exceptions::PyRuntimeError::new_err(
                format!(
                    "CANNOT CALL WIRE: Box '{}' must be sealed before direct wire calls.\n\n\
                    HINT: Call box.seal() first.",
                    self.name
                )
            ));
        }
        match self.wires.get(&wire_name) {
            Some(func) => {
                println!("[PlugBox] Direct call to wire '{}'", wire_name);
                func.call1(py, (data,))
            }
            None => Err(pyo3::exceptions::PyKeyError::new_err(
                format!(
                    "WIRE NOT FOUND: No wire named '{}' is connected to box '{}'.\n\n\
                    HINT: Check your spelling or ensure box.wire('{}', ...) was called.\n      \
                    Available wires: {:?}",
                    wire_name, self.name, wire_name, self.list_wires()
                )
            )),
        }
    }
}

#[pyclass]
pub struct CsvBox {
    wires: HashMap<String, PyObject>,
    sealed: bool,
}

#[pymethods]
impl CsvBox {
    #[new]
    fn new() -> Self { CsvBox { wires: HashMap::new(), sealed: false } }
    
    fn wire(&mut self, py: Python<'_>, wire_name: String, func: PyObject) -> PyResult<()> {
        if self.sealed { return Err(pyo3::exceptions::PyRuntimeError::new_err("Sealed")); }
        if !func.bind(py).is_callable() { return Err(pyo3::exceptions::PyTypeError::new_err("Not callable")); }
        self.wires.insert(wire_name, func);
        Ok(())
    }

    fn seal(&mut self) -> PyResult<()> { self.sealed = true; Ok(()) }

    fn process(&self, py: Python<'_>, path: String) -> PyResult<u32> {
        if !self.sealed { return Err(pyo3::exceptions::PyRuntimeError::new_err("Not sealed")); }
        let mut rdr = csv::Reader::from_path(path).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        let mut passed = 0;
        let filter_wire = self.wires.get("filter");

        for result in rdr.records() {
            let record = result.map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
            if let Some(func) = filter_wire {
                // Pass row as a list of strings to python
                let record_vec: Vec<&str> = record.iter().collect();
                let py_row = pyo3::types::PyList::new_bound(py, record_vec);
                let keep: bool = func.call1(py, (py_row,))?.extract(py)?;
                if keep { passed += 1; }
            } else {
                passed += 1;
            }
        }
        Ok(passed)
    }
}

#[pyclass]
pub struct GameBox { wires: HashMap<String, PyObject>, sealed: bool }

#[pymethods]
impl GameBox {
    #[new]
    fn new() -> Self { GameBox { wires: HashMap::new(), sealed: false } }
    fn wire(&mut self, py: Python<'_>, wire_name: String, func: PyObject) -> PyResult<()> {
        if !func.bind(py).is_callable() { return Err(pyo3::exceptions::PyTypeError::new_err("Not callable")); }
        self.wires.insert(wire_name, func); Ok(())
    }
    fn seal(&mut self) -> PyResult<()> { self.sealed = true; Ok(()) }
    fn run_loop(&self, py: Python<'_>, ticks: u32) -> PyResult<i32> {
        let mut state = 0;
        let update_wire = self.wires.get("update");
        for _ in 0..ticks {
            state += 1; // engine physics
            if let Some(func) = update_wire {
                state = func.call1(py, (state,))?.extract(py)?;
            }
        }
        Ok(state)
    }
}

#[pyclass]
pub struct NetworkBox { wires: HashMap<String, PyObject>, sealed: bool }

#[pymethods]
impl NetworkBox {
    #[new]
    fn new() -> Self { NetworkBox { wires: HashMap::new(), sealed: false } }
    fn wire(&mut self, py: Python<'_>, wire_name: String, func: PyObject) -> PyResult<()> {
        if !func.bind(py).is_callable() { return Err(pyo3::exceptions::PyTypeError::new_err("Not callable")); }
        self.wires.insert(wire_name, func); Ok(())
    }
    fn seal(&mut self) -> PyResult<()> { self.sealed = true; Ok(()) }
    fn process_requests(&self, py: Python<'_>, requests: Vec<String>) -> PyResult<Vec<bool>> {
        let mut responses = Vec::with_capacity(requests.len());
        let filter_wire = self.wires.get("filter");
        for req in requests {
            let mut allow = true;
            if let Some(func) = filter_wire {
                allow = func.call1(py, (req,))?.extract(py)?;
            }
            responses.push(allow);
        }
        Ok(responses)
    }
}

/// Register the Python module
#[pymodule]
fn plugbox(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RustBox>()?;
    m.add_class::<WireData>()?;
    m.add_class::<CsvBox>()?;
    m.add_class::<GameBox>()?;
    m.add_class::<NetworkBox>()?;
    Ok(())
}



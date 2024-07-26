mod energy;
mod power;
pub(crate) mod util;

use pyo3::{prelude::*, py_run};

pub(crate) use energy::Energy;
pub(crate) use power::Power;

pub(crate) fn register_submodule(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let submodule = PyModule::new_bound(parent_module.py(), "frequenz_sdk.quantity")?;
    py_run!(
        parent_module.py(),
        submodule,
        "import sys; sys.modules['frequenz_sdk.quantity'] = submodule"
    );
    submodule.add_class::<Energy>()?;
    submodule.add_class::<Power>()?;
    parent_module.add_submodule(&submodule)
}

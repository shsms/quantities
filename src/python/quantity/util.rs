use pyo3::{prelude::*, types::PyDelta};

pub(crate) fn duration_to_timedelta<'py>(
    py: Python<'py>,
    dur: std::time::Duration,
) -> PyResult<Bound<'py, PyDelta>> {
    PyDelta::new_bound(
        py,
        (dur.as_secs() / (60 * 60 * 24)) as i32,
        (dur.as_secs() % (60 * 60 * 24)) as i32,
        (dur.as_micros() % 1_000_000) as i32,
        false,
    )
}

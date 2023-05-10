// Copyright 2023 Datafuse Labs.
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

mod asyncio;

use databend_client::APIClient;
use databend_driver::rest_api::RestAPIConnection;
use futures::future::ok;
use pyo3::create_exception;
use crate::asyncio::*;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
create_exception!(databend_client, Error, PyException, "databend_client related errors");
fn build_rest_api_client(dsn: &str) -> PyResult<RestAPIConnection> {
    let conn = RestAPIConnection::try_create(dsn).unwrap();
    Ok(conn)
}

fn format_pyerr(err: &str) -> PyErr {
    match !err.is_empty() {
        _ => Error::new_err(err.to_string()),
    }
}

#[pymodule]
fn _databend_python(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AsyncDatabendDriver>()?;
    Ok(())
}

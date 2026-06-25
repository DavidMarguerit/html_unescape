#![allow(clippy::unused_unit)]
use html_escape::decode_html_entities;
use polars::prelude::*;
use pyo3::prelude::*;
use pyo3_polars::derive::polars_expr;

/// Mirrors Python's html.unescape(): converts HTML entities back to characters.
/// Handles named entities (&amp;, &lt;, &nbsp;, …) and numeric ones (&#123;, &#x7B;).
#[polars_expr(output_type = String)]
fn html_unescape(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: StringChunked = ca.apply(|opt_s| {
        opt_s.map(|s| decode_html_entities(s)) // returns Cow<str>, zero-copy when no entity found
    });
    Ok(out.into_series())
}

#[pymodule]
fn html_unescape(_py: Python, _m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
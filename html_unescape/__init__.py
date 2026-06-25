from pathlib import Path
import polars as pl
from polars.plugins import register_plugin_function

_LIB = Path(__file__).parent.parent  # root of the compiled .so

def html_unescape(expr: pl.Expr) -> pl.Expr:
    return register_plugin_function(
        plugin_path=_LIB,
        args=[expr],
        function_name="html_unescape",
        is_elementwise=True,   # enables parallelism across chunks
    )
# html_unescape

A Polars plugin that unescapes HTML entities in string columns, powered by Rust for maximum performance.

Handles named entities (`&amp;`, `&lt;`, `&nbsp;`, …), decimal (`&#123;`), and hex (`&#x7B;`) references.

## Installation

From a GitHub Release (no Rust toolchain needed):

```bash
pip install html_unescape@https://github.com/DavidMarguerit/html_unescape/releases/download/v0.1.0/html_unescape-0.1.0-cp312-cp312-win_amd64.whl
```

From source (requires Rust):

```bash
pip install git+https://github.com/DavidMarguerit/html_unescape.git
```

## Usage

```python
import polars as pl
from html_unescape import html_unescape

df = pl.DataFrame({"raw": ["&lt;div&gt;Hello &amp; world&lt;/div&gt;"]})

df.with_columns(clean=html_unescape(pl.col("raw")))
# ┌──────────────────────────────────────┬──────────────────────┐
# │ raw                                  ┆ clean                │
# │ ---                                  ┆ ---                  │
# │ str                                  ┆ str                  │
# ╞══════════════════════════════════════╪══════════════════════╡
# │ &lt;div&gt;Hello &amp; world&lt;/… ┆ <div>Hello & world</… │
# └──────────────────────────────────────┴──────────────────────┘
```

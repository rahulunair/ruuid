## ruuid

A fast UUID generator for Python built using Rust. Its a simple wrapper on top
of Rust's UUID library.


## How to use?

### Installation:


```bash
pip3 install ruuid==0.2.2 --force --no-cache
```

### Usage:

```bash
import ruuid

ruuid.uuid4()

'7a1ef475-904c-4d53-8985-528d09d57414'
```

## Performance

Tests ran on an M1 macbook air with 8 GB of RAM and Python 3.9

```bash
from ruuid import uuid4, simple

%timeit simple()
327 ns ± 0.442 ns per loop (mean ± std. dev. of 7 runs, 1000000 loops each)

%timeit uuid4()
348 ns ± 0.707 ns per loop (mean ± std. dev. of 7 runs, 1000000 loops each)

from uuid import uuid4

%timeit uuid4()
1.1 µs ± 2.91 ns per loop (mean ± std. dev. of 7 runs, 1000000 loops each)
```

## Available functions

- `uuid4()` - random UUID4 string
- `simple()` -  formatted simple string of UUID4
- `hyphenated()` - hyphenated UUID4 string
- `nil()` - a uuid with all zeros
- `urn()` - uuid4 as a URN string

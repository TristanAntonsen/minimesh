# minimesh

A very simple library to provide some basic mesh utilities (and practice some Rust).

## Building the library:

Create and activate a virtual environment

```zsh
python -mm venv venv
source venv/bin/activate
```

Install maturin

```zsh
minimesh/minimesh (venv) pip install maturin
```

from the minimesh/minimesh directory:
```zsh
minimesh/minimesh (venv) maturin develop --release
```

The library can then be imported into python using the virtual environment.

---
Usage:

```python
import minimesh as mm

STL_PATH = "../stl/cube.stl"

vertices = mm.vertices(STL_PATH) # vertex coordinates
triangles = mm.triangles(STL_PATH) # triangle vertex indices
area = mm.area(STL_PATH) # mesh surface area
volume = mm.volume(STL_PATH) # mesh volume
dimensions = mm.dimensions(STL_PATH) # mesh XYZ dimensions
```
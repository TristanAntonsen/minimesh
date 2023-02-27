# minimesh

A very simple library to provide some basic mesh utilities (and practice some Rust).

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
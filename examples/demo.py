import minimesh as mm


STL_PATH = "stl/cube.stl"

print(f"STL Volume: {round(mm.volume(STL_PATH),2)} mm^3")
print(f"STL Surface area: {round(mm.area(STL_PATH),2)} mm^2")
print(f"STL Vertex count: {len(mm.vertices(STL_PATH))}")
print(f"STL Tri count: {len(mm.triangles(STL_PATH))}")
print(f"STL Dimensions: {[round(x, 2) for x in mm.dimensions(STL_PATH)]}")
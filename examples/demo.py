import minimesh as mm


FILE_PATH = "stl/cube.stl"

print(f"STL Volume: {round(mm.mesh_volume(FILE_PATH),2)} mm^3")
print(f"STL Surface area: {round(mm.mesh_area(FILE_PATH),2)} mm^2")
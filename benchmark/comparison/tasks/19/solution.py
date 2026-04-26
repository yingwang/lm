import math

def shape_area(shape, *args):
    if shape == "Circle":
        r = args[0]
        return 3.14159 * r * r
    elif shape == "Rect":
        w, h = args[0], args[1]
        return float(w * h)
    elif shape == "Triangle":
        b, h = args[0], args[1]
        return float(b * h / 2)

print(shape_area("Circle", 5))
print(shape_area("Rect", 3, 4))
print(shape_area("Triangle", 6, 4))

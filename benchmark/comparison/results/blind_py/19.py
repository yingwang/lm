import math

class Circle:
    def __init__(self, r):
        self.r = r

class Rect:
    def __init__(self, w, h):
        self.w = w
        self.h = h

class Triangle:
    def __init__(self, b, h):
        self.b = b
        self.h = h

def area(s):
    pi = 3.14159
    if isinstance(s, Circle):
        return s.r * s.r * pi
    elif isinstance(s, Rect):
        return s.w * s.h
    elif isinstance(s, Triangle):
        return 0.5 * s.b * s.h

print(area(Circle(5.0)))
print(area(Rect(3.0, 4.0)))
print(area(Triangle(6.0, 4.0)))

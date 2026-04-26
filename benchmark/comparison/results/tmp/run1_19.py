class Shape:
    """Base class for all shapes."""
    pass

class Circle(Shape):
    """Represents a circle with a given radius."""
    def __init__(self, radius: float):
        if not isinstance(radius, (int, float)) or radius < 0:
            raise ValueError("Radius must be a non-negative number.")
        self.radius = float(radius)

class Rect(Shape):
    """Represents a rectangle with given width and height."""
    def __init__(self, width: float, height: float):
        if not isinstance(width, (int, float)) or width < 0:
            raise ValueError("Width must be a non-negative number.")
        if not isinstance(height, (int, float)) or height < 0:
            raise ValueError("Height must be a non-negative number.")
        self.width = float(width)
        self.height = float(height)

class Triangle(Shape):
    """Represents a triangle with given base and height."""
    def __init__(self, base: float, height: float):
        if not isinstance(base, (int, float)) or base < 0:
            raise ValueError("Base must be a non-negative number.")
        if not isinstance(height, (int, float)) or height < 0:
            raise ValueError("Height must be a non-negative number.")
        self.base = float(base)
        self.height = float(height)

# Define pi as specified
PI = 3.14159

def area(s: Shape) -> float:
    """
    Computes the area of a given shape.

    Args:
        s: An instance of Circle, Rect, or Triangle.

    Returns:
        The area of the shape as a float.

    Raises:
        TypeError: If the input is not a recognized Shape type.
    """
    if isinstance(s, Circle):
        return PI * s.radius * s.radius
    elif isinstance(s, Rect):
        return s.width * s.height
    elif isinstance(s, Triangle):
        return 0.5 * s.base * s.height
    else:
        raise TypeError(f"Unsupported shape type: {type(s)}")

print(area(Circle(5.0)))
print(area(Rect(3.0, 4.0)))
print(area(Triangle(6.0, 4.0)))
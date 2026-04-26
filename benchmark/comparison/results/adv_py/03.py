class Color:
    Red = "Red"
    Green = "Green"
    Blue = "Blue"
    Yellow = "Yellow"

def to_hex(c):
    mapping = {
        Color.Red: "#FF0000",
        Color.Green: "#00FF00",
        Color.Blue: "#0000FF",
        Color.Yellow: "#FFFF00",
    }
    return mapping[c]

def is_primary(c):
    return c in (Color.Red, Color.Green, Color.Blue)

for c in [Color.Red, Color.Green, Color.Blue, Color.Yellow]:
    print(to_hex(c))

for c in [Color.Red, Color.Green, Color.Blue, Color.Yellow]:
    print("true" if is_primary(c) else "false")

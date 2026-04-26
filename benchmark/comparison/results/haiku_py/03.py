class Color:
    def __init__(self, name, hex_code):
        self.name = name
        self.hex_code = hex_code

    def to_hex(self):
        return self.hex_code

    def is_primary(self):
        return self.name in ["Red", "Green", "Blue"]

# Create colors
red = Color("Red", "#FF0000")
green = Color("Green", "#00FF00")
blue = Color("Blue", "#0000FF")
yellow = Color("Yellow", "#FFFF00")

colors = [red, green, blue, yellow]

# Print hex for all 4
for color in colors:
    print(color.to_hex())

# Print is_primary for all 4
for color in colors:
    print("true" if color.is_primary() else "false")

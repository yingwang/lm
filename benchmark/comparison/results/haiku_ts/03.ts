type Color = "Red" | "Green" | "Blue" | "Yellow";

function toHex(color: Color): string {
  switch (color) {
    case "Red": return "#FF0000";
    case "Green": return "#00FF00";
    case "Blue": return "#0000FF";
    case "Yellow": return "#FFFF00";
  }
}

function isPrimary(color: Color): boolean {
  return color === "Red" || color === "Green" || color === "Blue";
}

// Test cases
const colors: Color[] = ["Red", "Green", "Blue", "Yellow"];

for (const color of colors) {
  console.log(toHex(color));
}

for (const color of colors) {
  console.log(isPrimary(color) ? "true" : "false");
}

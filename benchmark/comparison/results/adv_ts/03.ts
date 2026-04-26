type Color = "Red" | "Green" | "Blue" | "Yellow";

function to_hex(c: Color): string {
  switch (c) {
    case "Red": return "#FF0000";
    case "Green": return "#00FF00";
    case "Blue": return "#0000FF";
    case "Yellow": return "#FFFF00";
  }
}

function is_primary(c: Color): boolean {
  return c === "Red" || c === "Green" || c === "Blue";
}

const colors: Color[] = ["Red", "Green", "Blue", "Yellow"];
for (const c of colors) {
  console.log(to_hex(c));
}
for (const c of colors) {
  console.log(String(is_primary(c)));
}

type Shape =
  | { type: "Circle"; r: number }
  | { type: "Rect"; w: number; h: number }
  | { type: "Triangle"; b: number; h: number };

function Circle(r: number): Shape { return { type: "Circle", r }; }
function Rect(w: number, h: number): Shape { return { type: "Rect", w, h }; }
function Triangle(b: number, h: number): Shape { return { type: "Triangle", b, h }; }

function area(s: Shape): number {
  switch (s.type) {
    case "Circle": return 3.14159 * s.r * s.r;
    case "Rect": return s.w * s.h;
    case "Triangle": return 0.5 * s.b * s.h;
  }
}

function formatFloat(n: number): string {
  if (Number.isInteger(n)) return n.toFixed(1);
  return String(n);
}

console.log(formatFloat(area(Circle(5.0))));
console.log(formatFloat(area(Rect(3.0, 4.0))));
console.log(formatFloat(area(Triangle(6.0, 4.0))));

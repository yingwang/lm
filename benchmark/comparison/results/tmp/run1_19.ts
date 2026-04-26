const PI = 3.14159;

interface Circle {
    kind: "circle";
    radius: number;
}

interface Rect {
    kind: "rect";
    width: number;
    height: number;
}

interface Triangle {
    kind: "triangle";
    base: number;
    height: number;
}

type Shape = Circle | Rect | Triangle;

function circle(radius: number): Circle {
    return { kind: "circle", radius };
}

function rect(width: number, height: number): Rect {
    return { kind: "rect", width, height };
}

function triangle(base: number, height: number): Triangle {
    return { kind: "triangle", base, height };
}

function area(s: Shape): number {
    switch (s.kind) {
        case "circle":
            return PI * s.radius * s.radius;
        case "rect":
            return s.width * s.height;
        case "triangle":
            return 0.5 * s.base * s.height;
        default:
            // This exhaustive check ensures all Shape variants are handled
            const _exhaustiveCheck: never = s;
            throw new Error(`Unknown shape kind: ${(_exhaustiveCheck as any).kind}`);
    }
}

console.log(area(circle(5.0)));
console.log(area(rect(3.0, 4.0)));
console.log(area(triangle(6.0, 4.0)));
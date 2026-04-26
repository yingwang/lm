type Result<T> = { type: "Ok"; value: T } | { type: "Err"; message: string };

function parseAge(s: string): Result<number> {
  const n = parseInt(s, 10);
  if (isNaN(n)) {
    return { type: "Err", message: "not a number" };
  }
  return { type: "Ok", value: n };
}

function validateAge(n: number): Result<number> {
  if (n >= 0 && n < 150) {
    return { type: "Ok", value: n };
  }
  return { type: "Err", message: "age out of range" };
}

function categorizeAge(n: number): string {
  if (n < 13) return "child";
  if (n < 20) return "teen";
  return "adult";
}

function processAge(s: string): Result<string> {
  const parseResult = parseAge(s);
  if (parseResult.type === "Err") {
    return parseResult;
  }

  const age = parseResult.value;
  const validateResult = validateAge(age);
  if (validateResult.type === "Err") {
    return validateResult;
  }

  const category = categorizeAge(age);
  return { type: "Ok", value: category };
}

function resultToString(result: Result<string>): string {
  if (result.type === "Ok") {
    return `Ok(${result.value})`;
  }
  return `Err(${result.message})`;
}

// Test cases
const testCases = ["10", "15", "25", "abc", "-5", "200"];
for (const testCase of testCases) {
  console.log(resultToString(processAge(testCase)));
}

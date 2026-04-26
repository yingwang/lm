type Result = { tag: "Ok"; value: string } | { tag: "Err"; msg: string };

function Ok(value: string): Result { return { tag: "Ok", value }; }
function Err(msg: string): Result { return { tag: "Err", msg }; }

function formatResult(r: Result): string {
  if (r.tag === "Ok") return `Ok(${r.value})`;
  return `Err(${r.msg})`;
}

function validate_age(age: number): Result {
  if (age < 0) return Err("age must be non-negative");
  if (age >= 150) return Err("age must be under 150");
  return Ok(String(age));
}

function validate_name(name: string): Result {
  if (name === "") return Err("name must not be empty");
  return Ok(name);
}

function validate_user(name: string, age: number): Result {
  const nameResult = validate_name(name);
  if (nameResult.tag === "Err") return nameResult;
  const ageResult = validate_age(age);
  if (ageResult.tag === "Err") return ageResult;
  return Ok(`${name} (age ${age})`);
}

console.log(formatResult(validate_age(25)));
console.log(formatResult(validate_age(-1)));
console.log(formatResult(validate_age(200)));
console.log(formatResult(validate_name("Alice")));
console.log(formatResult(validate_name("")));
console.log(formatResult(validate_user("Alice", 25)));
console.log(formatResult(validate_user("", 25)));
console.log(formatResult(validate_user("Alice", -1)));

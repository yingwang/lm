type Result<T> = { tag: "Ok"; value: T } | { tag: "Err"; message: string };

function validate_age(age: number): Result<number> {
  if (age < 0) return { tag: "Err", message: "age must be non-negative" };
  if (age >= 150) return { tag: "Err", message: "age must be under 150" };
  return { tag: "Ok", value: age };
}

function validate_name(name: string): Result<string> {
  if (name === "") return { tag: "Err", message: "name must not be empty" };
  return { tag: "Ok", value: name };
}

function validate_user(name: string, age: number): Result<string> {
  const nameResult = validate_name(name);
  if (nameResult.tag === "Err") return nameResult;
  const ageResult = validate_age(age);
  if (ageResult.tag === "Err") return { tag: "Err", message: ageResult.message };
  return { tag: "Ok", value: `${nameResult.value} (age ${ageResult.value})` };
}

function formatResultNum(r: Result<number>): string {
  if (r.tag === "Ok") return `Ok(${r.value})`;
  return `Err(${r.message})`;
}

function formatResultStr(r: Result<string>): string {
  if (r.tag === "Ok") return `Ok(${r.value})`;
  return `Err(${r.message})`;
}

console.log(formatResultNum(validate_age(25)));
console.log(formatResultNum(validate_age(-1)));
console.log(formatResultNum(validate_age(150)));

console.log(formatResultStr(validate_name("Alice")));
console.log(formatResultStr(validate_name("")));

console.log(formatResultStr(validate_user("Alice", 25)));
console.log(formatResultStr(validate_user("", 25)));
console.log(formatResultStr(validate_user("Bob", -1)));

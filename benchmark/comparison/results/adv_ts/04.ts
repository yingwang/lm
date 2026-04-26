function parse_age(s: string): string {
  const n = Number(s);
  if (!Number.isInteger(n) || s.trim() === "") return "Err(not a number)";
  return `Ok(${n})`;
}

function validate_age(n: number): string {
  if (n >= 0 && n < 150) return `Ok(${n})`;
  return "Err(age out of range)";
}

function categorize_age(n: number): string {
  if (n < 13) return "Ok(child)";
  if (n < 20) return "Ok(teen)";
  return "Ok(adult)";
}

function process_age(s: string): string {
  const parsed = parse_age(s);
  if (parsed.startsWith("Err")) return parsed;
  const n = Number(parsed.slice(3, -1));
  const validated = validate_age(n);
  if (validated.startsWith("Err")) return validated;
  return categorize_age(n);
}

for (const s of ["10", "15", "25", "abc", "-5", "200"]) {
  console.log(process_age(s));
}

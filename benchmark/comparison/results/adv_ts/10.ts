function validate_username(s: string): string {
  if (s === "") return "Err(username must not be empty)";
  if (s.length > 20) return "Err(username must be at most 20 characters)";
  return `Ok(${s})`;
}

function validate_email(s: string): string {
  if (!s.includes("@")) return "Err(email must contain @)";
  return `Ok(${s})`;
}

function validate_password(s: string): string {
  if (s.length < 8) return "Err(password must be at least 8 characters)";
  return `Ok(${s})`;
}

function register(u: string, e: string, p: string): string {
  const uResult = validate_username(u);
  if (uResult.startsWith("Err")) return uResult;
  const eResult = validate_email(e);
  if (eResult.startsWith("Err")) return eResult;
  const pResult = validate_password(p);
  if (pResult.startsWith("Err")) return pResult;
  return `Ok(${u} registered with ${e})`;
}

console.log(register("alice", "alice@example.com", "password123"));
console.log(register("", "a@b", "pass1234"));
console.log(register("alice", "noatsign", "pass1234"));
console.log(register("alice", "a@b", "short"));
console.log(register("aaaaabbbbbcccccddddde", "a@b", "pass1234"));

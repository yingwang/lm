#!/usr/bin/env node
/**
 * Blind cross-language benchmark: LM vs TypeScript vs Python
 *
 * Uses Gemini API to generate complete programs given only task descriptions.
 * No expected output is shown to the model — pure first-pass correctness.
 *
 * Usage: node blind_benchmark.mjs [--runs N] [--model MODEL]
 */

import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { execSync } from 'node:child_process';

const { readFileSync, writeFileSync, mkdirSync } = fs;
const { join, dirname } = path;

const __dirname = dirname(fileURLToPath(import.meta.url));
const TASKS = JSON.parse(readFileSync(join(__dirname, 'tasks.json'), 'utf8'));
const LM_REF = readFileSync(join(__dirname, '../../docs/llm-reference.md'), 'utf8');
const RESULTS_DIR = join(__dirname, 'results');
const TMP_DIR = join(RESULTS_DIR, 'tmp');
mkdirSync(RESULTS_DIR, { recursive: true });
mkdirSync(TMP_DIR, { recursive: true });

const API_KEY = process.env.GEMINI_API_KEY;
if (!API_KEY) {
  console.error('GEMINI_API_KEY env var is required');
  process.exit(1);
}
const args = process.argv.slice(2);
const NUM_RUNS = parseInt(args.find((_, i, a) => a[i - 1] === '--runs') || '3');
const MODEL = args.find((_, i, a) => a[i - 1] === '--model') || 'gemini-2.5-flash';
const API_URL = `https://generativelanguage.googleapis.com/v1beta/models/${MODEL}:generateContent?key=${API_KEY}`;

// Build lmc
console.log('Building lmc...');
try {
  execSync('cargo build --bin lmc', { cwd: join(__dirname, '../..'), stdio: 'ignore' });
} catch {
  console.error('Failed to build lmc');
  process.exit(1);
}
const LMC = join(__dirname, '../../target/debug/lmc');

// ─── Prompt builders ─────────────────────────────────────────

function buildLmPrompt(task) {
  return `You are writing code in LM, a purely functional programming language.

Here is the complete language reference:

${LM_REF}

## Task

${task.description}

Write a complete LM program that defines the required functions and prints the results using the following test code appended at the end:

\`\`\`
${task.lm_test}
\`\`\`

Write ONLY the function/type definitions needed. The test code above will be appended to your code automatically.
Output a single LM code block. No explanation.`;
}

function buildTsPrompt(task) {
  return `Write a complete, self-contained TypeScript program for the following task.
The program will be run with \`npx tsx file.ts\`. No external packages.

## Task

${task.description}

The program must print the following test results using console.log, by calling the functions you define:

\`\`\`
${task.ts_test}
\`\`\`

The test code above will be appended to your program automatically.
Write ONLY the function/type definitions and any helpers needed. No test calls, no explanation.
Output a single TypeScript code block.`;
}

function buildPyPrompt(task) {
  return `Write a complete, self-contained Python program for the following task.
The program will be run with \`python3 file.py\`. No external packages.

## Task

${task.description}

The program must print the following test results, by calling the functions you define:

\`\`\`
${task.py_test}
\`\`\`

The test code above will be appended to your program automatically.
Write ONLY the function/type definitions and any helpers needed. No test calls, no explanation.
Output a single Python code block.

IMPORTANT: For boolean output, print lowercase "true"/"false", NOT Python's "True"/"False".
For float output, preserve decimal points (e.g., 12.0 not 12).`;
}

// ─── API + Execution ─────────────────────────────────────────

async function callGemini(prompt, retries = 5) {
  const body = {
    contents: [{ parts: [{ text: prompt }] }],
    generationConfig: { temperature: 0.0 }
  };

  for (let attempt = 0; attempt <= retries; attempt++) {
    const resp = await fetch(API_URL, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body)
    });

    if (resp.ok) {
      const data = await resp.json();
      return data.candidates?.[0]?.content?.parts?.[0]?.text || '';
    }

    if (resp.status === 429 && attempt < retries) {
      const wait = Math.min(2000 * Math.pow(2, attempt), 60000);
      process.stdout.write(`[wait ${(wait/1000).toFixed(0)}s]`);
      await new Promise(r => setTimeout(r, wait));
      continue;
    }

    const text = await resp.text();
    throw new Error(`API ${resp.status}: ${text.slice(0, 300)}`);
  }
}

function extractCode(response) {
  const match = response.match(/```(?:\w+)?\n([\s\S]*?)```/);
  return match ? match[1].trim() : response.trim();
}

function runCode(code, lang, taskId, runId) {
  let cmd, file;
  const prefix = `run${runId}_${taskId}`;

  if (lang === 'lm') {
    file = join(TMP_DIR, `${prefix}.lm`);
    writeFileSync(file, code);
    cmd = `"${LMC}" run "${file}"`;
  } else if (lang === 'ts') {
    file = join(TMP_DIR, `${prefix}.ts`);
    writeFileSync(file, code);
    cmd = `npx tsx "${file}"`;
  } else {
    file = join(TMP_DIR, `${prefix}.py`);
    writeFileSync(file, code);
    cmd = `python3 "${file}"`;
  }

  try {
    const output = execSync(cmd, {
      timeout: 15000,
      encoding: 'utf8',
      stdio: ['pipe', 'pipe', 'pipe']
    }).trimEnd();
    return { ok: true, output };
  } catch (e) {
    const stderr = e.stderr?.toString().slice(0, 300) || '';
    return { ok: false, output: stderr || e.message?.slice(0, 200) || 'error' };
  }
}

// ─── Main ────────────────────────────────────────────────────

async function runOnce(runId) {
  const results = {};
  for (const lang of ['lm', 'ts', 'py']) {
    results[lang] = { pass: 0, fail: 0, errors: [] };
  }

  for (const task of TASKS) {
    process.stdout.write(`  [${runId}] ${task.id}_${task.name.padEnd(18)} `);

    for (const lang of ['lm', 'ts', 'py']) {
      let prompt;
      if (lang === 'lm') prompt = buildLmPrompt(task);
      else if (lang === 'ts') prompt = buildTsPrompt(task);
      else prompt = buildPyPrompt(task);

      try {
        const response = await callGemini(prompt);
        let code = extractCode(response);

        // Append test calls
        const testCode = lang === 'lm' ? task.lm_test : lang === 'ts' ? task.ts_test : task.py_test;
        code = code + '\n\n' + testCode;

        const result = runCode(code, lang, task.id, runId);
        const passed = result.ok && result.output === task.expected;

        if (passed) {
          results[lang].pass++;
          process.stdout.write(`${lang}:✓ `);
        } else {
          results[lang].fail++;
          process.stdout.write(`${lang}:✗ `);
          results[lang].errors.push({
            task: `${task.id}_${task.name}`,
            expected: task.expected.split('\n')[0],
            got: result.output?.split('\n')[0] || '(error)',
            compile_error: !result.ok
          });
        }
      } catch (e) {
        results[lang].fail++;
        process.stdout.write(`${lang}:E `);
        results[lang].errors.push({
          task: `${task.id}_${task.name}`,
          expected: '',
          got: `API: ${e.message?.slice(0, 80)}`,
          compile_error: true
        });
      }

      // Rate limit: 4s between calls to stay within free tier
      await new Promise(r => setTimeout(r, 4000));
    }
    console.log();
  }

  return results;
}

async function main() {
  console.log(`\n╔══════════════════════════════════════════╗`);
  console.log(`║  Blind Benchmark: LM vs TypeScript vs Python`);
  console.log(`║  Model: ${MODEL}`);
  console.log(`║  Runs:  ${NUM_RUNS}`);
  console.log(`║  Tasks: ${TASKS.length}`);
  console.log(`╚══════════════════════════════════════════╝\n`);

  const allRuns = [];

  for (let run = 1; run <= NUM_RUNS; run++) {
    console.log(`─── Run ${run}/${NUM_RUNS} ───`);
    const result = await runOnce(run);
    allRuns.push(result);
    console.log(`  → LM: ${result.lm.pass}/${TASKS.length}  TS: ${result.ts.pass}/${TASKS.length}  PY: ${result.py.pass}/${TASKS.length}\n`);
  }

  // Aggregate
  const agg = {};
  for (const lang of ['lm', 'ts', 'py']) {
    agg[lang] = allRuns.map(r => r[lang].pass);
  }
  const avg = arr => (arr.reduce((a, b) => a + b, 0) / arr.length).toFixed(1);

  console.log('╔══════════════════════════════════════════════════╗');
  console.log('║  RESULTS                                         ');
  console.log('╠══════════════════════════════════════════════════╣');
  console.log(`║  ${'Language'.padEnd(12)}  Avg      Min  Max  Runs`);
  console.log(`║  ${'─'.repeat(45)}`);
  for (const [lang, label] of [['lm', 'LM'], ['ts', 'TypeScript'], ['py', 'Python']]) {
    const a = agg[lang];
    console.log(`║  ${label.padEnd(12)}  ${avg(a).padStart(4)}/${TASKS.length}   ${Math.min(...a).toString().padStart(2)}   ${Math.max(...a).toString().padStart(2)}   [${a.join(', ')}]`);
  }
  console.log('╚══════════════════════════════════════════════════╝');

  // Show failures from last run
  const lastRun = allRuns[allRuns.length - 1];
  for (const [lang, label] of [['lm', 'LM'], ['ts', 'TypeScript'], ['py', 'Python']]) {
    const errs = lastRun[lang].errors;
    if (errs.length > 0) {
      console.log(`\n${label} failures (last run):`);
      for (const e of errs) {
        const tag = e.compile_error ? 'ERROR' : 'WRONG';
        console.log(`  ${e.task}: ${tag}`);
        if (!e.compile_error) {
          console.log(`    exp: ${e.expected}`);
          console.log(`    got: ${e.got}`);
        } else {
          console.log(`    ${e.got}`);
        }
      }
    }
  }

  // Save report
  const report = {
    model: MODEL,
    runs: NUM_RUNS,
    tasks: TASKS.length,
    date: new Date().toISOString(),
    aggregate: {},
    runs_detail: allRuns
  };
  for (const lang of ['lm', 'ts', 'py']) {
    report.aggregate[lang] = {
      avg: parseFloat(avg(agg[lang])),
      min: Math.min(...agg[lang]),
      max: Math.max(...agg[lang]),
      scores: agg[lang]
    };
  }

  const reportFile = join(RESULTS_DIR, `blind_${MODEL}_${new Date().toISOString().slice(0, 10)}.json`);
  writeFileSync(reportFile, JSON.stringify(report, null, 2));
  console.log(`\nReport saved: ${reportFile}`);
}

main().catch(e => { console.error(e); process.exit(1); });

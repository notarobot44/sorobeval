#!/usr/bin/env python3
"""SorobEval runner — week-0 skeleton (Python edition).

Scores ONE submission (a lib.rs) against ONE task directory inside the
pinned sandbox. Model adapters and the matrix loop come in week 1; this
file only has to produce a green run on the reference.

Usage (inside the sandbox container, /work = repo root):
    python3 runner/sorobeval.py reference tasks/01-counter
    python3 runner/sorobeval.py score     tasks/01-counter path/to/lib.rs

Output: one JSON object on stdout, exit 0 on pass, 1 on fail, 2 on usage.
Stdlib only — nothing to install.
"""
import json, os, shutil, subprocess, sys, tempfile, tomllib
from pathlib import Path

TARGET = os.environ.get("SOROBAN_WASM_TARGET", "wasm32v1-none")
TAIL = 30


def run(cmd, cwd):
    """Run a command; return (ok, combined log)."""
    try:
        p = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True)
        return p.returncode == 0, p.stdout + p.stderr
    except OSError as e:
        return False, str(e)


def version(binary):
    ok, out = run([binary, "--version"], cwd=None)
    return out.strip() if ok else "unknown"


def tail(s):
    return "\n".join(s.splitlines()[-TAIL:])


def wasm_size(work):
    d = work / "target" / TARGET / "release"
    files = sorted(d.glob("*.wasm")) if d.exists() else []
    return files[0].stat().st_size if files else None


def load_budget(task_dir):
    p = task_dir / "budget.toml"
    if not p.exists():
        return {}
    with open(p, "rb") as f:
        return tomllib.load(f)


def score(task_dir: Path, lib_path: Path, source: str):
    task = task_dir.name
    work = Path(tempfile.mkdtemp(prefix=f"sorobeval-{task}-"))
    src = work / "src"
    src.mkdir()

    # 1. Assemble a fresh crate: task Cargo.toml + submitted lib.rs + task tests.
    shutil.copy(task_dir / "reference" / "Cargo.toml", work / "Cargo.toml")
    lib = lib_path.read_text()
    if "mod test;" not in lib:
        lib += "\n#[cfg(test)]\nmod test;\n"
    (src / "lib.rs").write_text(lib)
    shutil.copy(task_dir / "tests" / "test.rs", src / "test.rs")

    # 2. Gate: build to wasm (offline — the sandbox has a warmed registry).
    build_ok, build_log = run(
        ["cargo", "build", "--release", "--offline", "--target", TARGET], work)

    # 3. Gate: tests (native, testutils feature).
    tests_ok, test_log = (run(["cargo", "test", "--release", "--offline"], work)
                          if build_ok else (False, ""))

    # 4. Gate: budget.
    budget = load_budget(task_dir)
    size = wasm_size(work) if build_ok else None
    max_bytes = int(budget.get("max_wasm_bytes", 0))
    budget_ok = build_ok and size is not None and (max_bytes == 0 or size <= max_bytes)
    # max_cpu_insns: measured via `stellar contract invoke --cost` in week 2.

    result = {
        "task": task,
        "source": source,
        "gates": {"build": build_ok, "tests": tests_ok, "budget": budget_ok},
        "pass": build_ok and tests_ok and budget_ok,
        "wasm_bytes": size,
        "build_log_tail": tail(build_log),
        "test_log_tail": tail(test_log),
        "toolchain": {"rustc": version("rustc"), "cargo": version("cargo"),
                      "target": TARGET},
    }
    shutil.rmtree(work, ignore_errors=True)
    return result


def main(argv):
    if len(argv) == 3 and argv[1] == "reference":
        task_dir = Path(argv[2])
        r = score(task_dir, task_dir / "reference" / "src" / "lib.rs", "reference")
    elif len(argv) == 4 and argv[1] == "score":
        r = score(Path(argv[2]), Path(argv[3]), argv[3])
    else:
        print("usage: sorobeval.py reference <task-dir> | score <task-dir> <lib.rs>",
              file=sys.stderr)
        return 2
    print(json.dumps(r))
    return 0 if r["pass"] else 1




# ----------------------------------------------------------------------------
# Week 1: generation (HOST side, needs network), matrix, leaderboard.
# ----------------------------------------------------------------------------
import hashlib, re, time, urllib.request, urllib.error

ROOT = Path(__file__).resolve().parent.parent
RUST_BLOCK = re.compile(r"```(?:rust|rs)\s*\n(.*?)```", re.S)
ANY_BLOCK = re.compile(r"```[a-zA-Z]*\s*\n(.*?)```", re.S)


def load_models():
    with open(ROOT / "runner" / "models.toml", "rb") as f:
        return tomllib.load(f)


def extract_rust(text):
    """Pull the contract source out of a chatty reply. Extraction failure is
    scored as a build failure, never as a runner error."""
    blocks = RUST_BLOCK.findall(text) or ANY_BLOCK.findall(text)
    if blocks:
        return max(blocks, key=len)
    return text if "#[contract]" in text else ""


def system_prompt(condition):
    base = (ROOT / "context" / "system_prompt.md").read_text()
    if condition == "bare":
        return base, None
    skill = ROOT / "context" / "stellar-dev-skill.md"
    if not skill.exists():
        sys.exit("context/stellar-dev-skill.md missing — see context/README.md")
    s = skill.read_text()
    return base + "\n\n# Reference material\n\n" + s, hashlib.sha256(s.encode()).hexdigest()


def chat(model_cfg, system, user, max_tokens, temperature):
    url = model_cfg["base_url"].rstrip("/") + "/chat/completions"
    key = os.environ.get(model_cfg.get("api_key_env") or "", "")
    body = json.dumps({
        "model": model_cfg["model"], "max_tokens": max_tokens,
        "temperature": temperature,
        "messages": [{"role": "system", "content": system},
                     {"role": "user", "content": user}],
    }).encode()
    headers = {"Content-Type": "application/json",
               "HTTP-Referer": "https://sorob.ai", "X-Title": "SorobEval"}
    if key:
        headers["Authorization"] = f"Bearer {key}"
    for attempt in range(4):
        try:
            req = urllib.request.Request(url, data=body, headers=headers)
            with urllib.request.urlopen(req, timeout=300) as r:
                data = json.load(r)
            text = data["choices"][0]["message"]["content"] or ""
            usage = data.get("usage", {})
            if not text:
                usage = dict(usage, raw_response=data)
            return text, usage, None
        except Exception as e:
            err = str(e)
            time.sleep(2 ** attempt)
    return "", {}, err


def generate(only_models=None, only_tasks=None):
    cfg = load_models()
    run = cfg["run"]
    models = [m for m in cfg["models"] if not only_models or m["name"] in only_models]
    tasks = [t for t in run["tasks"] if not only_tasks or t in only_tasks]
    for m in models:
        if m["model"].startswith("TODO"):
            sys.exit(f"models.toml: fill the model id for {m['name']}")
    for task in tasks:
        spec = (ROOT / "tasks" / task / "spec.md").read_text()
        for cond in run["conditions"]:
            system, skill_sha = system_prompt(cond)
            for m in models:
                for i in range(run["samples"]):
                    out = ROOT / "samples" / task / m["name"] / cond
                    out.mkdir(parents=True, exist_ok=True)
                    lib, meta = out / f"{i}.rs", out / f"{i}.json"
                    if meta.exists():
                        continue  # resumable
                    t0 = time.time()
                    text, usage, err = chat(m, system, spec, run["max_tokens"], run["temperature"])
                    code = extract_rust(text)
                    lib.write_text(code)
                    (out / f"{i}.raw.txt").write_text(text)
                    meta.write_text(json.dumps({
                        "task": task, "model": m["name"], "model_id": m["model"],
                        "condition": cond, "sample": i, "skill_sha256": skill_sha,
                        "seconds": round(time.time() - t0, 1), "usage": usage,
                        "extracted": bool(code), "api_error": err,
                    }))
                    print(f"{task} {m['name']} {cond} #{i}: {'ok' if code else 'NO CODE'}"
                          f"{' ERR ' + err if err else ''}", file=sys.stderr)


def sample_status(info, max_tokens):
    """Why a sample has no (valid) contract: ok / refused / truncated / error / no_code.
    Refusals and truncations still count as failures; they are reported separately."""
    if info.get("api_error"):
        return "error"
    u = info.get("usage") or {}
    ch = ((u.get("raw_response") or {}).get("choices") or [{}])[0]
    fr, nat = ch.get("finish_reason"), ch.get("native_finish_reason")
    if fr == "content_filter" or nat == "refusal" or (ch.get("message") or {}).get("refusal"):
        return "refused"
    if fr == "length" or (u.get("completion_tokens") or 0) >= max_tokens:
        return "truncated"
    if not info.get("extracted"):
        return "no_code"
    return "ok"


def annotate():
    """Stamp the status field onto existing results without rebuilding anything."""
    max_tokens = load_models()["run"]["max_tokens"]
    n = 0
    for res in sorted((ROOT / "results").rglob("*.json")):
        meta = ROOT / "samples" / res.relative_to(ROOT / "results")
        if not meta.exists():
            continue
        r = json.loads(res.read_text())
        r["status"] = sample_status(json.loads(meta.read_text()), max_tokens)
        res.write_text(json.dumps(r)); n += 1
    print(f"annotated {n} results", file=sys.stderr)


def score_all():
    """SANDBOX side: score every generated sample into results/<task>/..."""
    max_tokens = load_models()["run"]["max_tokens"]
    for meta in sorted((ROOT / "samples").rglob("*.json")):
        lib = meta.with_suffix(".rs")
        res = ROOT / "results" / meta.relative_to(ROOT / "samples")
        if res.exists():
            continue  # resumable
        info = json.loads(meta.read_text())
        if info["extracted"]:
            r = score(ROOT / "tasks" / info["task"], lib, str(lib.relative_to(ROOT)))
        else:
            r = {"task": info["task"], "source": str(lib), "pass": False,
                 "gates": {"build": False, "tests": False, "budget": False},
                 "wasm_bytes": None, "build_log_tail": "extraction failed",
                 "test_log_tail": "", "toolchain": {}}
        r.update({k: info[k] for k in ("model", "model_id", "condition", "sample", "skill_sha256", "usage", "seconds")})
        r["status"] = sample_status(info, max_tokens)
        res.parent.mkdir(parents=True, exist_ok=True)
        res.write_text(json.dumps(r))
        print(f"{info['task']} {info['model']} {info['condition']} #{info['sample']}: "
              f"{'PASS' if r['pass'] else 'fail ' + str(r['gates'])}", file=sys.stderr)


def leaderboard():
    rows = [json.loads(p.read_text()) for p in (ROOT / "results").rglob("*.json")]
    if not rows:
        sys.exit("no results yet")
    cfg = load_models(); n = cfg["run"]["samples"]
    tasks = sorted({r["task"] for r in rows})
    agg = {}
    for r in rows:
        k = (r["model"], r["condition"])
        a = agg.setdefault(k, {"pass": 0, "total": 0, "any": {}, "build": 0, "tests": 0, "refused": 0, "truncated": 0, "nocode": 0})
        a["total"] += 1; a["pass"] += (r["gates"]["build"] and r["gates"]["tests"])
        a["build"] += r["gates"]["build"]; a["tests"] += r["gates"]["tests"]
        st = r.get("status", "ok")
        if st == "refused": a["refused"] += 1
        elif st == "truncated": a["truncated"] += 1
        elif st in ("error", "no_code"): a["nocode"] += 1
        a["any"][r["task"]] = a["any"].get(r["task"], False) or (r["gates"]["build"] and r["gates"]["tests"])
    out = ["# SorobEval v0 — leaderboard", "",
           f"Tasks: {len(tasks)} · samples per cell: {n} · pass = build + tests; refused / truncated / no-code samples count as failures and are listed separately (wasm size reported, not gated in v0)", "",
           "| model | condition | pass rate | pass@%d | build ok | tests ok | refused | truncated | no code |" % n,
           "|---|---|---|---|---|---|---|---|---|"]
    for (m, c), a in sorted(agg.items(), key=lambda kv: -kv[1]["pass"] / max(kv[1]["total"], 1)):
        pr = a["pass"] / a["total"]
        pk = sum(a["any"].values()) / len(tasks)
        out.append(f"| {m} | {c} | {pr:.0%} | {pk:.0%} | {a['build']/a['total']:.0%} | {a['tests']/a['total']:.0%} | {a['refused']} | {a['truncated']} | {a['nocode']} |")
    tc = next((r["toolchain"] for r in rows if r.get("toolchain")), {})
    out += ["", f"Toolchain: {tc}", ""]
    (ROOT / "results" / "leaderboard.md").write_text("\n".join(out))
    print("\n".join(out))


if __name__ == "__main__" and len(sys.argv) > 1 and sys.argv[1] in ("generate", "score-all", "leaderboard", "annotate"):
    cmd = sys.argv[1]
    if cmd == "generate":
        generate(only_models=sys.argv[2].split(",") if len(sys.argv) > 2 else None,
                 only_tasks=sys.argv[3].split(",") if len(sys.argv) > 3 else None)
    elif cmd == "score-all":
        score_all()
    elif cmd == "annotate":
        annotate()
    else:
        leaderboard()
    sys.exit(0)

if __name__ == "__main__":
    sys.exit(main(sys.argv))

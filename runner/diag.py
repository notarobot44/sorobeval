import json
from pathlib import Path
rows=[json.loads(p.read_text()) for p in Path("results").rglob("*.json")]
print("T5: wasm bytes of samples that build and pass tests (budget=1356)")
for r in sorted(rows,key=lambda r:(r["model"],r["condition"])):
    if r["task"]=="05-admin-gated" and r["gates"]["tests"]: print(f'  {r["model"]}/{r["condition"]} #{r["sample"]}: {r["wasm_bytes"]}')
print("\nGPT skill T7: first build error")
for r in rows:
    if r["model"]=="gpt-6.1-sol" and r["condition"]=="skill" and r["task"]=="07-token-pay":
        print("  #",r["sample"]); print("  "+"\n  ".join(l for l in r["build_log_tail"].splitlines() if "error" in l or "-->" in l)[:800]); break
print("\nextraction failures: completion tokens (max_tokens=4000)")
for r in sorted(rows,key=lambda r:(r["model"],r["task"])):
    if r["build_log_tail"]=="extraction failed":
        u=r.get("usage") or {}; print(f'  {r["model"]}/{r["condition"]} {r["task"]} #{r["sample"]}: completion_tokens={u.get("completion_tokens")} finish={u.get("finish_reason","?")}')

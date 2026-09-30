import json
from pathlib import Path
rows=[json.loads(p.read_text()) for p in Path("results").rglob("*.json")]
ok=lambda r: r["gates"]["build"] and r["gates"]["tests"]
tasks=sorted({r["task"] for r in rows}); cells=sorted({(r["model"],r["condition"]) for r in rows})
print("per-task pass count (of 3), pass = build + tests"); print("model/condition".ljust(28)+" ".join(t[:2] for t in tasks))
for m,c in cells:
    print(f"{m}/{c}".ljust(28)+" ".join(str(sum(ok(r) for r in rows if (r["model"],r["condition"],r["task"])==(m,c,t))).rjust(2) for t in tasks))
print("\nfailure causes per cell (exclusive): refused / no-code / build / tests   | over-size (informational)")
def kind(r):
    if r["build_log_tail"]=="extraction failed":
        fr=(r.get("usage") or {}).get("raw_response",{}).get("choices",[{}])[0].get("finish_reason")
        return "refused" if fr=="content_filter" else "no-code"
    if not r["gates"]["build"]: return "build"
    if not r["gates"]["tests"]: return "tests"
    return None
for m,c in cells:
    rs=[r for r in rows if (r["model"],r["condition"])==(m,c)]
    k=[kind(r) for r in rs]
    over=sum(1 for r in rs if ok(r) and not r["gates"]["budget"])
    print(f"{m}/{c}".ljust(28)+f'{k.count("refused"):2} {k.count("no-code"):2} {k.count("build"):2} {k.count("tests"):2}   | {over:2}')

# memwright

<description>
Offline game hacking toolkit with a **Rust** core (memory read/write/scan via PyO3) and a **Python** layer (local LLM agents, PyTorch models).
</description>

```
core/               Rust crate -> memwright._core
python/memwright/
  scanner/          iterative value scanning (Cheat-Engine style)
  agents/           local LLM (llama.cpp) + agent loop
  models/           PyTorch nets (value classifier)
  cli.py            `memwright` CLI
```

## Setup
```bash
python -m venv .venv && source .venv/bin/activate
pip install maturin
maturin develop -E llm,nn,dev
memwright regions <pid>
memwright scan <pid> 100
pytest && cargo test
```
Put a GGUF model at `~/.memwright/models/model.gguf`.
Reading another process's memory needs ptrace permissions
(`sudo sysctl kernel.yama.ptrace_scope=0` or run as root).

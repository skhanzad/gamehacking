VENV   := .venv
BIN    := $(VENV)/bin
PY     := $(BIN)/python
EXTRAS ?= llm,nn,dev

.PHONY: help venv dev build release test test-rust test-py lint fmt run clean

help:  ## Show this help
	@grep -E '^[a-z-]+:.*##' $(MAKEFILE_LIST) | awk -F':.*## ' '{printf "  \033[36m%-10s\033[0m %s\n", $$1, $$2}'

UV     := uv
PYVER  ?= 3.11

$(PY):
	@command -v $(UV) >/dev/null || { echo "uv not found: curl -LsSf https://astral.sh/uv/install.sh | sh"; exit 1; }
	$(UV) venv --python $(PYVER) $(VENV)
	$(UV) pip install --python $(PY) "maturin>=1.5,<2"

venv: $(PY)  ## Create the virtualenv

dev: $(PY)  ## Build Rust core + install package in editable mode (rerun after Rust changes)
	env -u CONDA_PREFIX maturin develop --uv -E $(EXTRAS)

build: dev  ## Alias for dev

release: $(PY)  ## Build optimized wheel into target/wheels
	env -u CONDA_PREFIX $(BIN)/maturin build --release

test: test-rust test-py  ## Run all tests

test-rust:  ## Run Rust tests
	cargo test

test-py: dev  ## Run Python tests
	$(BIN)/pytest -q

lint:  ## Lint Rust and Python
	cargo clippy -- -D warnings
	$(BIN)/ruff check python tests

fmt:  ## Format Rust and Python
	cargo fmt
	$(BIN)/ruff format python tests

run: dev  ## Run CLI, e.g. make run ARGS="ps doom"
	$(BIN)/memwright $(ARGS)

clean:  ## Remove build artifacts
	cargo clean
	rm -f python/memwright/*.so
	find . -name __pycache__ -type d -prune -exec rm -rf {} +

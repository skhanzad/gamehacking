import os

from rapidfuzz import fuzz, utils

from memwright import ProcessInfo, list_processes

# Command lines are long and noisy, so a match there counts a bit less
# than a match on the process name.
CMDLINE_WEIGHT = 0.9


def _score(query: str, p: ProcessInfo) -> float:
    """Best similarity between the query and a process's name/executable/cmdline."""
    opts = {"processor": utils.default_process}
    exe = os.path.basename(p.cmdline.split(" ", 1)[0]) if p.cmdline else ""
    return max(
        fuzz.WRatio(query, p.name, **opts) if p.name else 0,
        fuzz.WRatio(query, exe, **opts) if exe else 0,
        # partial_ratio: is the query (nearly) a substring of the cmdline?
        fuzz.partial_ratio(query, p.cmdline, **opts) * CMDLINE_WEIGHT if p.cmdline else 0,
    )


def get_ps_fuzzy(
    name: str | None,
    score_cutoff: float = 60,
    limit: int | None = 10,
) -> list[ProcessInfo]:
    """
    Return processes whose name, executable or cmdline fuzzy-matches
    the query, best match first. With no query, return every process.
    """
    # Our own cmdline (and our parent's, e.g. `make run ARGS="ps doom"`)
    # contains the query, so skip both.
    skip = {os.getpid(), os.getppid()}
    procs = [p for p in list_processes() if p.pid not in skip]

    if not name or not name.strip():
        return procs

    scored = [(s, p) for p in procs if (s := _score(name, p)) >= score_cutoff]
    scored.sort(key=lambda sp: sp[0], reverse=True)
    return [p for _, p in scored[:limit]]


def list_all_processes() -> list[ProcessInfo]:
    return list(list_processes())

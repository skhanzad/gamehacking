import typer
from rich import print
from rich.table import Table
from pathlib import Path

__REPO_ROOT__ = Path(__file__).parent.parent.parent
__README__ = __REPO_ROOT__ / "README.md"

with open(__README__, "r") as f:
    __README_CONTENT__ = f.read()

content = __README_CONTENT__
content = content[content.find("<description>")+len("<description>"):
                  content.find("</description>")]

app = typer.Typer(help=content,
                  no_args_is_help=True,
                  rich_markup_mode="markdown")


@app.command()
def regions(pid: int):
    """List the target process's memory regions."""
    from memwright import Process
    t = Table("start", "end", "perms")
    for s, e, p in Process(pid).regions():
        t.add_row(hex(s), hex(e), p)
    print(t)


@app.command()
def scan(pid: int, value: int, dtype: str = "i32"):
    """Run a first scan for an exact value."""
    from memwright.scanner.session import ScanSession
    s = ScanSession(pid, dtype)
    n = s.first_scan(value)
    print(f"[green]{n}[/] candidates")
    for a in s.candidates[:20]:
        print(hex(a))


if __name__ == "__main__":
    app()

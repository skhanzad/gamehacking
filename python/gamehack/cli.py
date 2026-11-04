import typer
from rich import print
from rich.table import Table

app = typer.Typer(help="Offline game hacking toolkit")


@app.command()
def regions(pid: int):
    """List the target process's memory regions."""
    from gamehack import Process
    t = Table("start", "end", "perms")
    for s, e, p in Process(pid).regions():
        t.add_row(hex(s), hex(e), p)
    print(t)


@app.command()
def scan(pid: int, value: int, dtype: str = "i32"):
    """Run a first scan for an exact value."""
    from gamehack.scanner.session import ScanSession
    s = ScanSession(pid, dtype)
    n = s.first_scan(value)
    print(f"[green]{n}[/] candidates")
    for a in s.candidates[:20]:
        print(hex(a))


if __name__ == "__main__":
    app()

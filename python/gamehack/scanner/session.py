"""Cheat-Engine-style iterative value scanning on top of the Rust core."""
import struct
from gamehack import Process

FMT = {"i32": "<i", "i64": "<q", "f32": "<f", "f64": "<d"}


class ScanSession:
    def __init__(self, pid: int, dtype: str = "i32"):
        self.proc = Process(pid)
        self.fmt = FMT[dtype]
        self.size = struct.calcsize(self.fmt)
        self.candidates: list[int] = []

    def first_scan(self, value) -> int:
        self.candidates = self.proc.scan(list(struct.pack(self.fmt, value)))
        return len(self.candidates)

    def next_scan(self, value) -> int:
        target = struct.pack(self.fmt, value)
        keep = []
        for a in self.candidates:
            try:
                if bytes(self.proc.read(a, self.size)) == target:
                    keep.append(a)
            except OSError:
                pass
        self.candidates = keep
        return len(keep)

    def write(self, addr: int, value) -> None:
        self.proc.write(addr, list(struct.pack(self.fmt, value)))

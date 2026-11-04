"""Agent loop: the LLM picks scanner actions (tool calls) to find a game value."""
import json
from gamehack.agents.llm import LocalLLM
from gamehack.scanner.session import ScanSession

SYSTEM = """You help find variables in a running single-player game's memory.
Reply with JSON only: {"action": "first_scan"|"next_scan"|"done", "value": <number>, "note": "..."}"""


class ScanAgent:
    def __init__(self, session: ScanSession, llm: LocalLLM):
        self.s, self.llm = session, llm

    def step(self, observation: str) -> dict:
        ctx = f"Candidates: {len(self.s.candidates)}\nObservation: {observation}"
        try:
            act = json.loads(self.llm.chat(SYSTEM, ctx))
        except json.JSONDecodeError:
            return {"action": "error"}
        if act.get("action") in ("first_scan", "next_scan"):
            act["result"] = getattr(self.s, act["action"])(act["value"])
        return act

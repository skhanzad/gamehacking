"""Local LLM wrapper (llama.cpp). Runs fully offline."""
from gamehack.config import settings


class LocalLLM:
    def __init__(self, model_path: str | None = None):
        from llama_cpp import Llama  # optional dep: pip install .[llm]
        path = model_path or str(settings.model_dir / settings.llm_model)
        self.llm = Llama(model_path=path, n_ctx=settings.llm_ctx, verbose=False)

    def chat(self, system: str, user: str, max_tokens: int = 512) -> str:
        out = self.llm.create_chat_completion(
            messages=[{"role": "system", "content": system},
                      {"role": "user", "content": user}],
            max_tokens=max_tokens,
        )
        return out["choices"][0]["message"]["content"]

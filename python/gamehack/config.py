from pathlib import Path
from pydantic import BaseModel


class Settings(BaseModel):
    model_dir: Path = Path.home() / ".gamehack" / "models"
    llm_model: str = "model.gguf"  # local GGUF for llama.cpp, no network
    llm_ctx: int = 8192
    device: str = "cpu"


settings = Settings()

import os
from dotenv import load_dotenv

load_dotenv()

TALLY_HOST = os.getenv("TALLY_HOST", "http://localhost:9000")
TALLY_AGENT_PORT = int(os.getenv("TALLY_AGENT_PORT", "8004"))
REQUEST_TIMEOUT_SECONDS = int(os.getenv("REQUEST_TIMEOUT_SECONDS", "30"))
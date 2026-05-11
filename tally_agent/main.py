# tally_agent/main.py
import logging
from fastapi import FastAPI, HTTPException

from tally_agent.config import TALLY_AGENT_PORT
from tally_agent.schemas import TallyCallRequest, TallyCallResponse
from tally_agent.tally_client import TallyClient, TallyClientError

logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

app = FastAPI(title="Tally Agent", version="1.0.0")

# Initialize TallyClient
tally_client = TallyClient()


@app.get("/health")
def health():
    """Health check endpoint."""
    return {"status": "ok", "service": "tally-agent"}


@app.post("/call", response_model=TallyCallResponse)
def call_tally(request: TallyCallRequest):
    """
    Single endpoint to relay XML requests to Tally.

    Args:
        request_type: Type of Tally request (e.g., "Export Data", "Import Data")
        xml_body: The XML envelope to send to Tally

    Returns:
        Tally response or error
    """
    logger.info(f"Received request_type={request.request_type}, xml_body length={len(request.xml_body)}")

    try:
        response = tally_client.post_xml(request.xml_body, request_name=request.request_type)
        return TallyCallResponse(success=True, tally_response=response)
    except TallyClientError as e:
        logger.error(f"Tally call failed: {e}")
        raise HTTPException(status_code=502, detail=f"Tally error: {str(e)}")


@app.get("/")
def root():
    """Root endpoint with service info."""
    return {
        "service": "tally-agent",
        "version": "1.0.0",
        "endpoints": ["/call", "/health"]
    }


if __name__ == "__main__":
    import uvicorn
    uvicorn.run("tally_agent.main:app", host="0.0.0.0", port=TALLY_AGENT_PORT, reload=True)
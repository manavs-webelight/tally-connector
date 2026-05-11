# tests/test_main.py
import pytest
from fastapi.testclient import TestClient
from tally_agent.main import app

client = TestClient(app)


def test_call_endpoint_missing_xml_body():
    """Missing xml_body should return 422 validation error."""
    response = client.post("/call", json={
        "request_type": "Export Data"
    })
    assert response.status_code == 422


def test_health_endpoint():
    """Health check should work."""
    response = client.get("/health")
    assert response.status_code == 200
    data = response.json()
    assert data["status"] == "ok"


def test_root_endpoint():
    """Root endpoint should return service info."""
    response = client.get("/")
    assert response.status_code == 200
    data = response.json()
    assert data["service"] == "tally-agent"
    assert "/call" in data["endpoints"]
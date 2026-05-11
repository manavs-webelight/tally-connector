# Tally Agent

Simple XML relay service that forwards XML requests to Tally accounting software.

## Purpose

Tally Agent is a thin passthrough service - it receives XML from callers and forwards to Tally's XML API. No business logic, no JSON-to-XML conversion.

## Endpoints

### POST /call

Relays XML to Tally API.

**Request:**
```json
{
  "request_type": "Export Data",
  "xml_body": "<ENVELOPE>...</ENVELOPE>"
}
```

**Response:**
```json
{
  "success": true,
  "tally_response": "<ENVELOPE>...</ENVELOPE>",
  "error": null
}
```

### GET /health

Health check.

## Running

```bash
cd tally-agent
poetry install
poetry run python -m tally_agent.main
```

Or directly:

```bash
cd tally-agent
python -m tally_agent.main
```

## Configuration

Create `.env` file:
```
TALLY_HOST=http://192.168.7.189:9001
TALLY_AGENT_PORT=8004
REQUEST_TIMEOUT_SECONDS=30
```

## Architecture

```
Caller (e.g., Main Connector Server)
    |
    | POST /call {request_type, xml_body}
    v
+-----------------+
|  Tally Agent    |  <-- Simple FastAPI passthrough
+-----------------+
    |
    | POST XML to TALLY_HOST
    v
+----------+
|  Tally  |
+----------+
```

## Project Structure

```
tally-agent/
├── pyproject.toml
├── .env
├── tally_agent/
│   ├── __init__.py
│   ├── config.py
│   ├── main.py
│   ├── schemas.py
│   └── tally_client.py
└── tests/
    ├── test_main.py
    └── test_tally_client.py
```
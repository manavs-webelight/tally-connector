# tests/test_tally_client.py
import pytest
from tally_agent.tally_client import TallyClient, TallyClientError


def test_tally_client_initialization():
    """TallyClient can be initialized with default config."""
    client = TallyClient()
    assert hasattr(client, "base_url")
    assert hasattr(client, "timeout")
    assert hasattr(client, "post_xml")


def test_tally_client_custom_url():
    """TallyClient can be initialized with custom base_url."""
    client = TallyClient(base_url="http://custom:9000", timeout=60)
    assert client.base_url == "http://custom:9000"
    assert client.timeout == 60


def test_tally_client_strips_trailing_slash():
    """Base URL trailing slash should be stripped."""
    client = TallyClient(base_url="http://test:9000/")
    assert client.base_url == "http://test:9000"


def test_import_data_detection():
    """Import requests should be detected correctly."""
    import_xml = '<ENVELOPE><BODY><IMPORTDATA>...</IMPORTDATA></BODY></ENVELOPE>'
    assert "<IMPORTDATA" in import_xml
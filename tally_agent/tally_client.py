# tally_agent/tally_client.py
import logging
import requests
from typing import Optional

from tally_agent.config import TALLY_HOST, REQUEST_TIMEOUT_SECONDS

logger = logging.getLogger(__name__)


class TallyClientError(Exception):
    pass


class TallyClient:
    def __init__(self, base_url: Optional[str] = None, timeout: Optional[int] = None):
        self.base_url = (base_url or TALLY_HOST).rstrip("/")
        self.timeout = timeout or REQUEST_TIMEOUT_SECONDS

    def post_xml(self, xml: str, request_name: str = "Unknown") -> str:
        """
        Posts XML to Tally and returns the response.
        Handles the ?xmlrequest=true fallback for non-Import requests.
        """
        headers = {"Content-Type": "application/xml", "Accept": "application/xml"}
        try:
            resp = requests.post(
                self.base_url,
                data=xml.encode("utf-8"),
                headers=headers,
                timeout=self.timeout
            )
            resp.raise_for_status()
            text = resp.text

            # For Import requests, do not retry to avoid duplicate creations
            is_import = "<IMPORTDATA" in xml or "<IMPORTDATA>" in xml

            if (not is_import) and (not text.lstrip().startswith("<") or "<ENVELOPE" not in text):
                alt_url = self.base_url + "/?xmlrequest=true"
                logger.info(f"Primary response not valid XML, trying fallback: {alt_url}")
                resp2 = requests.post(alt_url, data=xml.encode("utf-8"), headers=headers, timeout=self.timeout)
                resp2.raise_for_status()
                text = resp2.text

            return text
        except requests.RequestException as e:
            logger.error(f"Tally request failed: {e}")
            raise TallyClientError(str(e)) from e
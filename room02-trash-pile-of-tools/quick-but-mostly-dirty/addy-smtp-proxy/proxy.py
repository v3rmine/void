import asyncio
import logging
import os
import re
import smtplib

from aiosmtpd.controller import Controller
from aiosmtpd.smtp import AuthResult, SMTP, Envelope, LoginPassword, Session
from dotenv import load_dotenv
from email import policy
from email.message import EmailMessage
from email.parser import BytesHeaderParser
from email.generator import BytesGenerator
from io import BytesIO
from typing import Literal

EMAIL_REGEX = r"^(?:(?P<display_name>[^<]+)\s+<)?(?P<email_address>[^<@]+@[^>]+)>?$"

class AddyProxyHandler:
    def __init__(self, upstream_smtp_host, upstream_smtp_port, upstream_from):
        self.upstream_smtp_host = upstream_smtp_host
        self.upstream_smtp_port = upstream_smtp_port
        self.upstream_smtp_user: str | Literal[False] = os.getenv("SMTP_USER", False)
        self.upstream_smtp_password: str | Literal[False] = os.getenv("SMTP_PASSWORD", False)
        self.upstream_from = upstream_from
        self.upstream_ssl = os.getenv("UPSTREAM_SSL", "ssl")
        self.debug_upstream = bool(os.getenv("UPSTREAM_DEBUG", False))
        self.proxy_allowed_senders = set(os.getenv("PROXY_ALLOWED_SENDER", "").split(","))

    async def handle_DATA(self, _server: asyncio.Server, session: Session, envelope: Envelope):
        """
        Modify the mail and forward it
        """
        logging.debug(f"Receiving message from: {session.peer}")
        logging.debug(f"MAIL FROM (envelope): {envelope.mail_from}")
        logging.debug(f"Original RCPT TO(s) (envelope): {envelope.rcpt_tos}")
        logging.debug(f"Original RCPT OPTIONS(s) (envelope): {envelope.rcpt_options}")

        # Parse email content
        parser = BytesHeaderParser(policy=policy.default)
        message: EmailMessage
        if isinstance(envelope.content, bytes):
            message = parser.parse(BytesIO(envelope.content))
        else:
            logging.warning("Received message isnt bytes")
            return '550 Message could not be forwarded: Received invalid content'

        original_from_header = message.get("From")
        logging.debug(f"Original From (header): {original_from_header}")
        message.replace_header("From", self.upstream_from)

        # Rewrite the recipients
        new_rcpt_tos = []
        # Extract the mail, ignoring any display name (Addy forward using a static name)
        match = re.match(EMAIL_REGEX, str(envelope.mail_from))
        sender_email_address = envelope.mail_from
        if match:
            sender_email_address = match.group("email_address")
        else:
            logging.warning(f"Sender doesn't match mail regex {envelope.mail_from}")
            return '550 Message could not be forwarded: Internal error'

        sender_envelope_local_part, sender_envelope_domain = sender_email_address.split('@', 1)

        # Check if sender_email is allowed
        if sender_email_address not in self.proxy_allowed_senders:
            logging.error(f"Sender {sender_email_address} is not in the allowed senders list.")
            return '550 Message could not be forwarded: Sender not allowed'

        for original_rcpt_to in envelope.rcpt_tos:
            match = re.match(EMAIL_REGEX, original_rcpt_to)
            if match:
                name_part = match.group("display_name")
                email_part = match.group("email_address")
                recipient_local_part, recipient_domain = email_part.split('@', 1)
                new_rcpt_address = f"{sender_envelope_local_part}+{recipient_local_part}={recipient_domain}@{sender_envelope_domain}"
                if name_part:
                    new_rcpt_address = f"{name_part} <{new_rcpt_address}>"

                new_rcpt_tos.append(new_rcpt_address)
            else:
                logging.warning(f"Recipient doesn't match mail regex {original_rcpt_to}")

        logging.debug(f"Rewritten RCPT TO(s) (envelope): {new_rcpt_tos}")

        # Modify the mail content
        modified_content_bytes = BytesIO()
        g = BytesGenerator(modified_content_bytes, policy=policy.SMTPUTF8)
        g.flatten(message, unixfrom=False)
        final_content = modified_content_bytes.getvalue()

        # Forward the message
        loop = asyncio.get_running_loop()
        try:
            await loop.run_in_executor(
                None,
                self._forward_email,
                self.upstream_from,
                new_rcpt_tos,
                final_content
            )
            logging.debug("Message forwarded successfully.")
            return '250 OK'
        except Exception as e:
            logging.warning(f"Error forwarding message: {e}")
            return '550 Message could not be forwarded: Failed to send'

    def _forward_email(self, mail_from, rcpt_tos, content_bytes):
        """
        Designed to be run in a separate thread.
        """
        try:
            # Upstream initialisation
            if self.upstream_ssl == "starttls":
                server = smtplib.SMTP(self.upstream_smtp_host, self.upstream_smtp_port)
                server.starttls()
                server.ehlo()
            else:
                server = smtplib.SMTP_SSL(self.upstream_smtp_host, self.upstream_smtp_port)

            # Upstream authentication
            if self.upstream_smtp_user and self.upstream_smtp_password:
                server.login(self.upstream_smtp_user, self.upstream_smtp_password)

            server.set_debuglevel(self.debug_upstream)
            server.sendmail(mail_from, rcpt_tos, content_bytes)
        except Exception as e:
            logging.error("Failed to send email via upstream SMTP: {e}")
            raise RuntimeError(f"Failed to send email via upstream SMTP: {e}") from e

def simple_authenticator(_server: SMTP, _session: Session, _envelope: Envelope, mechanism: str, auth_data: LoginPassword):
    if mechanism == "LOGIN" or mechanism == "PLAIN":
        client_user = auth_data.login.decode()
        client_password = auth_data.password.decode()

        logging.info(f"Attempting to authenticate user: {client_user} (valid user: {os.getenv("PROXY_USER")})")

        login_check = client_user == os.getenv("PROXY_USER")
        password_check = client_password == os.getenv("PROXY_PASSWORD")
        if login_check and password_check:
            logging.info(f"Authentication successful for user: {client_user}")
            return AuthResult(success=True, auth_data=auth_data)

        logging.debug(f"Invalid username for {client_user}") if not login_check else None
        logging.debug(f"Invalid password for {client_user}") if not password_check else None

        logging.warning(f"Authentication failed for user: {client_user}")
        return AuthResult(success=False, message="Invalid credentials", handled=False, auth_data=auth_data)
    else:
        logging.warning(f"Unsupported authentication mechanism: {mechanism}")
        return AuthResult(success=False, message="Unsupported authentication mechanism", handled=False, auth_data=auth_data)


async def main():
    load_dotenv()
    logging.basicConfig(level=os.getenv("LOG_LEVEL", default=logging.WARNING))

    proxy_host = os.getenv("PROXY_HOST", "127.0.0.1")
    proxy_port = int(os.getenv("PROXY_PORT", 8025))

    upstream_from = os.getenv("UPSTREAM_FROM")
    if not upstream_from:
        logging.error("UPSTREAM_FROM environment variable not set. Exiting.")
        exit(1)
    upstream_host = os.getenv("UPSTREAM_HOST")
    if not upstream_host:
        logging.error("UPSTREAM_HOST environment variable not set. Exiting.")
        exit(1)
    upstream_port = int(os.getenv("UPSTREAM_PORT", 465))

     # Check for proxy authentication credentials
    proxy_user = os.getenv("PROXY_USER")
    proxy_password = os.getenv("PROXY_PASSWORD")
    if not proxy_user or not proxy_password:
        logging.warning("PROXY_USER or PROXY_PASSWORD environment variables not set. Proxy will run without client authentication.")
        auth_required = False
    else:
        auth_required = True

    handler = AddyProxyHandler(
        upstream_host,
        upstream_port,
        upstream_from,
    )

    # Only work if you can allow self signed certs
    # openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes -subj '/CN=localhost'
    # context = ssl.create_default_context(ssl.Purpose.CLIENT_AUTH)
    # context.load_cert_chain('cert.pem', 'key.pem')

    controller = Controller(
        handler,
        hostname=proxy_host,
        port=proxy_port,
        enable_SMTPUTF8=True,
        authenticator=simple_authenticator,
        auth_required=auth_required,
        auth_require_tls=False,
        # ssl_context=context,
    )

    controller.start()
    logging.info(f"Starting Addy Mail Proxy on {proxy_host}:{proxy_port}...")
    logging.debug(f"Forwarding to upstream: {upstream_host}:{upstream_port}")
    logging.debug(f"Rewriting From header to: {upstream_from}")

    try:
        await asyncio.Event().wait()
    except asyncio.CancelledError:
        pass
    finally:
        controller.stop()
        logging.info("Addy Mail Proxy stopped.")

if __name__ == '__main__':
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        logging.info("\n^C received, shutting down server.")

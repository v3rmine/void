FROM python:3.14.2-alpine
RUN adduser --disabled-password --gecos "" appuser
USER appuser
COPY pyproject.toml poetry.lock ./
RUN pip install poetry && /home/appuser/.local/bin/poetry install --only main --no-root --no-directory
COPY proxy.py .
CMD [ "/home/appuser/.local/bin/poetry", "run", "python", "proxy.py" ]

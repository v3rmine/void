FROM python:alpine

RUN mkdir /.local /config && chown 101:101 -R /.local /config

WORKDIR /config

USER 101:101

RUN pip install pleroma-bot

ENV EVERY_SECONDS 3600

VOLUME /config

CMD watch -n $EVERY_SECONDS /.local/bin/pleroma-bot -c /config/config.yml

FROM python:3.9-rc-alpine

ENV APP_ID "Fill me with your app ID"
ENV INSTALL_ID "Fill me with the installation ID of your app"
ENV APP_PKEY "Fill me with a private key for your app"

RUN apk add --update coreutils bash gcc musl-dev libffi-dev openssl-dev && rm -rf /var/cache/apk/*
RUN pip install check-in

COPY ./drone.sh /drone.sh

CMD ["/drone.sh", $APP_ID, $INSTALL_ID, $APP_PKEY]

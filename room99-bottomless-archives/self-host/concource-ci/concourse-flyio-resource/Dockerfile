FROM flyio/flyctl:latest as fly

FROM python:alpine
COPY --from=fly /flyctl /bin/flyctl

WORKDIR /opt/resource

COPY ./check ./in ./out /opt/resource/

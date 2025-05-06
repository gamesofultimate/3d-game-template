FROM debian:stable-slim

WORKDIR /opt/app

RUN apt-get update -y && apt-get install -y ca-certificates
RUN dpkg-reconfigure -p critical ca-certificates

COPY ./.env ./.env
COPY ./resources ./resources
COPY ./3d_game_server ./dist/3d_game_server

CMD ["/opt/app/dist/3d_game_server"]

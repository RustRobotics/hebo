FROM debian:bullseye

LABEL maintainer="shaohua@biofan.org"
LABEL description="Run hebo in debian docker image."

COPY hebo /usr/bin/hebo
COPY hebo.toml /etc/hebo/hebo.toml

CMD ["/usr/bin/hebo"]

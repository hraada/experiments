ARG CROSS_BASE_IMAGE
FROM $CROSS_BASE_IMAGE

ARG CROSS_DEB_ARCH
ENV DEBIAN_FRONTEND noninteractive

RUN dpkg --add-architecture $CROSS_DEB_ARCH
RUN apt-get update && apt-get -y install libvips-dev:$CROSS_DEB_ARCH
FROM debian:stable-slim
# install required packages
RUN apt update && apt upgrade -y
RUN apt-get install -y curl gcc libssl-dev pkg-config libfreetype6-dev libfontconfig1-dev dpkg-dev git sudo ssh
# this directory is needed to run openssh server
RUN mkdir /run/sshd
# allow sudo
RUN sed -i 's/^%sudo.*ALL=(ALL:ALL) ALL/%sudo ALL=(ALL:ALL) NOPASSWD:ALL/' /etc/sudoers
# create normal user 'builder'
RUN useradd -rm -d /home/builder -G sudo -s /bin/bash builder
USER builder
WORKDIR /home/builder
# install rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
# add cargo to path
ENV PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/bin:/home/builder/.cargo/bin
#
USER root
# set password for builder
RUN echo 'builder:builder' | chpasswd
EXPOSE 22
CMD ["/usr/sbin/sshd", "-D"]

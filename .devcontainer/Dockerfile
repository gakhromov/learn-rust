# Note: You can use any Debian/Ubuntu based image you want. 
FROM debian:buster

# Setup fish
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends fish curl git gcc
RUN chsh -s /usr/bin/fish; mkdir /root/.config; mkdir /root/.config/fish
COPY config.fish /root/.config/fish

# Setup starship
RUN sh -c "$(curl -fsSL https://starship.rs/install.sh)"
COPY starship.toml /root/.config

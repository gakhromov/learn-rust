FROM ubuntu:focal

# Setup important stuff
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends curl git gcc gpg ca-certificates

# Setup starship
RUN curl https://starship.rs/install.sh --output /starship_install.sh; chmod +x /starship_install.sh
RUN /starship_install.sh -y
COPY ./starship.toml /root/.config

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs --output /rust_install.sh; chmod +x /rust_install.sh
RUN /rust_install.sh -y

# Config bash
RUN echo 'eval "$(starship init bash)"' >> $HOME/.bashrc
RUN echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> $HOME/.bash_profile

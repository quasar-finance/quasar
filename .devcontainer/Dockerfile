FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y python3 python3-venv python3-pip git clang curl libssl-dev build-essential wget sudo libclang-dev

ARG INSTALL_ZSH="true"
ARG UPGRADE_PACKAGES="false"
ARG ENABLE_NONROOT_DOCKER="true"
ARG USE_MOBY="true"
ARG USERNAME="vscode"
ARG USER_UID=1000
ARG USER_GID=$USER_UID
COPY library-scripts/*.sh /tmp/library-scripts/
RUN apt-get update \
    && /bin/bash /tmp/library-scripts/common-debian.sh "${INSTALL_ZSH}" "${USERNAME}" "${USER_UID}" "${USER_GID}" "${UPGRADE_PACKAGES}" "true" "true" \
    # Use Docker script from script library to set things up
    && /bin/bash /tmp/library-scripts/docker-in-docker-debian.sh "${ENABLE_NONROOT_DOCKER}" "${USERNAME}" "${USE_MOBY}" \
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/* /tmp/library-scripts

VOLUME [ "/var/lib/docker" ]

# Setting the ENTRYPOINT to docker-init.sh will start up the Docker Engine
# inside the container "overrideCommand": false is set in devcontainer.json.
# The script will also execute CMD if you need to alter startup behaviors.
ENTRYPOINT [ "/usr/local/share/docker-init.sh" ]
CMD [ "sleep", "infinity" ]

# persist zsh history
RUN SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/commandhistory/.zsh_history" \
    && mkdir /commandhistory \
    && touch /commandhistory/.zsh_history \
    && chown -R $USERNAME /commandhistory \
    && echo $SNIPPET >> "/home/$USERNAME/.zshrc"

# install go
RUN cd /tmp && wget https://go.dev/dl/go1.21.6.linux-amd64.tar.gz && tar -xvf go1.21.6.linux-amd64.tar.gz -C /usr/local 
ENV PATH=$PATH:/usr/local/go/bin:/home/$USERNAME/go/bin
ENV GOPATH=/home/$USERNAME/go
ENV GOBIN=/usr/local/go/bin
RUN chown -R $USERNAME /usr/local/go

RUN curl -sL https://deb.nodesource.com/setup_22.x | bash - && apt-get install -y nodejs
RUN curl -sL https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - && echo "deb https://dl.yarnpkg.com/debian/ stable main" | sudo tee /etc/apt/sources.list.d/yarn.list && apt-get update && apt-get install -y yarn
RUN npm i -g typescript ts-node
#RUN python3 -m pip install terra_sdk requests

RUN apt-get update && apt-get install -y pkg-config
# install rust
USER $USERNAME
RUN cd /home/$USERNAME && curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/home/$USERNAME/.cargo/bin:${PATH}"
RUN rustup default stable
RUN rustup target add wasm32-unknown-unknown
RUN rustup component add rust-analysis --toolchain stable-x86_64-unknown-linux-gnu
RUN rustup component add rust-src --toolchain stable-x86_64-unknown-linux-gnu
RUN rustup component add rls --toolchain stable-x86_64-unknown-linux-gnu
RUN cargo install cargo-generate --features vendored-openssl
RUN cargo install just
RUN cargo install cargo-nextest --locked
RUN cargo install taplo-cli --locked
RUN cargo install cargo-watch
RUN cargo install cargo-limit
RUN cd /home/$USERNAME && git clone https://github.com/osmosis-labs/osmosis && cd osmosis && git checkout v25.0.4 && make install

#RUN cd /home/$USERNAME && mkdir -p tmp && cd tmp && curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl" && sudo install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl
#RUN cd /home/$USERNAME/tmp && curl -fsSL -o get_helm.sh https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash -
#RUN cd /home/$USERNAME/tmp && curl -Lo ./kind https://kind.sigs.k8s.io/dl/v0.23.0/kind-linux-amd64 && chmod +x ./kind && sudo mv ./kind /usr/local/bin/kind
#RUN sudo npm install -g @starship-ci/cli && starship install

RUN sudo apt-get update && sudo apt-get install -y unzip protobuf-compiler
#ENV PROTOC_VERSION="$(curl -s \"https://api.github.com/repos/protocolbuffers/protobuf/releases/latest\" | grep -Po \'\"tag_name\": \"v\K[0-9.]+\')"
#RUN curl -Lo protoc.zip "https://github.com/protocolbuffers/protobuf/releases/latest/download/protoc-${PROTOC_VERSION}-linux-x86_64.zip" && sudo unzip -q protoc.zip bin/protoc -d /usr/local && sudo chmod a+x /usr/local/bin/protoc
RUN curl -L https://foundry.paradigm.xyz | bash

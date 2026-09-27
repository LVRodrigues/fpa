#FROM mcr.microsoft.com/devcontainers/rust:1-1
FROM rust:1.98.1

RUN apt update \
    && apt install -y openssl ca-certificates git-flow git \
        zsh nano curl sudo \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
	
RUN useradd -c 'VSCode Desenv User' -m -p vscode -s /bin/zsh vscode

ARG USERNAME=vscode
RUN echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME
	
# Set `DEVCONTAINER` environment variable to help with orientation
ENV DEVCONTAINER=true

USER vscode

#RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
#RUN git clone --depth=1 https://github.com/romkatv/powerlevel10k.git ${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k

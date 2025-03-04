FROM ubuntu:22.04
ENV DEBIAN_FRONTEND=noninteractive

# Habilitar todos los repositorios y actualizar
RUN apt-get update && apt-get install -y software-properties-common && \
    add-apt-repository universe && \
    add-apt-repository restricted && \
    add-apt-repository multiverse && \
    apt-get update

# Instalar dependencias, forzando la instalación de libsoup-3.0-dev
RUN apt-get install -y \
    libwebkit2gtk-4.0-dev \
    libgtk-3-dev \
    webkit2gtk-4.1 \
    javascriptcoregtk-4.1 \
    curl \
    x11-apps \
    libsoup-3.0-dev \
    build-essential \
    libssl-dev \
    || (apt-get install -f -y && apt-get install -y libsoup-3.0-dev) \
    && rm -rf /var/lib/apt/lists/*

# Instalar Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs

# Instalar Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/lib/pkgconfig
ENV DISPLAY=host.docker.internal:0

# Instalar Tauri CLI globalmente
RUN npm install -g @tauri-apps/cli

WORKDIR /app
COPY . .

# Instalar dependencias de Node.js
RUN npm install

VOLUME ["/downloads"]
CMD ["npm", "run", "tauri", "dev"]
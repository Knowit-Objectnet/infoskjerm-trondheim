# Use Ubuntu as base image
FROM ubuntu:latest

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add cargo bin directory to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Set up the working directory
WORKDIR /usr/src/app

# Install dependencies
RUN apt-get update && \
    apt-get install -y curl \
    wget \
    xvfb \
    libxrender1 \
    libxtst6 \
    libxi6 \
    gcc \
    libc6-dev \
    libssl-dev \
    pkg-config \
    x11-apps \
    imagemagick \
    gcc-multilib

# Copy over the source code
COPY . .

RUN rustup target add x86_64-unknown-linux-gnu && \
    cargo build --target x86_64-unknown-linux-gnu --release

# Set up virtual framebuffer for headless mode
ENV DISPLAY=:99

# Start XVFB
CMD Xvfb :99 -screen 0 1024x768x24 -ac & \
    sleep 5 && \
    # Run your Rust app
    ./target/x86_64-unknown-linux-gnu/release/infoskjerm & \
    sleep 5 && \
    # Capture screenshot
    xwd -root -out screenshot.xwd && \
    convert screenshot.xwd screenshot.png && \
    # Make screenshot available outside the container
    mv screenshot.png /usr/src/app/screenshot.png && \
    echo "Screenshot available at /usr/src/app/screenshot.png"
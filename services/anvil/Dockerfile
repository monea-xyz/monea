FROM ghcr.io/foundry-rs/foundry:latest

# Copy our source code into the container
WORKDIR /app

# Build and test the source code
COPY . .

ENTRYPOINT ["anvil"]

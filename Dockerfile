# ---------- Build Dioxus fullstack app ----------
FROM rust:1-trixie AS builder

RUN rustup target add wasm32-unknown-unknown \
 && curl -L --proto '=https' --tlsv1.2 -sSf \
    https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

WORKDIR /app
COPY app/Cargo.toml app/Cargo.lock app/Dioxus.toml app/clippy.toml ./

# Install the dioxus-cli version matching the locked dioxus crate
RUN dx_version="$(grep -A1 '^name = "dioxus"$' Cargo.lock | sed -n 's/^version = "\(.*\)"$/\1/p')" \
 && test -n "$dx_version" \
 && echo "Installing dioxus-cli ${dx_version}" \
 && cargo binstall -y "dioxus-cli@${dx_version}"

COPY app/src ./src
COPY app/assets ./assets

# Produces a server binary + public/ (wasm, js, css)
RUN dx bundle --platform web --release \
 && out_dir="$(find target/dx -type d -path '*/release/web' | head -n1)" \
 && test -n "$out_dir" \
 && exe="$(find "$out_dir" -maxdepth 1 -type f -perm -u+x | head -n1)" \
 && test -n "$exe" \
 && mkdir -p /out \
 && cp "$exe" /out/server \
 && cp -r "$out_dir/public" /out/public

# ---------- Fetch CyberChef release ----------
FROM alpine:3 AS cyberchef

ARG CYBERCHEF_VERSION
RUN apk add --no-cache curl unzip \
 && test -n "$CYBERCHEF_VERSION" \
 && curl -fsSL -o /tmp/cc.zip \
    "https://github.com/gchq/CyberChef/releases/download/${CYBERCHEF_VERSION}/CyberChef_${CYBERCHEF_VERSION}.zip" \
 && mkdir /cyberchef \
 && unzip -q /tmp/cc.zip -d /cyberchef \
 && mv "/cyberchef/CyberChef_${CYBERCHEF_VERSION}.html" /cyberchef/index.html

# ---------- Runtime ----------
FROM gcr.io/distroless/cc-debian13:nonroot

WORKDIR /app
COPY --from=builder /out/ /app/
COPY --from=cyberchef /cyberchef /app/cyberchef

ENV IP=0.0.0.0 \
    PORT=8080 \
    CYBERCHEF_DIR=/app/cyberchef

EXPOSE 8080
CMD ["/app/server"]

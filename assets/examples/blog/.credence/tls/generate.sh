#/bin/bash
set -e

HERE=$(dirname "$(readlink --canonicalize "$BASH_SOURCE")")

# See: https://unix.stackexchange.com/a/104305

openssl ecparam \
    -genkey \
    -name prime256v1 \
    -out "$HERE/key.pem"

openssl req \
    -new \
    -days 365 \
    -nodes \
    -x509 \
    -subj "/C=US/ST=Illinois/L=Chicago/O=Credence/CN=localhost" \
    -key "$HERE/key.pem" \
    -out "$HERE/cert.pem"

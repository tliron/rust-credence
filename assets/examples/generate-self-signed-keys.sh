#/bin/bash
set -e

HERE=$(dirname "$(readlink --canonicalize "$BASH_SOURCE")")

# See: https://unix.stackexchange.com/a/104305

function gen () {
    local DIR=$1
    local HOST=$2

    rm --force "$DIR/private-key.pem" "$DIR/certificates.pem"

    openssl ecparam \
        -genkey \
        -name prime256v1 \
        -out "$DIR/private-key.pem"

    openssl req \
        -new \
        -days 365 \
        -nodes \
        -x509 \
        -subj "/C=US/ST=Illinois/L=Chicago/O=Credence/CN=$HOST" \
        -addext "subjectAltName=DNS:$HOST" \
        -key "$DIR/private-key.pem" \
        -out "$DIR/certificates.pem"
}

gen "$HERE/blog/.credence/key" localhost
gen "$HERE/multi/.credence/keys/site1" site1
gen "$HERE/multi/.credence/keys/site2" site2

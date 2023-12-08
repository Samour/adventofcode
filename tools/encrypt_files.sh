#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

ENCRYPT_PATHS=$(cat "$SCRIPT_DIR/encrypt_paths")
if [ -z "$ENCRYPT_KEY" ]; then
  ENCRYPT_KEY=$(cat "$SCRIPT_DIR/encrypt_key")
fi

DECRYPT_MODE=$([[ "$1" == "decrypt" ]]; echo $?)

for glob in "$ENCRYPT_PATHS"; do
  if [ $DECRYPT_MODE -eq 0 ]; then
    glob="$glob.enc"
  fi

  for file in $(ls $glob); do
    if [ $DECRYPT_MODE -eq 0 ]; then
      file=${file%.enc}
      openssl aes256 -d -in "$file.enc" -out "$file" -k "$ENCRYPT_KEY" -pbkdf2
    else
      openssl aes256 -in "$file" -out "$file.enc" -k "$ENCRYPT_KEY" -pbkdf2
    fi
  done
done

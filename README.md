# Verseghy IAM

## Setup

- Generate a private key: `openssl ecparam -name prime256v1 -genkey -noout | openssl pkcs8 -topk8 -nocrypt -outform DER -out iam-signing.der`

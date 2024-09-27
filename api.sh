docker run --rm -v "${PWD}:/local" -u $(id -u) \
openapitools/openapi-generator-cli:v7.8.0 \
generate \
-g rust-axum \
-i /local/docs/openapi.yaml \
-o /local/server/apis \
--generate-alias-as-model

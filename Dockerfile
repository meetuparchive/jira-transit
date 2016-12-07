FROM scratch
COPY target/x86_64-unknown-linux-musl/release/jira-transit /
# install ca root certificates
# https://curl.haxx.se/docs/caextract.html
# ADD https://curl.haxx.se/ca/cacert.pem /etc/ssl/certs/ca-certificates.crt
ADD ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
CMD ["/jira-transit"]

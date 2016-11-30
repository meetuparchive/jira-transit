FROM scratch
COPY target/x86_64-unknown-linux-musl/release/jira-transit /
CMD ["/jira-transit"]

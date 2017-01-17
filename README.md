# jira-transit [![Build Status](https://travis-ci.org/meetup/jira-transit.svg?branch=master)](https://travis-ci.org/meetup/jira-transit) ![build status](https://img.shields.io/badge/project%20status-wip-yellow.svg) ![](https://img.shields.io/github/tag/meetup/jira-transit.svg)

A Github webhook handler for transitioning Jira issues. Listens on port 4567.

This is a work in progress.

## usage

Intended to be run as a docker app.

Pick a secret, let's call it `YOUR_HOOK_SECRET` that you'll use as a means of verifying
the source of a hook invocation.

```bash
$ make package
$ docker run --rm -it \
   -e RUST_LOG=info \
   -e GITHUB_SECRET=YOUR_HOOK_SECRET \
   -e GITHUB_TOKEN=GITHUB_OAUTH_ACCESS_TOKEN \
   -e JIRA_HOST=YOUR_JIRA_HOST \
   -e JIRA_USERNAME=YOUR_JIRA_BOT_USERNAME \
   -e JIRA_PASSWORD=YOUR_JIRA_BOT_PASSWORD \
   -e SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt \
   -e SSL_CERT_DIR=/etc/ssl/certs \
   meetup/jira-transit:0.1.{tag}
```

To configure this server you'll want to create a new webhook integration using
the following steps.

1) Visit `https://github.com/{owner}/{repo}/settings`
2) Select "Webhooks" tab
3) Click the "Add webhook" tab
4) Select `application/json` as the Content type hooks will be delivered as
5) Set your secret to the one you've chosen above
6) Select event's your server should be notified about

Note:

This docker image is based on `scatch` which doesn't contain ssl required
information about trusted authorizes. A `ca-certificates.crt` is bundled directly
which is sourced from

Meetup 2016
.

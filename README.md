# jira-transit

A github webhook handler for transitioning jira issues. Listens on port 4567.

## usage

Intended to be run as a docker app.

```bash
$ make package
$ docker run --rm -it -e RUST_LOG=info -e GITHUB_SECRET=YOUR_HOOK_SECRET meetup/jira-transit:0.1.{tag}
```

Meetup 2016

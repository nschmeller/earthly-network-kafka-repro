# Repro of `rdkafka` consume failing in GitHub Actions

This repo is meant to test and potentially reproduce the behavior of
`rdkafka` to fail in GitHub Actions when running in Earthly while using
a Redpanda `docker-compose` stack.

To reproduce locally, install Earthly and run:

```sh
earthly -P +test
```

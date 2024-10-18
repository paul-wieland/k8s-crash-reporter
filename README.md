
# K8s Crash Reporter

This service is fully written in rust and can be deployed to your 
kubernetes cluster. There it will observe pods in state ```CrashLoopBackOff```
and report your those via Telegram (Other report channels might be implemented).

<p align="center">
<img src="/assets/architecture.png" height="400" alt=""/>
</p>

## Deployment

## Running Tests

## Development

## Features

- [x] Reporting pods in state ```CrashLoopBackOff``` to telegram channel
- [ ] Running tests with test containers


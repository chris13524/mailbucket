# Development

"Use Rosetta for x86/amd64 emulation on Apple Silicon" must be enabled on Mac Docker Desktop?

## Install Werf

https://werf.io/documentation/v1.2/?usage=localDev&buildBackend=docker

## Install k3d

https://k3d.io/#installation

```bash
brew install k3d
```

## Create k3d cluster

```bash
k3d registry create registry.localhost --port=5001
k3d cluster create mailbucket -p "2525:25@loadbalancer" --registry-use k3d-registry.localhost:5001
```

If you use port 5000 on Mac, it may be in-use and you may need to disable [something](https://stackoverflow.com/a/72369347/3171100).

https://github.com/werf/werf/issues/1940#issuecomment-783346780

## Deploy

```bash
werf converge --dev --follow --repo=k3d-registry.localhost:5001/mailbucket --platform=linux/amd64
```

On Mac you may need to follow [this](https://github.com/werf/werf/issues/1940#issuecomment-886165787) to add k3d-registry.localhost to your hosts file.

## Logs

```bash
kubectl -n mailbucket logs -f deploy/mailbucket
```

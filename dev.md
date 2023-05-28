# Development

## Install Werf

https://werf.io/documentation/v1.2/?usage=localDev&buildBackend=docker

## Install k3d

https://k3d.io/#installation

```bash
brew install k3d
```

## Create k3d cluster

```bash
k3d registry create registry.localhost
k3d cluster create mycluster --registry-use k3d-registry.localhost
```

If port 5000 is in-use on Mac, you may need to disable [something](https://stackoverflow.com/a/72369347/3171100).

https://github.com/werf/werf/issues/1940#issuecomment-783346780

## Deploy

```bash
werf converge --dev --follow --repo=k3d-registry.localhost:<registry-port>/mailbucket
```

On Mac you may need to follow [this](https://github.com/werf/werf/issues/1940#issuecomment-886165787) to add k3d-registry.localhost to your hosts file.

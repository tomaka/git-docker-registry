# Git-docker-registry

This image contains two things:

- The official Docker registry, exposed by **port 5000**.
- A git repository served by a proxy, exposed by **port 80**.

Both the git repository and the registry are accessible anonymously. Therefore, **do not expose
any port**. If you want to expose them, use a reverse proxy like traefik or nginx with HTTP
authentication.

Whenever you push to the git repository, a script will run. It will try to find any `Dockerfile`.

## Usage

```sh
docker service create --name git-docker-registry --mount src=git-docker-repo,dst=/var/git \
    --mount src=git-docker-registry,dst=/var/lib/registry tomaka/git-docker-registry
```

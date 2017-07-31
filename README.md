# Git-docker-registry

This image contains a git repository served by a proxy, exposed by **port 80**.

Note that this git repository is accessible anonymously. Therefore, **do not expose
this port** without using a reverse proxy like traefik or nginx with HTTP authentication.

Whenever you push to the git repository, a script will run. It will try to find any `Dockerfile`
in any of the immediate children directory of the repository, and build each of them.

If you pass the `REGISTRY_URL` environment variable, then the built images will also be pushed
to the given registry.

## Usage

```sh
docker service create --name git-docker-registry --mount src=git-docker-repo,dst=/var/git \
    tomaka/git-docker-registry
```

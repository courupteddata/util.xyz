# util.xyz

Landing page for hosted services (currently CyberChef), built with Dioxus fullstack and deployed to Linode Kubernetes.

## Layout
- `app/` - Dioxus app (see `app/README.md`)
- `Dockerfile` - builds the app and bundles the latest CyberChef release
- `manifests/` - Kubernetes Deployment, Service, Ingress
- `.github/workflows/build-and-deploy.yaml` - CI/CD

## CI/CD
- Push to any branch: build, push to Docker Hub, deploy
  - `main` -> `util.xyz` (namespace `main-app`)
  - other branches -> `main-page-<branch>.dev.k8s.util.xyz` (namespace `preview-<branch>`)
- PR closed / branch deleted: preview namespace removed
- Daily 04:00 UTC: rebuild `main` with the latest CyberChef release, prune unused image tags
- Dependabot opens daily PRs for Rust dependency updates

Image tags: `ci-<commit sha12>-<cyberchef version>`

## Required secrets
- `DOCKER_USERNAME`, `DOCKER_PASSWORD`
- `KUBE_CONFIG`

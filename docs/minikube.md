# Set up rootless minikube

```
minikube config set driver podman
minikube config set rootless true
minikube config set container-runtime containerd
minikube delete
minikube start
```

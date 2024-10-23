
# K8s Crash Reporter

This service is fully written in rust and can be deployed to your 
kubernetes cluster. There it will observe pods in state ```CrashLoopBackOff```
and report you those via Telegram (Other report channels might be implemented).

<p align="center">
<img src="/assets/architecture.png" height="400" alt=""/>
</p>

## Deployment

***1. Prepare Secrets***

- Get your telegram token and chatId of your target telegram chat
- Replace the placeholders in ```deployment/secrets.yaml```

***2. Create Secrets***

Execute the following command to create the secrets:

```
kubectl apply -f deployment/secrets.yaml
```


***2. Create Deployment***

Execute the following command to create the deployment:

```
kubectl apply -f deployment/secrets.yaml
```

***3. Test the Deployment***

In order to test the deployment, you might want to create a crashing pod. 
You might use the following resource, to create a crashing pod in your cluster:

```
apiVersion: v1
kind: Pod
metadata:
  name: crash-loop-pod
spec:
  containers:
  - name: crash-container
    image: busybox
    command: ["sh", "-c", "exit 1"]  # This command forces the container to exit with an error code
    resources:
      requests:
        memory: "64Mi"
        cpu: "250m"
      limits:
        memory: "128Mi"
        cpu: "500m"
  restartPolicy: Always  # This ensures the pod will keep restarting (causing the crash loop)
```

## Development

You can simply run the service executing ```cargo run```. Make sure to provide ```TOKEN```
and ```CHAT_ID``` via environment variable. Also make sure, you are in a valid [Kubernetes context](https://kubernetes.io/docs/reference/kubectl/generated/kubectl_config/kubectl_config_current-context/).

## Features

- [x] Reporting pods in state ```CrashLoopBackOff``` to your telegram channel
- [x] Provide Kubernetes deployment
- [ ] Filter crash reports based on label/namespace
- [ ] Support other report channels (e.g. Teams, E-Mail, ...)
- [ ] Add linter for code formatting and checks


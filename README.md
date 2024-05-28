# Foundry DnD Beyond Rolls Companion

This project is a Rocket-based Rust application that serves as a companion for Foundry VTT and DnD Beyond rolls. The application includes health checks, readiness, and liveness probes, and can be deployed using Kubernetes.

## Quick Start

### Running Locally with Docker Compose

To run the application locally using Docker Compose, use the following command:

```sh
docker-compose up
```

### Deploying to Kubernetes

To deploy the application to a Kubernetes cluster, follow these steps:

1. Navigate to the `k8s` directory:

```sh
cd k8s
```

2. Apply the Kustomize configuration:

```sh
kubectl apply -k .
```

This will deploy the application to your Kubernetes cluster using the specified manifests and Kustomization file.

## Environment Variables

- **ROCKET_MOUNT** (optional): Specifies the Rocket mount point.

## Endpoints

- `/healthz`: Health check endpoint.
- `/readiness`: Readiness probe endpoint.
- `/liveness`: Liveness probe endpoint.
- `/startup`: Startup probe endpoint.
- `/*`: Proxy endpoint to handle authorization with DnD Beyond.

## Contributing

### Prerequisites

- Rust and Cargo
- Docker
- Kubernetes cluster
- kubectl
- Kustomize

### Building the Docker Image

To build the Docker image for the application, run the following command:

```sh
docker build -t mbround18/fvtt-dndbeyond-companion:latest .
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE.md) file for details.

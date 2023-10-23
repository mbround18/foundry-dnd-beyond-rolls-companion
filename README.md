# Foundry VTT - D&D Beyond Roll Integration - Companion

## Disclaimer

[This is a full rewrite of the original repository over here (click this link). It appears as tho the original is not
maintained nor does it have active contributions or GitHub workflows. Original core logic credit goes to the original
creator. <3](https://github.com/rm2kdev/foundry-dnd-beyond-rolls-companion)

## Setup

### Shell

1. Clone the repo
2. `npm install`
3. `npm start`

### Docker

```shell
docker run -p 8745:8745 mbround18/fvtt-dndbeyond-companion
```

### Kubernetes

```shell
# Replace namespace with your own
export NAMESPACE="foundry"

# Add the repo
helm repo add mbround18 https://mbround18.github.io/helm-charts/

# Update helm repos
helm repo update

# Download chart values file
helm show values mbround18/fvtt-dndbeyond-companion > values.yaml

# Edit values.yaml to your liking
eval "${EDITOR:-vi} values.yaml" 

# Create or use an existing namespace
helm -n ${NAMESPACE} install beyond-companion mbround18/fvtt-dndbeyond-companion -f values.yaml

# If you need to update the chart
helm -n ${NAMESPACE} upgrade beyond-companion mbround18/fvtt-dndbeyond-companion -f values.yaml
```

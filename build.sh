#!/bin/bash

# Build the docker images
eval $(minikube docker-env)

./clean.sh

docker build -t faucet images/faucet
docker build -t plumber images/plumber
docker build -t plumbmark plumbmark

# Create the deployment
kubectl apply -f k8s/plumber.yaml
kubectl apply -f k8s/faucet.yaml
kubectl apply -f k8s/plumbmark.yaml


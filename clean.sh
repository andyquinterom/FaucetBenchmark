#!/bin/bash

kubectl delete -f k8s/plumber.yaml
kubectl delete -f k8s/faucet.yaml
kubectl delete -f k8s/plumbmark.yaml

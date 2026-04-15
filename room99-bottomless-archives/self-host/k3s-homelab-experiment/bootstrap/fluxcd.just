bootstrap-fluxcd:
  helm install --create-namespace --namespace flux-system flux-operator oci://ghcr.io/controlplaneio-fluxcd/charts/flux-operator
  kubectl apply -f ../apps/core/deployment/flux-instance.yaml

uninstall-fluxcd:
  kubectl -n flux-system delete fluxinstances --all
  helm -n flux-system uninstall flux-operator
  kubectl delete namespace flux-system


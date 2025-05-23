bootstrap-fluxcd:
  helm install --dependency-update --create-namespace --namespace flux-system flux-operator ./fluxcd/flux-operator
  helm install --dependency-update --namespace flux-system flux-instance ./fluxcd/flux-instance


uninstall-fluxcd:
  kubectl delete all --all -n flux-system
  kubectl delete namespace flux-system


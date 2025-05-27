KEY_NAME := "nodes.reta.re"
KEY_COMMENT := "k3s secrets"
KEY_GENERATION := "
%no-protection
Key-Type: 1
Key-Length: 4096
Subkey-Type: 1
Subkey-Length: 4096
Expire-Date: 0
Name-Comment: " + KEY_COMMENT +"
Name-Real: " + KEY_NAME

bootstrap-sops:
  echo "{{KEY_GENERATION}}" | gpg --batch --full-generate-key
  export KEY_FP="$(gpg --list-secret-keys "nodes.reta.re" | awk 'NR==2 { print $1 }')"
  gpg --export-secret-keys --armor "${KEY_FP}" | kubectl create secret generic sops-gpg \
    --create-namespace \
    --namespace=flux-system \
    --from-file=sops.asc=/dev/stdin

uninstall-sops:
  gpg --delete-secret-keys "$(gpg --list-secret-keys "nodes.reta.re" | awk 'NR==2 { print $1 }'))"
  kubectl delete -n flux-system secrets sops-gp

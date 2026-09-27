#!/bin/bash
set -eu

ssh_port=
POSITIONAL_ARGS=()

while [[ $# -gt 0 ]]; do
  case $1 in
    -p|--port)
      ssh_port="$2"
      shift # past argument
      shift # past value
      ;;
    -*|--*)
      echo "Unknown option $1"
      exit 1
      ;;
    *)
      POSITIONAL_ARGS+=("$1") # save positional arg
      shift # past argument
      ;;
  esac
done

set -- "${POSITIONAL_ARGS[@]}" # restore positional parameters

image="$1"
target="$2"
oci="/dev/null"
if command -v podman >/dev/null; then oci="podman"; 
elif command -v docker >/dev/null; then oci="docker";
fi

target_ssh="$target"
if [ ! -z "$ssh_port" ]; then target_ssh="$target_ssh -p $ssh_port"; fi
remote_oci="$(ssh $target_ssh sh -c 'false || which podman >/dev/null 2>&1 && echo podman || (which docker >/dev/null 2>&1 && echo docker || echo /dev/null)')"

if command -v pv >/dev/null; then
  "$oci" save "$image" |\
    pv --size "$("$oci" image inspect "$image" | jq '.[0].Size')" |\
    ssh $target_ssh "$remote_oci" load
else
  "$oci" save "$image" |\
    ssh $target_ssh "$remote_oci" load
fi

#!/usr/bin/env bash

set -e -o pipefail

url=
rev=
expHash=

out="${out:-}"
tmpPath=
finalPath=
hash=

usage(){
  # TODO: Adapt
  echo 1>&2 "syntax: nix-prefetch-flathub [options] [URL [REVISION [EXPECTED-HASH]]]
Options:
      --out path      Path where the output would be stored.
      --url url       Any url understood by 'git clone'.
      --rev ref       Any sha1 or references (such as refs/heads/master)
      --quiet         Only print the final json summary.
"
  exit 1
}

clean_ostree() {
  ostree "$@" 1>&2
}

argi=0
argfun=""
for arg; do
  if test -z "$argfun"; then
    case $arg in
      --out) argfun=set_out;;
      --url) argfun=set_url;;
      --rev) argfun=set_rev;;
      --name) argfun=set_name;;
      --quiet) QUIET=true;;
      -h|--help) usage; exit;;
      *)
        : $((++argi))
        case $argi in
          1) url=$arg;;
          2) rev=$arg;;
          3) expHash=$arg;;
          *) exit 1;;
        esac
        ;;
    esac
  else
    case $argfun in
      set_*)
        var=${argfun#set_}
        eval "$var=$(printf %q "$arg")"
        ;;
    esac
    argfun=""
  fi
done

name="$(echo "$url" | sed 's/[^.]*\.//' | sed 's/\./-/g' | tr '[:upper:]' '[:lower:]')"

init_remote(){
  local outdir="$1"
  mkdir -p "$outdir/ostree_source"
  mkdir -p "$outdir/cache"
  clean_ostree init --repo "$outdir/ostree_source" --mode=bare-user-only
  clean_ostree remote add --no-gpg-verify --repo "$outdir/ostree_source" flathub https://dl.flathub.org/repo/
}

pull() {
  local url="$1"
  local outdir="$2"
  local lrev="$3"
  if test -n "$lrev"; then ref="@$lrev"; fi

  local ref="app/$url/x86_64/stable$ref"

  # Init repo
  init_remote "$outdir"

  clean_ostree pull --repo "$outdir/ostree_source" --cache-dir "$outdir/cache" flathub "$ref"
  if test -z "$lrev"; then
    local rev_info
    rev_info="$(ostree show --raw --repo "$outdir/ostree_source" "$ref")"
    lrev="$(echo "$rev_info" | grep -oP "commit [a-z0-9]+" | sed 's/commit //')"
  fi
  clean_ostree export -v --repo "$outdir/ostree_source" flathub:"$lrev" --no-xattrs --subpath files --output "$outdir/$name.tar"
  
  rev="$lrev"
}

exit_handlers=()

run_exit_handlers() {
    exit_status=$?
    for handler in "${exit_handlers[@]}"; do
        eval "$handler $exit_status"
    done
}

trap run_exit_handlers EXIT

quiet_exit_handler() {
    exec 2>&3 3>&-
    if [ "$1" -ne 0 ]; then
        cat "$errfile" >&2
    fi
    rm -f "$errfile"
}

quiet_mode() {
    errfile="$(mktemp "${TMPDIR:-/tmp}/git-checkout-err-XXXXXXXX")"
    exit_handlers+=(quiet_exit_handler)
    exec 3>&2 2>"$errfile"
}

json_escape() {
    local s="$1"
    s="${s//\\/\\\\}" # \
    s="${s//\"/\\\"}" # "
    s="${s//^H/\\\b}" # \b (backspace)
    s="${s//^L/\\\f}" # \f (form feed)
    s="${s//
/\\\n}" # \n (newline)
    s="${s//^M/\\\r}" # \r (carriage return)
    s="${s//   /\\t}" # \t (tab)
    echo "$s"
}

remove_tmpPath() {
    # rm -rf "$tmpPath"
    echo 
}

if test -n "$QUIET"; then
    quiet_mode
fi

tmpPath="$(mktemp -d "${TMP:-/tmp}/ostree-flathub-tmp-XXXXXXXX")"
exit_handlers+=(remove_tmpPath)
mkdir -p "$tmpPath/nix"

# if test -n "$expHash"; then
#   finalPath="$(nix-store --print-fixed-path "sha256" "$expHash" "$name")"
#   if ! nix-store --check-validity "$finalPath" 2>/dev/null; then
#     finalPath=
#   fi
#   hash="$expHash"

if test -z "$finalPath"; then

  pull "$url" "$tmpPath" "$rev"
  outfile="$tmpPath/$name.tar"
  finalPath="$out"
  cp "$outfile" "$finalPath"

  # hash="$(nix-hash --type sha256 --base32 "$outfile")"
  #
  # finalPath="$(nix-store --add-fixed "sha256" "$outfile")"
  #
  # if test -n "$expHash" -a "$expHash" != "$hash"; then
  #   echo 1>&2 "hash mismatch for URL \`$url'. Got \`$hash'; expected \`$expHash'."
  #   exit 1
  # fi
fi

if test -n "$hash"; then
  cat <<EOF
{
  "url": "$(json_escape "$url")",
  "rev": "$(json_escape "$rev")",
  "path": "$(json_escape "$finalPath")",
  "sha256": "$(json_escape "$hash")",
}
EOF
fi

if test -n "$PRINT_PATH"; then
  echo "$finalPath"
fi

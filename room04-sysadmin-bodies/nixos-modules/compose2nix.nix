{ config, lib, pkgs, ... }:

with lib;
let
  ocfg = config.virtualisation.oci-containers;
  prefix = "${ocfg.backend}-";
  networkSvcName = name: "${prefix}network-${name}";

  rootlessContainers = lib.filterAttrs (_: c: c.rootless != null && c.rootless.user != "root") ocfg.containers;

  uidOf = user: (config.users.users.${user} or { }).uid or null;

  dockerHostFor = c: "unix:///run/user/${toString (uidOf c.rootless.user)}/docker.sock";

  networkFlags = [ "--network" "--net" ];
  # Modes that never correspond to a user-defined network service.
  specialNetworkModes = [ "host" "none" "bridge" "container" "slirp4netns" "pasta" ];

  # Values of flags passed as a separate list item, e.g. [ "--network" "foo" ].
  valuesAfterFlags = flags: options:
    let
      go = l:
        if builtins.length l < 2 then [ ]
        else if lib.elem (builtins.head l) flags
        then [ (builtins.elemAt l 1) ] ++ go (builtins.drop 2 l)
        else go (builtins.tail l);
    in
    go options;

  # Every network value mentioned in `extraOptions`, in both
  # "--network=foo" and "--network foo" forms.
  networkValuesOf = container:
    lib.concatMap (opt:
      let m = builtins.match "--(network|net)=(.+)" opt;
      in if m != null then [ (lib.elemAt m 1) ] else [ ])
      container.extraOptions
    ++ valuesAfterFlags networkFlags container.extraOptions;

  # User-defined network names; drops built-in modes and anything
  # after a ":" (e.g. "container:foo", "mynet:alias").
  networkNamesOf = container:
    lib.unique (lib.filter (n: !lib.elem n specialNetworkModes)
      (map (v: lib.head (lib.splitString ":" v))
        (networkValuesOf container)));

  defaultNetworkRegex = "(.+)_default";

  # The project name of a container, or null when it declares no
  # "<project>_default" network in `extraOptions`.
  projectNameOf = container:
    let
      projects = lib.concatMap (n:
        let m = builtins.match defaultNetworkRegex n;
        in if m != null then m else [ ])
        (networkNamesOf container);
    in
    if projects == [ ] then null else lib.head projects;

  # "<source>:<destination>[:opts]": a *named* volume is one whose source
  # is a bare volume name — i.e. not a bind-mount path ("/...", "./...", "~...")
  # and not an anonymous volume (empty source).
  namedVolumeRegex = "[A-Za-z0-9][A-Za-z0-9_.-]*:.*";
  volumeNamesOf = container:
    lib.unique (lib.concatMap (v:
      if builtins.match namedVolumeRegex v != null
      then [ (lib.head (lib.splitString ":" v)) ]
      else [ ])
      container.volumes);

  # The service managing compose volume "<name>" of project "<project>" is
  #   ${backend}-volume-<project>_<name>
  # Falls back to the bare "<name>" when no project could be determined,
  # and never double-prefixes an already-qualified name.
  volumeSvcName = projectName: name:
    let
      qualified =
        if projectName == null || lib.hasPrefix "${projectName}_" name
        then name
        else "${projectName}_${name}";
    in
    "${prefix}volume-${qualified}";

  # "<backend>-network-<name>" / "<backend>-volume-<name>" service name
  # -> rootless settings of the rootless container that uses it.
  # NOTE: a network/volume shared by several rootless containers keeps the
  # settings of the last container processed.
  rootlessDepServices = lib.foldl' lib.mergeAttrs { } (lib.mapAttrsToList (_: container:
    let
      user = container.rootless.user;
      dockerHost = dockerHostFor container;
      projectName = projectNameOf container;
      perService = svc: lib.nameValuePair svc { inherit user dockerHost; };
    in
    builtins.listToAttrs (
      map perService (
        map networkSvcName (networkNamesOf container)
        ++ map (volumeSvcName projectName) (volumeNamesOf container)
      ))
  ) rootlessContainers);
in
{
  config = lib.mkIf (ocfg.containers != { } && ocfg.backend == "docker") {
    systemd.services = lib.mapAttrs (_: cfg: {
      serviceConfig.User = lib.mkDefault cfg.user;
      environment.DOCKER_HOST = lib.mkDefault cfg.dockerHost;
      after = [ "user@${toString (uidOf cfg.user)}.service" "linger-users.service" "docker.service" "docker.socket" ];
      requires = [ "user@${toString (uidOf cfg.user)}.service" "linger-users.service" "docker.service" "docker.socket" ];
    }) rootlessDepServices;
  };
}

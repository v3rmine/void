{ modulesPath, ... }: {
  imports = [ (modulesPath + "/virtualisation/proxmox-lxc.nix") ];
  nix.settings = { sandbox = false; };
  proxmoxLXC = {
    manageNetwork = false;
    privileged = false;
  };

  services.tailscale = {
    enable = true;
    authKeyFile = "/run/secrets/tailscale_key";
    useRoutingFeatures = "server";
    extraUpFlags =
      [ "--accept-dns=false" "--advertise-routes=10.0.0.0/16" "--ssh" ];
  };

  security.pam.services.sshd.allowNullPassword = true;
  services.openssh = {
    enable = true;
    authorizedKeysFiles = [ "/etc/sshd/authorized_keys" ];
    settings = {
      PermitRootLogin = "yes";
      PasswordAuthentication = false;
      PermitEmptyPasswords = "yes";
    };
  };
  system.stateVersion = "25.11";

  time.timeZone = "Europe/Paris";
}

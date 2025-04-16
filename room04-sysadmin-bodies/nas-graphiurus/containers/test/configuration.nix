{ modulesPath, ... }: {
  imports = [ (modulesPath + "/virtualisation/proxmox-lxc.nix") ];
  nix.settings = { sandbox = false; };
  proxmoxLXC = {
    manageNetwork = false;
    privileged = false;
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
  system.stateVersion = "24.11";

  time.timeZone = "Europe/Paris";
}

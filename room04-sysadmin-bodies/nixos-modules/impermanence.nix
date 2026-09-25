{ ... }: {
  imports = [];
  options = {};

  config = {
    environment.persistence."/persist" = {
      files = [
        # Machine identifier
        "/etc/machine-id"
        # ssh host keys
        "/etc/ssh/ssh_host_ed25519_key"
        "/etc/ssh/ssh_host_ed25519_key.pub"
        "/etc/ssh/ssh_host_rsa_key"
        "/etc/ssh/ssh_host_rsa_key.pub"
      ];
      directories = [
        "/boot"
        "/nix"
        "/tmp"
        "/var/tmp"
        "/var/lib/nixos"
        "/var/log"
        "/var/lib/systemd/system"
      ];
    };
  };
}

{ ... }: {
  security.pam.services.sshd.allowNullPassword = true;
  services.openssh = {
    enable = true;
    settings = {
      PermitRootLogin = "yes";
      PasswordAuthentication = true;
      PermitEmptyPasswords = "yes";
    };
  };
  system.stateVersion = "24.11";

  time.timeZone = "Europe/Paris";
}

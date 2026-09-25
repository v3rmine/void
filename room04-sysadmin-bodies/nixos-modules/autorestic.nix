{ pkgs, config, ... }:
let
  run-autorestic = pkgs.writeShellScriptBin "run-autorestic.sh" ''
    # Merge location and backend autorestic confs
    autorestic_conf="$(${pkgs.yq-go}/bin/yq eval-all '. as $item ireduce ({}; . * $item)' /etc/autorestic.yml /etc/autorestic-backends.yml | sed 's/"<<"/<</')"
    echo "$autorestic_conf" > /tmp/.autorestic.yml

    # Run the cron task of autorestic
    ${pkgs.autorestic}/bin/autorestic -c /tmp/.autorestic.yml --ci cron > /var/log/autorestic.log 2>&1
    rm -f /tmp/.autorestic.yml
  '';
  
  prune-autorestic = pkgs.writeShellScriptBin "prune-autorestic.sh" ''
    # Merge location and backend autorestic confs
    autorestic_conf="$(${pkgs.yq-go}/bin/yq eval-all '. as $item ireduce ({}; . * $item)' /etc/autorestic.yml /etc/autorestic-backends.yml | sed 's/"<<"/<</')"
    echo "$autorestic_conf" > /tmp/.autorestic-prune.yml

    sed -i -E 's/(^.*run-autorestic.*$)/#\1/' /etc/crontab

    ${pkgs.autorestic}/bin/autorestic -v -c /tmp/.autorestic-prune.yml forget --all > /var/log/autorestic-prune.log 2>&1
    ${pkgs.autorestic}/bin/autorestic -v -c /tmp/.autorestic-prune.yml exec --all -- prune >> /var/log/autorestic-prune.log 2>&1

    rm -f /tmp/.autorestic-prune.yml
    sed -i -E 's/^#(.*run-autorestic.*$)/\1/' /etc/crontab
  '';
in {
  imports = [];
  options = {};

  config = {
    environment.persistence."/persist" = {
      files = [
        "/etc/autorestic-backends.yml"
      ];
      directories = [
        "/root/.cache/restic"
      ];
    };

    environment.systemPackages = with pkgs; [
      restic
      autorestic
      run-autorestic
      prune-autorestic
    ];

    services.cron.systemCronJobs = [
      "*/5 * * * * root ${run-autorestic}/bin/run-autorestic.sh"
      "3 1 * * * root ${prune-autorestic}/bin/prune-autorestic.sh"
    ];
  };
}

terraform {
    backend "local" {
        path = "../../.secrets/terraform/.state/oci"
    }
}

module "free1-dasyprocta" {
    source = "../../room05-sysadmin-souls/terraform-but-its-actually-eatable/modules/oci"

    fingerprint = var.fingerprint
    private_key_path = var.private_key_path
    region = var.region
    tenancy_ocid = var.tenancy_ocid
    user_ocid = var.user_ocid
    compartment_ocid = var.compartment_ocid

    instance_name = "dasyprocta"
    instance_shape = "VM.Standard.A1.Flex"
    instance_ocpus = 4              # Free tier is 4CPU (3,000 OCPU hours)
    instance_memory_in_gbs = 24     # Free tier is 24GB (18,000 GB hours)
    boot_volume_size_in_gbs = 100   # Free tier is 200GB (Up to 2 block volumes)
    # eu-marseille-1 | Canonical-Ubuntu-24.04-Minimal-aarch64-2024.10.08-0
    # eu-paris-1     | Canonical-Ubuntu-24.04-Minimal-aarch64-2025.01.31-1
    # See https://docs.oracle.com/en-us/iaas/images/ for more information
    instance_image_ocid = "ocid1.image.oc1.eu-paris-1.aaaaaaaal7gsuv6pdtalp3zjhufyat46kbzmooy7rpxi3nh4js2hcjmezrha"

    instance_state = "RUNNING"
    ssh_public_key = var.ssh_public_key
    instance_inital_cmds = [
        "apt-get update --quiet --assume-yes",
        "apt-get remove --quiet --assume-yes --purge apparmor"
    ]
}

terraform {
  required_providers {
    incus = {
      source = "lxc/incus"
    }
  }
}

resource "incus_instance" "nixos" {
  name  = "nixos"
  image = "images:nixos/unstable"
  config = {
    "boot.autostart"   = true
    "security.nesting" = true
  }
}


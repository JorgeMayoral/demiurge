declare global {
  type Hostname = string;
  interface System {
    hostname: Hostname;
  }

  type Pkgs = string[];
  interface Packages {
    paru: Pkgs;
  }

  type Dotfiles = Dotfile[];
  interface Dotfile {
    source: string;
    target: string;
  }

  type Services = Service[];
  type Service = string;

  type Users = User[];
  interface User {
    name: string;
    groups: string[];
  }

  interface DemiurgeConfig {
    system: System;
    packages: Packages;
    dotfiles: Dotfiles;
    services: Services;
    users: Users;
  }

  type Demiurge = { [key: string]: DemiurgeConfig };

  type DemiurgFn = () => Demiurge;
}

export {};

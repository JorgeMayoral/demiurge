declare global {
  type Hostname = string;

  interface System {
    hostname: Hostname;
  }

  type Pkgs = string[];

  interface Packages {
    paru: Pkgs;
  }

  interface Dotfile {
    source: string;
    target: string;
  }

  interface DemiurgeConfig {
    system: System;
    packages: Packages;
    dotfiles: Dotfile[];
  }

  type Demiurge = { [key: string]: DemiurgeConfig };

  type DemiurgFn = () => Demiurge;
}

export {};

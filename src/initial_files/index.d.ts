declare global {
  type Hostname = string;

  interface System {
    hostname: Hostname;
  }

  type Pkgs = string[];

  interface Packages {
    paru: Pkgs;
  }

  interface DemiurgeConfig {
    system: System;
    packages: Packages;
  }

  type Demiurge = { [key: string]: DemiurgeConfig };

  type DemiurgFn = () => Demiurge;
}

export {};

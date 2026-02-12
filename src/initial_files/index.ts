const config: DemiurgeConfig = {
  system: {
    hostname: "my-system",
  },
  packages: {
    paru: ["git", "curl"],
  },
  dotfiles: [],
  services: [],
};

export default (): Demiurge => ({
  "my-config": config,
});

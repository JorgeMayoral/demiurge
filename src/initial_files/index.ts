const config: DemiurgeConfig = {
  system: {
    hostname: "my-system",
  },
  users: [{ name: "me", groups: [] }],
  packages: {
    paru: ["git", "curl"],
  },
  dotfiles: [],
  services: [],
};

export default (): Demiurge => ({
  "my-config": config,
});

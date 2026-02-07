const config: DemiurgeConfig = {
  system: {
    hostname: "my-system",
  },
  packages: {
    paru: ["git", "curl"],
  },
};

export default (): Demiurge => ({
  "my-config": config,
});

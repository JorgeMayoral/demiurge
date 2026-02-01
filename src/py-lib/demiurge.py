from dataclasses import dataclass, field


@dataclass
class SystemState:
    hostname: str | None = None


@dataclass
class PackageState:
    pacman: list[str] = field(default_factory=list)
    aur: list[str] = field(default_factory=list)


@dataclass
class GlobalState:
    system: SystemState = field(default_factory=SystemState)
    packages: PackageState = field(default_factory=PackageState)

    def get_state(self):
        return self


STATE = GlobalState()


class Packages:
    @staticmethod
    def pacman(*pkgs: str) -> None:
        for pkg in pkgs:
            if pkg not in STATE.packages.pacman:
                STATE.packages.pacman.append(pkg)

    @staticmethod
    def aur(*pkgs: str) -> None:
        for pkg in pkgs:
            if pkg not in STATE.packages.aur:
                STATE.packages.aur.append(pkg)


class System:
    @staticmethod
    def hostname(name: str) -> None:
        if STATE.system.hostname is not None:
            raise RuntimeError("Hostname already set")
        STATE.system.hostname = name

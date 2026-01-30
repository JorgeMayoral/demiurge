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


STATE = GlobalState()

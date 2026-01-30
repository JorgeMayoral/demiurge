# dsl/packages.py
from ..state import STATE


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

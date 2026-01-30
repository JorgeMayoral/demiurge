# demiurge.pyi
# This file is ONLY for type checking and editor support.
# Runtime implementation is provided by the demiurge binary.

from typing import Iterable

class System:
    @staticmethod
    def hostname(name: str) -> None: ...
    @staticmethod
    def timezone(tz: str) -> None: ...
    @staticmethod
    def locale(locale: str) -> None: ...

class Packages:
    @staticmethod
    def pacman(packages: Iterable[str]) -> None: ...
    @staticmethod
    def aur(packages: Iterable[str]) -> None: ...

class Services:
    @staticmethod
    def enable(name: str) -> None: ...
    @staticmethod
    def disable(name: str) -> None: ...

class Users:
    @staticmethod
    def create(
        name: str,
        *,
        groups: Iterable[str] | None = None,
        shell: str | None = None,
    ) -> None: ...

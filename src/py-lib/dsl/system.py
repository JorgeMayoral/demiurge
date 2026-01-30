from ..state import STATE


class System:
    @staticmethod
    def hostname(name: str) -> None:
        if STATE.system.hostname is not None:
            raise RuntimeError("Hostname already set")
        STATE.system.hostname = name

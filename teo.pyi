from typing import Optional, Awaitable

class App:
    """Teo application."""

    async def run(self) -> None:
        """Start the Teo application server."""
        ...

class AppBuilder:

    def __init__(self) -> None:
        """Create an Teo application builder."""
        ...

    def load(self, schema_file_name: Optional[str]) -> None:
        """Load a schema file into application builder."""
        ...

    async def build(self) -> App:
        """Build the Teo application."""
        ...

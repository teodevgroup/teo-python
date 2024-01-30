"""This module contains classes and variables for Teo web framework."""
from __future__ import annotations
from typing import TypeVar, Union

T = TypeVar('T')

Enumerable = Union[T, list[T]]

class App:

    def __init__(self) -> None:
        """
        Create a new application. Only one can be presented in a program.
        """

    def main_namespace(self) -> Namespace:
        """
        Get the attached main namespace of the app.
        """
        pass

    async def run(self):
        """
        Run the application.
        """


class Namespace:
    """
    Namespace is where handlers and models are defined.
    """

    def define_handler(self):
        pass
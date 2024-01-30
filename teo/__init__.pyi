class App:

    def __init__(self):
        """
        Create a new app. Only one can be present in a program.
        """

    def main_namespace(self) -> Namespace:
        """
        Get the attached main namespace of the app.
        """
        pass

    async def run(self):
        """
        Start the app.
        """


class Namespace:
    """
    Namespace is where handlers and models are defined.
    """

    def define_handler(self):
        pass
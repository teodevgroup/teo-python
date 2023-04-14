from .teo import App, fetch_model_class
from signal import signal, SIGINT
from sys import exit


signal(SIGINT, lambda _, __: exit(0))

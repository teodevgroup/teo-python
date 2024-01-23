from .teo import App
from signal import signal, SIGINT
from sys import exit


signal(SIGINT, lambda _, __: exit(0))

from abc import ABC


class Window(ABC):
    def __init__(self, window_size, FPS, background_color):
        self.window_size = window_size
        self.FPS = FPS
        self.background_color = background_color

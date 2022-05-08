import pygame


class TextLabel:
    font_size = 30
    padding = 10
    margin = 10
    color = pygame.Color("white")
    highlight_color = pygame.Color("pink")

    def __init__(self, text=None, color=None, coordinates=None, font_size=None):
        self._color = color if color is not None else TextLabel.color
        self._font_size = font_size if font_size is not None else TextLabel.font_size
        self._text = text
        self._coordinates = coordinates
        self._font = pygame.font.Font(None, self._font_size)
        self._text_surface = self._font.render(self._text, True, self._color)

    def draw(self, screen, x=None, y=None):
        coordinates = (x, y) if x is not None and y is not None else self._coordinates
        screen.blit(self._text_surface, coordinates)

    def update_color(self, color):
        self._color = color
        self._text_surface = self._font.render(self._text, True, color)

    def update_text(self, text):
        self._text = text
        self._text_surface = self._font.render(text, True, self._color)

    def get_width(self):
        return self._text_surface.get_width()

    def get_height(self):
        return self._text_surface.get_height()

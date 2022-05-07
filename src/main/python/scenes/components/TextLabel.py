import pygame


class TextLabel:
    font_size = 30
    padding = 10
    margin = 10
    color = pygame.Color("white")
    highlight_color = pygame.Color("pink")

    def __init__(self, text=None, color=None, coordinates=None):
        self.color = color if color is not None else TextLabel.color
        self.text = text
        self.coordinates = coordinates
        self.font = pygame.font.Font(None, TextLabel.font_size)
        self.text_surface = self.font.render(self.text, True, self.color)

    def draw(self, screen, x=None, y=None):
        coordinates = (x, y) if x is not None and y is not None else self.coordinates
        screen.blit(self.text_surface, coordinates)

    def update_color(self, color):
        self.color = color
        self.text_surface = self.font.render(self.text, True, color)

    def update_text(self, text):
        self.text = text
        self.text_surface = self.font.render(text, True, self.color)

    def get_size(self):
        return self.text_surface.get_width(), self.text_surface.get_height()

    def get_width(self):
        return self.text_surface.get_width()

    def get_height(self):
        return self.text_surface.get_height()

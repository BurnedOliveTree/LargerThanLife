import pygame


class TextLabel:
    font_size = 30
    color = pygame.Color("white")
    
    def __init__(self, text = None):
        self.color = TextLabel.color
        self.text = text
        self.font = pygame.font.Font(None, TextLabel.font_size)
        self.text_surface = self.font.render(self.text, True, self.color)

    def draw(self, screen, x, y):
        screen.blit(self.text_surface, (x, y))
    
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
    


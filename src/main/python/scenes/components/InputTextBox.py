from scenes.components.Component import Component, TextLabel
import pygame


class InputTextBox(Component):
    default_width = 200
    
    def __init__(
        self, coordinates, active_color, passive_color, description
    ):
        super().__init__("", coordinates, active_color, passive_color)
        self.description_label = TextLabel(description)

    def draw(self, screen):
        self.description_label.draw(screen, self.coordinates[0] - self.description_label.get_width(),self.coordinates[1])
        self.text_label.update_text(self.text)
        self.rect.w = max(self.text_label.get_width() + Component.padding, InputTextBox.default_width)

        self.change_color()
        pygame.draw.rect(screen, self.color, self.rect, Component.border_width)
        self.text_label.draw(screen, self.rect.x + Component.padding, self.rect.y + Component.padding)

    def get_text_after_event(self, event):
        if event.key == pygame.K_BACKSPACE:
            self.text = self.text[:-1]
        else:
            self.text += event.unicode

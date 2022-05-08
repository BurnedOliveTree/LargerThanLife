from scenes.components.Component import Component, TextLabel
import pygame


class InputTextBox(Component):
    default_width = 200

    def __init__(self, coordinates, active_color, passive_color, description):
        super().__init__("", coordinates, active_color, passive_color)
        self._description_label = TextLabel(description)

    def draw(self, screen):
        self._description_label.draw(
            screen,
            self._coordinates[0] - self._description_label.get_width(),
            self._coordinates[1],
        )
        self._text_label.update_text(self._text)
        self._rect.w = max(
            self._text_label.get_width() + Component.padding, InputTextBox.default_width
        )

        self.change_color()
        pygame.draw.rect(screen, self._color, self._rect, Component.border_width)
        self._text_label.draw(
            screen, self._rect.x + Component.padding, self._rect.y + Component.padding
        )

    def get_text(self):
        return self._text

    def set_text_after_event(self, event):
        if event.key == pygame.K_BACKSPACE:
            self._text = self._text[:-1]
        else:
            self._text += event.unicode

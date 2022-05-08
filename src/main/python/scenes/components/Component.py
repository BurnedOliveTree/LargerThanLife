import pygame

from scenes.components.TextLabel import TextLabel


class Component:
    border_radius = 5
    border_width = 5
    default_width = 200
    padding = 5

    def __init__(
        self, text, coordinates, active_color, passive_color, width=default_width
    ):
        self._text = text
        self._coordinates = coordinates
        self.is_active = False

        self._active_color = active_color
        self._passive_color = passive_color
        self._color = self._passive_color

        self._text_label = TextLabel(text)

        self._rect = pygame.Rect(
            coordinates[0] - Component.padding,
            coordinates[1] - Component.padding,
            self.adjust_width(width),
            self._text_label.get_height() + Component.padding * 2,
        )

    def adjust_width(self, dimension):
        if dimension is None:
            return self._text_label.get_width() + Component.padding * 2
        else:
            return dimension + Component.padding * 2

    def draw(self, screen):
        pygame.draw.rect(
            screen, self._color, self._rect, border_radius=Component.border_radius
        )
        self._text_label.draw(screen, self._coordinates[0], self._coordinates[1])

    def set_status(self, position):
        if self._rect.collidepoint(position):
            self.is_active = True
        else:
            self.is_active = False

    def change_color(self):
        if self.is_active:
            self._color = self._active_color
        else:
            self._color = self._passive_color

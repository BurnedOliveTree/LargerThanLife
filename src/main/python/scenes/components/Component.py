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
        self.text = text
        self.coordinates = coordinates
        self.is_active = False

        self.active_color = active_color
        self.passive_color = passive_color
        self.color = self.passive_color

        self.text_label = TextLabel(text)

        self.rect = pygame.Rect(
            coordinates[0] - Component.padding,
            coordinates[1] - Component.padding,
            self.adjust_width(width),
            self.text_label.get_height() + Component.padding * 2,
        )

    def adjust_width(self, dimesion):
        if dimesion is None:
            return self.text_label.get_width() + Component.padding * 2
        else:
            return dimesion + Component.padding * 2

    def draw(self, screen):
        pygame.draw.rect(
            screen, self.color, self.rect, border_radius=Component.border_radius
        )
        self.text_label.draw(screen, self.coordinates[0], self.coordinates[1])

    def set_status(self, position):
        if self.rect.collidepoint(position):
            self.is_active = True
        else:
            self.is_active = False

    def change_color(self):
        if self.is_active:
            self.color = self.active_color
        else:
            self.color = self.passive_color

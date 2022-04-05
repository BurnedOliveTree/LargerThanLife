import pygame


class Component:
    border_radius = 5
    border_width = 5
    font_size = 30
    text_color = pygame.Color("white")
    padding = 5

    def __init__(
        self,
        text,
        width,
        height,
        coordinates,
        active_color,
        passive_color,
    ):
        self.text = text
        self.coordinates = coordinates
        self.is_active = False

        self.active_color = active_color
        self.passive_color = passive_color
        self.color = self.passive_color

        self.font = pygame.font.Font(None, Component.font_size)
        self.text_surface = self.font.render(self.text, True, Component.text_color)

        self.width = self.adjust(width, self.text_surface.get_width())
        self.height = self.adjust(height, self.text_surface.get_height())

        self.rect = pygame.Rect(
            coordinates[0] - Component.padding,
            coordinates[1] - Component.padding,
            self.width,
            self.height,
        )
        self.rect.topleft

    def adjust(self, dimesion, surface_dimension):
        if dimesion is None:
            return surface_dimension + Component.padding * 2
        else:
            return dimesion + Component.padding * 2

    def draw(self, screen):
        self.change_color()
        pygame.draw.rect(
            screen, self.color, self.rect, border_radius=Component.border_radius
        )
        screen.blit(self.text_surface, (self.coordinates[0], self.coordinates[1]))

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

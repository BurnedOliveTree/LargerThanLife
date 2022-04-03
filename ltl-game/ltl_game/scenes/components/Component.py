import pygame


class Component:
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
        self.width = width
        self.height = height
        self.coordinates = coordinates
        self.is_active = False
        self.active_color = active_color
        self.passive_color = passive_color
        self.color = self.passive_color

        self.font = pygame.font.Font(None, 30)
        self.text_surface = self.font.render(self.text, True, pygame.Color("white"))
        
        self.rect = pygame.Rect(coordinates[0], coordinates[1], width, height)
        self.rect.w = max(self.text_surface.get_width() * 3 // 2, 100)

    def draw(self, screen):
        self.change_color()
        pygame.draw.rect(screen, self.color, self.rect, border_radius=10)
        screen.blit(self.text_surface, (self.coordinates[0], self.coordinates[1]))

    def set_status(self, position):
        if self.rect.collidepoint(position):
            self.is_active = True
        else:
            self.is_active = False
            return None

    def change_color(self):
        if self.is_active:
            self.color = self.active_color
        else:
            self.color = self.passive_color

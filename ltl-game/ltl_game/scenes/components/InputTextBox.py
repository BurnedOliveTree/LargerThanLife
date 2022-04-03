from scenes.components.Component import Component
import pygame


class InputTextBox(Component):
    input_rect_height = 40

    def __init__(
        self, width, height, coordinates, active_color, passive_color, description
    ):
        super().__init__("", width, height, coordinates, active_color, passive_color)
        self.description = description

    def draw_description(self, screen):
        description_surface = self.font.render(
            self.description, True, pygame.Color("white")
        )
        description_width = description_surface.get_width()
        screen.blit(
            description_surface,
            (
                self.coordinates[0] - description_width,
                self.coordinates[1] - (self.input_rect_height * 3 // 4),
            ),
        )

    def draw(self, screen):
        self.draw_description(screen)
        text_surface = self.font.render(self.text, True, (255, 255, 255))
        self.rect.w = max(text_surface.get_width() + 20, 100)

        self.change_color()
        pygame.draw.rect(screen, self.color, self.rect, 5)
        screen.blit(text_surface, (self.rect.x + 10, self.rect.y + 10))

    def get_text_after_event(self, event):
        if event.key == pygame.K_BACKSPACE:
            self.text = self.text[:-1]
        else:
            self.text += event.unicode

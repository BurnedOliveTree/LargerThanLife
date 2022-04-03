import pygame
import numpy as np
from scenes import Window


class Game(Window):
    def get_surface_from_bitmap(self, bitmap):
        scaled_color_bitmap = 255 * bitmap
        bitmap_surface = pygame.surfarray.make_surface(scaled_color_bitmap)
        scaled_bitmap_surface = pygame.transform.scale(
            bitmap_surface, (self.window_size, self.window_size)
        )
        return scaled_bitmap_surface

    def render(self, screen, clock):
        while True:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    return None

            bitmap = np.round(
                np.random.random((self.window_size // 4, self.window_size // 4))
            )  # to check if scaling works
            background = self.get_surface_from_bitmap(bitmap)
            screen.blit(background, (0, 0))

            pygame.display.update()
            clock.tick(self.FPS)

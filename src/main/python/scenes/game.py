import pygame
import numpy as np
from scenes import Window
import rust


class Game(Window):
    def __init__(self, window_size, FPS):
        super().__init__(window_size, FPS)
        self.rust_game = rust.Game(2, 2, 2, 2, rust.Neighbourhood.Moore)

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

            pre_bitmap = self.rust_game.generate_image(self.window_size)
            bitmap = np.array([np.array(xi) for xi in pre_bitmap])
            background = self.get_surface_from_bitmap(bitmap)
            screen.blit(background, (0, 0))

            pygame.display.update()
            clock.tick(self.FPS)

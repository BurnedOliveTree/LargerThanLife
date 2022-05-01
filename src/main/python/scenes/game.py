import pygame
import numpy as np
from scenes import Window
from rust import Engine, Rules


class Game(Window):
    def __init__(self, window_size, FPS, board_size=None):
        super().__init__(window_size, FPS)
        self.board_size = board_size if board_size is not None else window_size
        self.engine = None

    def set_rules(self, rules: Rules, path: str):
        path = None if path == "" else path
        self.engine = Engine(rules, self.board_size, path)

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

            pre_bitmap = self.engine.board()
            bitmap = np.array([np.array(xi) for xi in pre_bitmap])
            background = self.get_surface_from_bitmap(bitmap)
            screen.blit(background, (0, 0))

            pygame.display.update()
            self.engine.update()
            clock.tick(self.FPS)

from typing import Text
import pygame
import numpy as np
from scenes import Window
from scenes.components.TextLabel import TextLabel
from rust import Engine, Rules


class Game(Window):
    def __init__(self, window_size, FPS, board_size=None):
        super().__init__(window_size, FPS)
        self.board_size = board_size if board_size is not None else window_size
        self.rules_filename_label = None
        self.board_filename_label = None
        self.engine = None

    def set_rules(self, rules: Rules, path: str):
        path = None if path == "" else path
        self.engine = Engine(rules, self.board_size, path)

    def set_description_labels(self, rules_path, board_path):
        self.rules_filename_label = TextLabel(f"Rules file: {rules_path}")
        self.board_filename_label = TextLabel(f"Board file: {board_path}")

    def get_surface_from_bitmap(self, bitmap):
        scaled_color_bitmap = 255 * bitmap
        bitmap_surface = pygame.surfarray.make_surface(scaled_color_bitmap)
        scaled_bitmap_surface = pygame.transform.scale(
            bitmap_surface, (self.window_size, self.window_size)
        )
        # TODO odbite w pionie, poziomie i obrocone o 180 stopni
        return scaled_bitmap_surface

    def render(self, screen: pygame.Surface, clock: pygame.time.Clock):
        screen.fill((26, 26, 64))
        self.rules_filename_label.draw(screen, TextLabel.margin, TextLabel.margin)
        self.board_filename_label.draw(
            screen,
            TextLabel.margin,
            self.rules_filename_label.get_height()
            + TextLabel.padding
            + TextLabel.margin,
        )

        while True:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    return None

            pre_bitmap = self.engine.board()
            bitmap = np.array([np.array(xi) for xi in pre_bitmap])
            background = self.get_surface_from_bitmap(bitmap)
            # TODO uwzglednic w grze przesuniecie
            screen.blit(background, (100, 100))

            pygame.display.update()
            self.engine.update()
            clock.tick(self.FPS)

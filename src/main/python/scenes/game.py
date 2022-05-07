import pygame
import numpy as np
from scenes import Window
from scenes.components.TextLabel import TextLabel
from rust import Engine, Rules


class Game(Window):
    width_displacement = 150
    height_displacement = 70

    def __init__(self, window_size, FPS, board_size=None):
        super().__init__(window_size, FPS)
        self.board_size = board_size if board_size is not None else window_size
        self.engine = None
        self.preferences = None

    def set_rules(self, rules: Rules, path: str):
        path = None if path == "" else path
        self.engine = Engine(rules, self.board_size, path)

    def colored_text_label(text, flag):
        return TextLabel(
            text, color=TextLabel.highlight_color if flag is True else None
        )

    def set_description_labels(self, rules_path, board_path):
        self.preferences = [
            TextLabel(f"Rules file: {rules_path}"),
            TextLabel(f"Board file: {board_path}"),
            TextLabel(""),
            TextLabel("Rules"),
            # TODO usunac lancuszki
            self.colored_text_label(
                f"C: {self.engine.rules.cell}", self.engine.rules.flags.d_cell
            ),
            self.colored_text_label(
                f"R: {self.engine.rules.range}", self.engine.rules.flags.d_range
            ),
            self.colored_text_label(
                f"S: {self.engine.rules.survival.start} - {self.engine.rules.survival.end}",
                self.engine.rules.flags.d_survival,
            ),
            self.colored_text_label(
                f"B: {self.engine.rules.birth.start} - {self.engine.rules.birth.end}",
                self.engine.rules.flags.d_birth,
            ),
            self.colored_text_label(
                f"N: {str(self.engine.rules.neighbourhood).split('.')[1]}",
                self.engine.rules.flags.d_neighbourhood,
            ),
        ]

    def get_surface_from_bitmap(self, bitmap):
        scaled_color_bitmap = 255 * bitmap
        bitmap_surface = pygame.surfarray.make_surface(scaled_color_bitmap)
        bitmap_size = self.window_size - Game.width_displacement
        scaled_bitmap_surface = pygame.transform.scale(
            bitmap_surface, (bitmap_size, bitmap_size)
        )
        # TODO odbite w pionie, poziomie i obrocone o 180 stopni
        return scaled_bitmap_surface

    def draw_preferences(self, screen):
        height = TextLabel.margin
        text_height = self.preferences[0].get_height()
        for preference in self.preferences:
            preference.draw(screen, TextLabel.margin, height)
            height += TextLabel.padding + text_height

    def render(self, screen: pygame.Surface, clock: pygame.time.Clock):
        screen.fill((26, 26, 64))
        self.draw_preferences(screen)
        while True:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    return None

            pre_bitmap = self.engine.board()
            bitmap = np.array([np.array(xi) for xi in pre_bitmap])
            background = self.get_surface_from_bitmap(bitmap)
            # TODO uwzglednic w grze przesuniecie
            screen.blit(background, (Game.width_displacement, Game.height_displacement))

            pygame.display.update()
            self.engine.update()
            clock.tick(self.FPS)

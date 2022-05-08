import pygame
import numpy as np
from scenes import Window, Scene
from scenes.components import Button, TextLabel, Counter
from rust import Engine, Rules, Flag


class Game(Window):
    width_displacement = 150
    height_displacement = 70

    def __init__(
        self, window_size, FPS, board_size=None, background_color=(255, 255, 255)
    ):
        super().__init__(window_size, FPS, background_color)
        self._board_size = board_size if board_size is not None else window_size
        self._engine = None
        self._preferences = None

        self._return_button = Button(
            text="Return",
            coordinates=(
                Button.margin,
                self.window_size - 120,
            ),
            active_color=pygame.Color("#FA58B6"),
            passive_color=pygame.Color("#7A0BC0"),
            invoke_scene_name=Scene.MENU,
        )

        self._counter = Counter(
            FPS,
            description="FPS",
            coordinates=(self.window_size - 150, Counter.margin),
            active_color=pygame.Color("#FA58B6"),
            passive_color=pygame.Color("#7A0BC0"),
            minimum=1,
            maximum=60,
        )

    def set_rules(self, rules: Rules, path: str):
        path = None if path == "" else path
        self._engine = Engine(rules, self._board_size, path)

    @staticmethod
    def rule_text_label(text, flag):
        return TextLabel(
            text, color=TextLabel.highlight_color if flag is True else None
        )

    @staticmethod
    def file_text_label(default_text, filename, flag):
        if flag is True:
            return TextLabel(default_text, color=TextLabel.highlight_color)
        else:
            return TextLabel(default_text + filename)

    def set_description_labels(self, rules_path, board_path):
        rules = self._engine.rules
        self._preferences = [
            self.file_text_label("Rules file: ", rules_path, rules.get_flag(Flag.RFLoadIncorrect)),
            self.file_text_label(
                "Board file: ", board_path, self._engine.get_flag(Flag.EBFLoadIncorrect)
            ),

            TextLabel(""),
            TextLabel("Rules"),
            self.rule_text_label(f"C: {rules.cell}", rules.get_flag(Flag.RDefaultCell)),
            self.rule_text_label(f"R: {rules.range}", rules.get_flag(Flag.RDefaultRange)),
            self.rule_text_label(
                f"S: {rules.survival.start} - {rules.survival.end}",
                rules.get_flag(Flag.RDefaultSurvival),
            ),
            self.rule_text_label(
                f"B: {rules.birth.start} - {rules.birth.end}",
                rules.get_flag(Flag.RDefaultBirth),
            ),
            self.rule_text_label(
                f"N: {str(rules.neighbourhood).split('.')[1]}",
                rules.get_flag(Flag.RDefaultNeighbourhood),
            ),
        ]

    def get_surface_from_bitmap(self, bitmap):
        bitmap = 255 * (bitmap / bitmap.max())
        grayscale_bitmap = np.empty((*bitmap.shape, 3), dtype=np.uint8)
        grayscale_bitmap[:, :, 2] = grayscale_bitmap[:, :, 1] = grayscale_bitmap[
            :, :, 0
        ] = bitmap
        bitmap_surface = pygame.surfarray.make_surface(grayscale_bitmap)
        bitmap_size = self.window_size - Game.width_displacement
        scaled_bitmap_surface = pygame.transform.scale(
            bitmap_surface, (bitmap_size, bitmap_size)
        )
        # TODO odbite w pionie, poziomie i obrocone o 180 stopni
        return scaled_bitmap_surface

    def draw_preferences(self, screen):
        height = TextLabel.margin
        text_height = self._preferences[0].get_height()
        for preference in self._preferences:
            preference.draw(screen, TextLabel.margin, height)
            height += TextLabel.padding + text_height

    def render(self, screen: pygame.Surface, clock: pygame.time.Clock):
        while True:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    return None
                if event.type == pygame.MOUSEBUTTONDOWN:
                    next_screen = self._return_button.set_status(event.pos)
                    self.FPS = self._counter.set_status(event.pos)
                    if next_screen is not None:
                        return next_screen

            pre_bitmap = self._engine.board()
            bitmap = np.array([np.array(xi) for xi in pre_bitmap])
            background = self.get_surface_from_bitmap(bitmap)

            screen.fill(self.background_color)
            self.draw_preferences(screen)
            self._return_button.draw(screen)
            self._counter.draw(screen)
            screen.blit(background, (Game.width_displacement, Game.height_displacement))

            pygame.display.update()
            self._engine.update()
            clock.tick(self.FPS)

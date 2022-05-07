import pygame
import numpy as np
from scenes import Window, Scene
from scenes.components import Button, TextLabel, Counter
from rust import Engine, Rules


class Game(Window):
    width_displacement = 150
    height_displacement = 70

    def __init__(self, window_size, FPS, board_size=None):
        super().__init__(window_size, FPS)
        self.board_size = board_size if board_size is not None else window_size
        self.engine = None
        self.preferences = None

        self.return_button = Button(
            text="Return",
            coordinates=(
                Button.margin,
                self.window_size - 120,
            ),
            active_color=pygame.Color("#FA58B6"),
            passive_color=pygame.Color("#7A0BC0"),
            invoke_scene_name=Scene.MENU,
        )

        self.counter = Counter(
            FPS,
            coordinates=(self.window_size - 150, Counter.margin),
            active_color=pygame.Color("#FA58B6"),
            passive_color=pygame.Color("#7A0BC0"),
            min=1,
            max=60,
        )

    def set_rules(self, rules: Rules, path: str):
        path = None if path == "" else path
        self.engine = Engine(rules, self.board_size, path)

    def rule_text_label(self, text, flag):
        return TextLabel(
            text, color=TextLabel.highlight_color if flag is True else None
        )

    def file_text_label(self, default_text, filename, flag):
        if flag is True:
            return TextLabel(default_text, color=TextLabel.highlight_color)
        else:
            return TextLabel(default_text + filename)

    def set_description_labels(self, rules_path, board_path):
        rules = self.engine.rules
        self.preferences = [
            self.file_text_label("Rules file: ", rules_path, rules.get_flag("FNF")),
            self.file_text_label(
                "Board file: ", board_path, self.engine.get_flag("FNF")
            ),
            TextLabel(""),
            TextLabel("Rules"),
            self.rule_text_label(f"C: {rules.cell}", rules.get_flag("DC")),
            self.rule_text_label(f"R: {rules.range}", rules.get_flag("DR")),
            self.rule_text_label(
                f"S: {rules.survival.start} - {rules.survival.end}",
                rules.get_flag("DS"),
            ),
            self.rule_text_label(
                f"B: {rules.birth.start} - {rules.birth.end}",
                rules.get_flag("DB"),
            ),
            self.rule_text_label(
                f"N: {str(rules.neighbourhood).split('.')[1]}",
                rules.get_flag("DN"),
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
        while True:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    return None
                if event.type == pygame.MOUSEBUTTONDOWN:
                    next_screen = self.return_button.set_status(event.pos)
                    self.FPS = self.counter.set_status(event.pos)
                    if next_screen is not None:
                        return next_screen

            pre_bitmap = self.engine.board()
            bitmap = np.array([np.array(xi) for xi in pre_bitmap])
            background = self.get_surface_from_bitmap(bitmap)

            screen.fill((26, 26, 64))
            self.draw_preferences(screen)
            self.return_button.draw(screen)
            self.counter.draw(screen)
            screen.blit(background, (Game.width_displacement, Game.height_displacement))

            pygame.display.update()
            self.engine.update()
            clock.tick(self.FPS)

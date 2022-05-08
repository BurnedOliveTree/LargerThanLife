import pygame
from scenes import Scene, Window
from scenes.components import Button, InputTextBox, TextLabel


class Menu(Window):
    def __init__(self, window_size, FPS, background_color=(255, 255, 255)):
        super().__init__(window_size, FPS, background_color)
        self._rules_text_box = InputTextBox(
            coordinates=(self.window_size // 2, self.window_size * 5 // 12),
            active_color=pygame.Color("#FA58B6"),
            passive_color=pygame.Color("#7A0BC0"),
            description="Enter rules  ",
        )

        self._path_text_box = InputTextBox(
            coordinates=(self.window_size // 2, self.window_size * 6 // 12),
            active_color=pygame.Color("#FA58B6"),
            passive_color=pygame.Color("#7A0BC0"),
            description="Enter rules file path (JSON)  ",
        )

        self._board_text_box = InputTextBox(
            coordinates=(self.window_size // 2, self.window_size * 7 // 12),
            active_color=pygame.Color("#FA58B6"),
            passive_color=pygame.Color("#7A0BC0"),
            description="Enter board file path (CSV)  ",
        )

        self._start_game_button = Button(
            text="Start game",
            coordinates=(
                (self.window_size // 2) - 50,
                (self.window_size * 3 // 4) - 25,
            ),
            active_color=pygame.Color("#FA58B6"),
            passive_color=pygame.Color("#7A0BC0"),
            invoke_scene_name=Scene.GAME,
        )

        self._title_label = TextLabel(
            text="Larger than Life",
            color=pygame.Color("#FA58B6"),
            coordinates=(self.window_size // 4, self.window_size * 2 // 12),
            font_size=48,
        )
    
    def get_rules_path(self):
        return self._path_text_box.get_text()

    def get_rules_str(self):
        return self._rules_text_box.get_text()

    def get_board_path(self):
        return self._board_text_box.get_text()
    

    def render(self, screen, clock):
        while True:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    return None
                if event.type == pygame.MOUSEBUTTONDOWN:
                    self._rules_text_box.set_status(event.pos)
                    self._path_text_box.set_status(event.pos)
                    self._board_text_box.set_status(event.pos)
                    game_screen = self._start_game_button.set_status(event.pos)
                    if game_screen is not None:
                        self._start_game_button.set_status((-1, -1))
                        return game_screen
                if event.type == pygame.KEYDOWN:
                    if self._rules_text_box.is_active is True:
                        self._rules_text_box.set_text_after_event(event)
                    elif self._path_text_box.is_active is True:
                        self._path_text_box.set_text_after_event(event)
                    elif self._board_text_box.is_active is True:
                        self._board_text_box.set_text_after_event(event)

            screen.fill(self.background_color)
            self._title_label.draw(screen)
            self._rules_text_box.draw(screen)
            self._path_text_box.draw(screen)
            self._board_text_box.draw(screen)
            self._start_game_button.draw(screen)

            pygame.display.flip()
            clock.tick(self.FPS)

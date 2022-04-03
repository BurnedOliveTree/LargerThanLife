import pygame
from scenes import Scene
from scenes.components import Button, InputTextBox


class Menu:
    def __init__(self, window_size):
        self.window_size = window_size
        self.input_text_box = InputTextBox(
            description="Enter notation params: ",
            coordinates=(self.window_size // 2, self.window_size // 2),
            active_color=pygame.Color("lightblue"),
            passive_color=pygame.Color("blue"),
        )

        self.start_game_button = Button(
            text="Start game",
            invoke_scene_name=Scene.GAME,
            width=100,
            height=50,
            coordinates=(
                (self.window_size // 2) - 50,
                (self.window_size * 3 // 4) - 25,
            ),
            active_color=pygame.Color("lightblue"),
            passive_color=pygame.Color("blue"),
        )

    def draw_title(self, screen):
        font = pygame.font.Font(None, 30)
        text = font.render("Larger than life", True, pygame.Color("white"))
        text_rect = text.get_rect()
        text_rect.center = (self.window_size // 2, self.window_size // 3)
        screen.blit(text, text_rect)

    def render(self, screen, clock, FPS):
        while True:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    return None
                if event.type == pygame.MOUSEBUTTONDOWN:
                    self.input_text_box.set_status(event.pos)
                    game_screen = self.start_game_button.set_status(event.pos)
                    if game_screen is not None:
                        return game_screen
                if (
                    event.type == pygame.KEYDOWN
                    and self.input_text_box.is_active is True
                ):
                    self.input_text_box.get_text_after_event(event)

            screen.fill((0, 0, 0))
            self.draw_title(screen)
            self.input_text_box.draw(screen)
            self.start_game_button.draw(screen)
            pygame.display.flip()
            clock.tick(FPS)

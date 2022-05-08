import pygame
import sys
from scenes import Scene, Menu, Game
from rust import Rules

IMG_SIZE = 600
MENU_FPS = 60
FPS = 1
BOARD_SIZE = 100
BACKGROUND_COLOR = (26, 26, 64)

pygame.init()
clock = pygame.time.Clock()
screen = pygame.display.set_mode((IMG_SIZE, IMG_SIZE))
pygame.mouse.set_visible(True)
pygame.display.set_caption("Larger than Life")

if __name__ == "__main__":
    scene = Scene.MENU
    menu = Menu(IMG_SIZE, MENU_FPS, BACKGROUND_COLOR)
    game = Game(IMG_SIZE, FPS, BOARD_SIZE, BACKGROUND_COLOR)
    while True:
        if scene == Scene.MENU:
            scene = menu.render(screen, clock)
        elif scene == Scene.GAME:
            path_text = menu.path_text_box.get_text()
            input_text = menu.rules_text_box.get_text()
            board_text = menu.board_text_box.get_text()
            if path_text != "":
                rules = Rules.from_file(path_text)
            else:
                rules = Rules.from_str(input_text)
            game.set_rules(rules, board_text)
            game.set_description_labels(path_text, board_text)
            scene = game.render(screen, clock)
        elif scene is None:
            sys.exit()

import pygame
import sys
from scenes import Scene, Menu, Game
from rust import Rules

IMG_SIZE = 600
FPS = 1

pygame.init()
clock = pygame.time.Clock()
screen = pygame.display.set_mode((IMG_SIZE, IMG_SIZE))
pygame.mouse.set_visible(1)
pygame.display.set_caption("Larger than Life")

if __name__ == "__main__":
    scene = Scene.MENU
    menu = Menu(IMG_SIZE, FPS)
    game = Game(IMG_SIZE, FPS)
    while True:
        if scene == Scene.MENU:
            scene = menu.render(screen, clock)
        elif scene == Scene.GAME:
            game.set_rules(Rules.parse(menu.rules_text_box.text, menu.path_text_box.text), menu.board_text_box.text)
            scene = game.render(screen, clock)
        elif scene is None:
            sys.exit()

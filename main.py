import pygame
import sys
from pygame.locals import *
from scenes import Scene, Menu, Game

IMG_SIZE = 600
FPS = 1

pygame.init()
clock = pygame.time.Clock()
screen = pygame.display.set_mode((IMG_SIZE, IMG_SIZE))
pygame.mouse.set_visible(1)
pygame.display.set_caption('Larger than life')

if __name__ == "__main__":
    scene = Scene.MENU
    menu = Menu(IMG_SIZE)
    game = Game(IMG_SIZE)
    while True:
        if scene == Scene.MENU:
            scene = menu.render(screen, clock, FPS)
        elif scene == Scene.GAME:
            scene = game.render(screen, clock, FPS)
        elif scene == None:
            sys.exit()
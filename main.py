import pygame
import sys
import os
from pygame.locals import *
from itertools import cycle

pygame.init()
clock = pygame.time.Clock()
screen = pygame.display.set_mode((600, 480))
pygame.mouse.set_visible(0)
pygame.display.set_caption('Larger than life')
FPS = 1

def load_image(filename):
    return pygame.image.load(os.path.join("./src", filename))
    
bg = [load_image("background.png"), load_image("square.jpg"), load_image("square1.jpg")]
backgrounds = cycle(bg)

while True:
    clock.tick(FPS)
    background = next(backgrounds)
    screen.blit(background, (0, 0))
    pygame.display.update()

    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            sys.exit()
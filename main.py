import pygame
import sys
from pygame.locals import *
import numpy as np

IMG_SIZE = 600
FPS = 1

def get_surface_from_bitmap(bitmap):
    scaled_bitmap = 255*bitmap
    return pygame.surfarray.make_surface(scaled_bitmap)

pygame.init()
clock = pygame.time.Clock()
screen = pygame.display.set_mode((IMG_SIZE, IMG_SIZE))
pygame.mouse.set_visible(0)
pygame.display.set_caption('Larger than life')

while True:
    clock.tick(FPS)
    bitmap = np.round(np.random.random((IMG_SIZE,IMG_SIZE)))
    background = get_surface_from_bitmap(bitmap)
    screen.blit(background, (0, 0))
    pygame.display.update()

    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            sys.exit()
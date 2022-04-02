import pygame
import sys
from pygame.locals import *
import numpy as np
from InputTextBox import InputTextBox

IMG_SIZE = 600
FPS = 1
FONT_SIZE = 30
def get_surface_from_bitmap(bitmap):
    scaled_bitmap = 255*bitmap
    return pygame.surfarray.make_surface(scaled_bitmap)

pygame.init()
clock = pygame.time.Clock()
screen = pygame.display.set_mode((IMG_SIZE, IMG_SIZE))
pygame.mouse.set_visible(1)
pygame.display.set_caption('Larger than life')

def draw_title(screen):
    font = pygame.font.Font(None, 30)
    text = font.render('Larger than life', True, pygame.Color('white'))
    text_rect = text.get_rect()
    text_rect.center = (IMG_SIZE//2, IMG_SIZE//3)
    screen.blit(text, text_rect)

inputTextBox = InputTextBox(
    description="Enter notation params: ", 
    coordinates=(IMG_SIZE//2, IMG_SIZE//2), 
    active_color=pygame.Color('lightblue'), 
    passive_color=pygame.Color('blue'))

while True:
    # clock.tick(FPS)
    # bitmap = np.round(np.random.random((IMG_SIZE,IMG_SIZE)))
    # background = get_surface_from_bitmap(bitmap)
    # screen.blit(background, (0, 0))
    # pygame.display.update()

    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            sys.exit()
        if event.type == pygame.MOUSEBUTTONDOWN:
            inputTextBox.set_status(event.pos)
        if event.type == pygame.KEYDOWN  and inputTextBox.is_active == True:
            inputTextBox.get_text_after_event(event)

    screen.fill((0,0,0))
    draw_title(screen)
    inputTextBox.draw(screen)
    pygame.display.flip()
    clock.tick(60)
        
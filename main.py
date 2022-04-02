import pygame
import sys
from pygame.locals import *
import numpy as np

IMG_SIZE = 600
FPS = 1
FONT_SIZE = 30
INPUT_RECT_HEIGHT = 40
def get_surface_from_bitmap(bitmap):
    scaled_bitmap = 255*bitmap
    return pygame.surfarray.make_surface(scaled_bitmap)

pygame.init()
clock = pygame.time.Clock()
screen = pygame.display.set_mode((IMG_SIZE, IMG_SIZE))
pygame.mouse.set_visible(1)
pygame.display.set_caption('Larger than life')
font = pygame.font.Font(None, 30)

user_text = ''
input_rect = pygame.Rect(IMG_SIZE//2, IMG_SIZE//2-INPUT_RECT_HEIGHT, 150, INPUT_RECT_HEIGHT)
color_active = pygame.Color('lightblue')
color_passive = pygame.Color('blue')
color = color_passive
active = False

def draw_title(screen):
    text = font.render('Larger than life', True, pygame.Color('white'))
    text_rect = text.get_rect()
    text_rect.center = (IMG_SIZE//2, IMG_SIZE//3)
    screen.blit(text, text_rect)

def draw_input(screen):
    if active:
        color = color_active
    else:
        color = color_passive
    pygame.draw.rect(screen, color, input_rect, 5)
    text_surface = font.render(user_text, True, (255, 255, 255))
    screen.blit(text_surface, (input_rect.x + 10, input_rect.y+10))

    input_rect.w = max(text_surface.get_width() + 20, 100)

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
            if input_rect.collidepoint(event.pos):
                active = True
            else:
                active = False
        if event.type == pygame.KEYDOWN  and active == True:
            if event.type == pygame.K_BACKSPACE:
                user_text = user_text[:-1]
            else:
                user_text += event.unicode
    screen.fill((0,0,0))

    draw_title(screen)
    draw_input(screen)

    pygame.display.flip()
    clock.tick(60)
        
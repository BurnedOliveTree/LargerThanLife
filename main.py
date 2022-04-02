import pygame
import sys
from pygame.locals import *
import numpy as np
from enum import Enum
from InputTextBox import InputTextBox
from Button import Button

IMG_SIZE = 600
FPS = 1

class Scene(Enum):
    MENU = 0
    GAME = 1

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

button = Button(
    text="Start game",
    invoke_scene_name=Scene.GAME,
    width=100,
    height=50,
    coordinates=(IMG_SIZE*3//4, IMG_SIZE*3//4),
    active_color=pygame.Color('lightblue'), 
    passive_color=pygame.Color('blue'))


def menu(screen):
    while True:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                sys.exit()
            if event.type == pygame.MOUSEBUTTONDOWN:
                inputTextBox.set_status(event.pos)
                screen = button.set_status(event.pos)
                if screen != None:
                    return screen
            if event.type == pygame.KEYDOWN  and inputTextBox.is_active == True:
                inputTextBox.get_text_after_event(event)

        screen.fill((0,0,0))
        draw_title(screen)
        inputTextBox.draw(screen)
        button.draw(screen)
        pygame.display.flip()
        clock.tick(FPS)

def game(screen):
    while True:
        clock.tick(FPS)
        bitmap = np.round(np.random.random((IMG_SIZE,IMG_SIZE)))
        background = get_surface_from_bitmap(bitmap)
        screen.blit(background, (0, 0))
        pygame.display.update()


if __name__ == "__main__":
    scene = Scene.MENU
    while True:
        if scene == Scene.MENU:
            scene = menu(screen)
        elif scene == Scene.GAME:
            scene = game(screen)
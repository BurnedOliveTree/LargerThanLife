import pygame
import numpy as np

class Game:
    def __init__(self, window_size):
        self.window_size = window_size

    def get_surface_from_bitmap(self, bitmap):
        scaled_bitmap = 255*bitmap
        return pygame.surfarray.make_surface(scaled_bitmap)
        
    def render(self, screen, clock, FPS):
        while True:
            clock.tick(FPS)
            bitmap = np.round(np.random.random((self.window_size, self.window_size)))
            background = self.get_surface_from_bitmap(bitmap)
            screen.blit(background, (0, 0))
            pygame.display.update()
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    return None
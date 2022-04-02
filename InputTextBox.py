import pygame

class InputTextBox:
    def __init__(self, description, coordinates, active_color, passive_color):
        self.input_rect_height = 40
        self.description = description
        self.input_rect = pygame.Rect(coordinates[0], coordinates[1]-self.input_rect_height, 150, self.input_rect_height)
        self.coordinates = coordinates
        self.active_color = active_color
        self.passive_color = passive_color
        self.color = self.passive_color
        self.is_active = False
        self.font = pygame.font.Font(None, 30)
        self.text = ""
    
    def draw_description(self, screen):
        description_surface = self.font.render(self.description, True, pygame.Color('white'))
        description_width = description_surface.get_width()
        screen.blit(description_surface, (self.coordinates[0]-description_width, self.coordinates[1]-(self.input_rect_height*3//4)))
    
    def change_color(self):
        if self.is_active:
            self.color = self.active_color
        else:
            self.color = self.passive_color
        
    def set_status(self, position):
        if self.input_rect.collidepoint(position):
            self.is_active = True
        else:
            self.is_active = False
    
    def draw(self, screen):
        self.draw_description(screen)
        self.change_color()
        pygame.draw.rect(screen, self.color, self.input_rect, 5)
        text_surface = self.font.render(self.text, True, (255, 255, 255))
        screen.blit(text_surface, (self.input_rect.x + 10, self.input_rect.y+10))
        self.input_rect.w = max(text_surface.get_width() + 20, 100)
    
    def get_text_after_event(self, event):
        if event.type == pygame.K_BACKSPACE:
            self.text = self.text[:-1]
        else:
            self.text += event.unicode 

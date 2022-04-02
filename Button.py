import pygame

class Button:
    def __init__(self, text, invoke_scene_name, width, height, coordinates,  active_color, passive_color):
        self.text = text
        self.invoke_scene_name = invoke_scene_name
        self.width = width
        self.height = height
        self.coordinates = coordinates
        self.is_pressed = False
        self.active_color = active_color
        self.passive_color = passive_color
        self.color = self.passive_color
        self.font = pygame.font.Font(None, 30)
        self.text_surface = self.font.render(self.text, True, pygame.Color('white'))
        self.button_rect = pygame.Rect(coordinates[0], coordinates[1], width, height)
        self.button_rect.w = max(self.text_surface.get_width() *3//2, 100)
        
    def draw_description(self, screen):
        description_surface = self.font.render(self.description, True, pygame.Color('white'))
        description_width = description_surface.get_width()
        screen.blit(description_surface, (self.coordinates[0]-description_width, self.coordinates[1]-(self.input_rect_height*3//4)))
    
    def draw(self, screen):
        self.change_color()
        pygame.draw.rect(screen, self.color, self.button_rect, border_radius=10)
        screen.blit(self.text_surface, (self.coordinates[0]+self.button_rect.w//4, self.coordinates[1]))
    
    def set_status(self, position):
        if self.button_rect.collidepoint(position):
            self.is_pressed = True
            return self.invoke_scene_name
        else:
            self.is_pressed = False
            return None

    def change_color(self):
        if self.is_pressed:
            self.color = self.active_color
        else:
            self.color = self.passive_color
        
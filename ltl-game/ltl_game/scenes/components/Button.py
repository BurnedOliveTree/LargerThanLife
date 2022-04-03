from scenes.components.Component import Component

class Button(Component):
    def __init__(
        self,
        text,
        width,
        height,
        coordinates,
        active_color,
        passive_color,
        invoke_scene_name
    ):
        super().__init__(text, width, height, coordinates, active_color, passive_color)
        self.invoke_scene_name = invoke_scene_name

    def set_status(self, position):
        if self.rect.collidepoint(position):
            self.is_active = True
            return self.invoke_scene_name
        else:
            self.is_active = False
            return None 

from scenes.components.Component import Component


class Button(Component):
    margin = 15

    def __init__(
        self,
        text,
        coordinates,
        active_color,
        passive_color,
        invoke_scene_name=None,
    ):
        super().__init__(text, coordinates, active_color, passive_color, None)
        self._invoke_scene_name = invoke_scene_name

    def set_status(self, position):
        if self._rect.collidepoint(position):
            self.is_active = True
            return self._invoke_scene_name
        else:
            self.is_active = False
            return None

    def get_collidepoint(self, position):
        return self._rect.collidepoint(position)

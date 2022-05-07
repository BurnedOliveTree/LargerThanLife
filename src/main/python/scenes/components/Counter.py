from scenes.components.Button import Button
from scenes.components.TextLabel import TextLabel


class Counter:
    margin = 15

    def __init__(self, value, coordinates, active_color, passive_color, min, max):
        self.value = value
        self.min = min
        self.max = max
        self.minus_button = Button(
            text="-",
            coordinates=(coordinates[0], coordinates[1]),
            active_color=active_color,
            passive_color=passive_color,
        )
        self.value_label = TextLabel(
            f"{value} FPS", coordinates=(coordinates[0] + 25, coordinates[1])
        )

        self.plus_button = Button(
            text="+",
            coordinates=(coordinates[0] + 100, coordinates[1]),
            active_color=active_color,
            passive_color=passive_color,
        )

    def set_status(self, position):
        if self.minus_button.rect.collidepoint(position):
            self.decrease_value()
        elif self.plus_button.rect.collidepoint(position):
            self.increase_value()
        return self.value

    def increase_value(self):
        if self.value < max:
            self.value += 1
            self.value_label.update_text(f"{self.value} FPS")

    def decrease_value(self):
        if self.value > min:
            self.value -= 1
            self.value_label.update_text(f"{self.value} FPS")

    def draw(self, screen):
        self.minus_button.draw(screen)
        self.value_label.draw(screen)
        self.plus_button.draw(screen)

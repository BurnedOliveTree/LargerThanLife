from scenes.components.Button import Button
from scenes.components.TextLabel import TextLabel


class Counter:
    margin = 15

    def __init__(
        self,
        value,
        description,
        coordinates,
        active_color,
        passive_color,
        minimum,
        maximum,
    ):
        self._value = value
        self._description = description
        self._minimum = minimum
        self._maximum = maximum
        self._minus_button = Button(
            text="-",
            coordinates=(coordinates[0], coordinates[1]),
            active_color=active_color,
            passive_color=passive_color,
        )
        self._value_label = TextLabel(
            f"{value} FPS", coordinates=(coordinates[0] + 25, coordinates[1])
        )

        self._plus_button = Button(
            text="+",
            coordinates=(coordinates[0] + 100, coordinates[1]),
            active_color=active_color,
            passive_color=passive_color,
        )

    def set_status(self, position):
        if self._minus_button.get_collidepoint(position):
            self.decrease_value()
        elif self._plus_button.get_collidepoint(position):
            self.increase_value()
        return self._value

    def increase_value(self):
        if self._value < self._maximum:
            self._value += 1
            self._value_label.update_text(f"{self._value} {self._description}")

    def decrease_value(self):
        if self._value > self._minimum:
            self._value -= 1
            self._value_label.update_text(f"{self._value} {self._description}")

    def draw(self, screen):
        self._minus_button.draw(screen)
        self._value_label.draw(screen)
        self._plus_button.draw(screen)

import struct
import unittest


class Value:
    def __init__(self, value: float):
        self.value = value

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Value):
            return NotImplemented
        return self.value == other.value

    def __add__(self, other: object) -> "Value":
        if not isinstance(other, Value):
            return NotImplemented
        return Value(self.value + other.value)

    def __sub__(self, other: object) -> "Value":
        if not isinstance(other, Value):
            return NotImplemented
        return Value(self.value - other.value)

    def __mul__(self, other: object) -> "Value":
        if not isinstance(other, Value):
            return NotImplemented
        return Value(self.value * other.value)

    def __truediv__(self, other: object) -> "Value":
        if not isinstance(other, Value):
            return NotImplemented
        return Value(self.value / other.value)

    def serialize(self) -> bytes:
        return struct.pack("f", self.value)

    def disassemble(self) -> str:
        return str(self.value)

import unittest

from value import *


class ValueTests(unittest.TestCase):
    def test_equality(self):
        self.assertEqual(Value(5.0), Value(5.0))
        self.assertNotEqual(Value(5.0), Value(6.0))

    def test_add(self):
        self.assertEqual(Value(10.0), Value(5.0) + Value(5.0))

    def test_subtract(self):
        self.assertEqual(Value(2.0), Value(7.0) - Value(5.0))

    def test_multiply(self):
        self.assertEqual(Value(25.0), Value(5.0) * Value(5.0))

    def test_divide(self):
        self.assertEqual(Value(2.0), Value(10.0) / Value(5.0))

    def test_serialize_size(self):
        self.assertEqual(4, len(Value(5.0).serialize()))

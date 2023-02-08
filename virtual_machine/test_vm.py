import unittest

from vm import *


class ResultTests(unittest.TestCase):
    def test_equality_status(self):
        lhs = Result(Status.OK)
        rhs = Result(Status.NOT_IMPLEMENTED)
        self.assertNotEqual(lhs, rhs)
        rhs.status = Status.OK
        self.assertEqual(lhs, rhs)

    def test_equality_value(self):
        lhs = Result(Status.OK, value=Value(5.0))
        rhs = Result(Status.OK)
        self.assertNotEqual(lhs, rhs)
        rhs.value = Value(5.0)
        self.assertEqual(lhs, rhs)


class ContextTests(unittest.TestCase):
    def test_pop_no_pushed_values(self):
        context = Context()
        self.assertEqual(None, context.pop())

    def test_push_pop(self):
        context = Context()
        context.push(5)
        self.assertEqual(5, context.pop())
        self.assertEqual(None, context.pop())


class VirtualMachineTests(unittest.TestCase):
    def test_negate(self):
        code = ByteCode()
        code.add_instruction(Immediate(Value(5.0)), 1)
        code.add_instruction(Negate(), 1)
        code.add_instruction(Return(), 1)
        logging.debug(code.disassemble())
        vm = VirtualMachine()
        expected = Result(status=Status.OK, value=Value(-5.0))
        actual = vm.execute(code.serialize())
        self.assertEqual(expected.status, actual.status)
        self.assertEqual(expected.value.value, actual.value.value)
        self.assertEqual(expected, actual)

    def test_add(self):
        code = ByteCode()
        code.add_instruction(Immediate(Value(5.0)), 1)
        code.add_instruction(Immediate(Value(5.0)), 1)
        code.add_instruction(Add(), 1)
        code.add_instruction(Return(), 1)
        logging.debug(code.disassemble())
        vm = VirtualMachine()
        expected = Result(status=Status.OK, value=Value(10.0))
        actual = vm.execute(code.serialize())
        self.assertEqual(expected.status, actual.status)
        self.assertEqual(expected.value.value, actual.value.value)
        self.assertEqual(expected, actual)

    def test_subtract(self):
        code = ByteCode()
        code.add_instruction(Immediate(Value(5.0)), 1)
        code.add_instruction(Immediate(Value(3.0)), 1)
        code.add_instruction(Subtract(), 1)
        code.add_instruction(Return(), 1)
        logging.debug(code.disassemble())
        vm = VirtualMachine()
        expected = Result(status=Status.OK, value=Value(2.0))
        actual = vm.execute(code.serialize())
        self.assertEqual(expected.status, actual.status)
        self.assertEqual(expected.value.value, actual.value.value)
        self.assertEqual(expected, actual)

    def test_multiply(self):
        code = ByteCode()
        code.add_instruction(Immediate(Value(5.0)), 1)
        code.add_instruction(Immediate(Value(5.0)), 1)
        code.add_instruction(Multiply(), 1)
        code.add_instruction(Return(), 1)
        logging.debug(code.disassemble())
        vm = VirtualMachine()
        expected = Result(status=Status.OK, value=Value(25.0))
        actual = vm.execute(code.serialize())
        self.assertEqual(expected.status, actual.status)
        self.assertEqual(expected.value.value, actual.value.value)
        self.assertEqual(expected, actual)

    def test_divide(self):
        code = ByteCode()
        code.add_instruction(Immediate(Value(5.0)), 1)
        code.add_instruction(Immediate(Value(2.0)), 1)
        code.add_instruction(Divide(), 1)
        code.add_instruction(Return(), 1)
        logging.debug(code.disassemble())
        vm = VirtualMachine()
        expected = Result(status=Status.OK, value=Value(2.5))
        actual = vm.execute(code.serialize())
        self.assertEqual(expected.status, actual.status)
        self.assertEqual(expected.value.value, actual.value.value)
        self.assertEqual(expected, actual)

    def test_expression(self):
        # -((1.2 + 3.4) / 5.6)
        code = ByteCode()
        code.add_instruction(Immediate(Value(1.2)), 1)
        code.add_instruction(Immediate(Value(3.4)), 1)
        code.add_instruction(Add(), 1)

        code.add_instruction(Immediate(Value(5.6)), 1)
        code.add_instruction(Divide(), 1)

        code.add_instruction(Negate(), 1)
        code.add_instruction(Return(), 1)

        vm = VirtualMachine()
        expected = Result(status=Status.OK, value=Value(-(1.2 + 3.4) / 5.6))
        actual = vm.execute(code.serialize())
        # floating point math assertions, lol
        self.assertAlmostEqual(expected.value.value, actual.value.value)

    def test_no_return_is_runtime_error(self):
        code = ByteCode()
        code.add_instruction(Immediate(Value(5.0)), 1)
        vm = VirtualMachine()
        expected = Result(status=Status.RUNTIME_ERROR)
        actual = vm.execute(code.serialize())
        self.assertEqual(expected.status, actual.status)
        self.assertEqual(expected, actual)


if __name__ == "__main__":
    unittest.main()

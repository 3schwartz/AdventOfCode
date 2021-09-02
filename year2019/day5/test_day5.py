import unittest

from year2019.day5.day5_func import *


class TestDay5(unittest.TestCase):

    def test_opcode_three(self):
        # Arrange
        input_sequence = ['3', '2', '5']
        input = '1'
        expected = ['3', '2', '1']

        # Act
        opcode_three_at_input_at_position(input_sequence, int(input_sequence[1]), input)

        # Assert
        self.assertEquals(expected, input_sequence)

    def test_prefix_zeros(self):
        # Arrange
        opcode = '2'
        expected = '00002'

        # Act
        actual = add_zero(opcode, 5)

        # Assert
        self.assertEquals(expected, actual)

    def test_get_opcode(self):
        # Arrange
        opcode_string = '00002'

        # Act
        opcode, opcode_list = get_opcode(opcode_string)

        self.assertEquals(opcode, '02')
        self.assertEquals(['0', '0', '0', '0', '2'], opcode_list)

    def test_get_action_zero(self):
        # Arrange
        opcode_list = ['0', '0', '0', '0', '2']
        index = 0
        instructions = ['0', '1', '3', '5']

        # Act
        action = get_position(opcode_list[0], index, instructions)

        # Assert
        self.assertEquals(5, action)

    def test_get_action_non_zero(self):
        # Arrange
        opcode_list = ['1', '0', '0', '0', '2']
        index = 0
        instructions = ['0', '1', '3', '5']

        # Act
        action = get_position(opcode_list[0], index, instructions)

        # Assert
        self.assertEquals(3, action)

    def test_get_actions(self):
        # Arrange
        opcode_list = ['1', '0', '1', '0', '2']
        index = 0
        instructions = ['0', '1', '3', '5']

        # Act
        actions = get_actions(opcode_list[-3:0:-1], index, instructions)

        # Assert
        self.assertEquals(1, actions[0])
        self.assertEquals(5, actions[1])







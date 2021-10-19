import unittest


class TestDay4(unittest.TestCase):

    def test_correct_increasing(self):
        # Arrange
        sequence = "1234"
        expected = [True, True, True]

        # Act
        actual = rule_increasing(sequence)

        # Assert
        self.assertEqual(expected, actual)

    def test_any_equal(self):
        # Arrange
        sequence = "1123"
        expected = [True, False, False]

        # Act
        actual = rule_two_sequentially_increasing(sequence)

        # Assert
        self.assertEqual(expected, actual)

    def test_pass_two_first_rules(self):
        # Arrange
        sequence = 1123
        expected = True
        rules = get_rules()[:2]

        # Act
        actual = sequence_pass_rules(sequence, rules)

        # Assert
        self.assertEqual(expected, actual)

    def test_does_not_pass_two_first_rules(self):
        # Arrange
        sequence = 1423
        expected = False
        rules = get_rules()[:2]

        # Act
        actual = sequence_pass_rules(sequence, rules)

        # Assert
        self.assertEqual(expected, actual)

    def test_group_by_counts_(self):
        # Arrange
        sequence = "1123"
        expected = [True, False, False]

        # Act
        actual = rule_count_groups_of_two(sequence)

        # Assert
        self.assertEqual(expected, actual)
from unittest import TestCase

import calories_parser


class TestCaloriesParser(TestCase):
    def test_parse_file(self):
        expected = [
            [1000, 2000, 3000],
            [4000],
            [5000, 6000],
            [7000, 8000, 9000],
            [10000],
        ]

        result = calories_parser.parse_file("test_input.txt")
        self.assertEqual(expected, result)

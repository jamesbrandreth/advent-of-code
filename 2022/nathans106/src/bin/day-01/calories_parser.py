from pathlib import Path
from typing import List

import pyparsing as pp


def parse_file(file_path: str) -> List[List[int]]:
    path = Path(file_path)
    return parse_text(path.read_text())


def parse_text(text: str) -> List[List[int]]:
    pp.ParserElement.setDefaultWhitespaceChars(" \t")
    calorie_parser = pp.OneOrMore(
        pp.Group(
            pp.delimited_list(pp.Word(pp.nums), "\n") + pp.Optional("\n\n").suppress()
        )
    )
    calories = calorie_parser.parse_string(text)
    return [[int(num) for num in elf] for elf in calories.as_list()]

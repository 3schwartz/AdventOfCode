from __future__ import annotations
from itertools import groupby
from abc import ABC, abstractmethod
from typing import List


class PasswordFinder:
    def __init__(self, start: int, end: int) -> None:
        self.start = start
        self.end = end

    def getPasswords(self):
        return [s for s in range(self.start, self.end + 1)]


class Validation(ABC):

    @abstractmethod
    def validate(self, password: str) -> bool:
        pass


class IncreasingValidation(Validation):

    def validate(self, password: str) -> bool:
        return all([a <= b for a, b in zip(password, password[1:])])


class TwoSequentuallyEqual(Validation):

    def validate(self, password: str) -> bool:
        return any([a == b for a, b in zip(password, password[1:])])


class TwoEqual(Validation):

    def validate(self, password: str) -> bool:
        return any([len(list(k)) == 2 for _, k in groupby(password)])


class PasswordValidator:

    def __init__(self):
        self._validations = None

    @property
    def validations(self):
        return self._validations

    @validations.setter
    def validations(self, validationsInput: List[Validation]) -> None:
        self._validations = validationsInput

    def validatePassword(self, password: int):
        return all(validation.validate(str(password)) for validation in self.validations)

    def numberValidPassword(self, passwords: List[int]):
        return sum([self.validatePassword(password) for password in passwords])

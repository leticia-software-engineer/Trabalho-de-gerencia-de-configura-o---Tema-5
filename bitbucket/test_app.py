"""
Testes unitários simples com pytest.
Esses são os testes que cada pipeline (Jenkins, GitLab, GitHub Actions,
Bitbucket, AWS CodePipeline) vai executar na etapa de "teste".
"""

import pytest
from app import somar, subtrair, multiplicar, dividir


def test_somar():
    assert somar(2, 3) == 5
    assert somar(-1, 1) == 0


def test_subtrair():
    assert subtrair(10, 4) == 6
    assert subtrair(0, 5) == -5


def test_multiplicar():
    assert multiplicar(3, 4) == 12
    assert multiplicar(-2, 3) == -6


def test_dividir():
    assert dividir(10, 2) == 5
    assert dividir(7, 2) == 3.5


def test_dividir_por_zero_lanca_erro():
    with pytest.raises(ValueError):
        dividir(10, 0)

"""
Aplicação de exemplo: uma calculadora simples.
Serve como base para demonstrar testes automatizados
rodando dentro de pipelines de CI/CD.
"""


def somar(a, b):
    return a + b


def subtrair(a, b):
    return a - b


def multiplicar(a, b):
    return a * b


def dividir(a, b):
    if b == 0:
        raise ValueError("Não é possível dividir por zero")
    return a / b

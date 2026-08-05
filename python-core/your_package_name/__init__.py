from .rust_core import hello_from_bin

def feat1():
    print("feat1")

def main() -> None:
    print(hello_from_bin())

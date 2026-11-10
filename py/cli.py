import sys
import base64
import re


def slugify(s: str) -> str:
    s = s.lower()
    s = re.sub(r"[^a-z0-9]+", "-", s)
    return s.strip("-")


def main() -> None:
    if len(sys.argv) < 2:
        print("usage: py cli.py <slugify|b64encode|b64decode> [input]")
        sys.exit(1)
    cmd = sys.argv[1]
    data = " ".join(sys.argv[2:])
    if cmd == "slugify":
        print(slugify(data))
    elif cmd == "b64encode":
        print(base64.b64encode(data.encode()).decode())
    elif cmd == "b64decode":
        print(base64.b64decode(data).decode())
    else:
        print(f"unknown command: {cmd}")
        sys.exit(1)


if __name__ == "__main__":
    main()
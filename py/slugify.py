def slugify(s):
    return "-".join(filter(None, "".join(c.lower() if c.isalnum() else " " for c in s).split()))


def main():
    import sys
    if len(sys.argv) < 3:
        print("usage: slugify.py <input>")
        sys.exit(1)
    print(slugify(" ".join(sys.argv[2:])))


if __name__ == "__main__":
    main()
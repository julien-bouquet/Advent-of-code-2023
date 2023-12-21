
def main():
    with open("../input.txt") as reader:
        result = 0
        for line in reader.readlines():
            first = last = None
            for char in line:
                if char.isdigit():
                    if first is None:
                        first = last = char
                    else:
                        last = char
            result = result + int(first +last)
    print(result)

    
if __name__ == "__main__":
    main()

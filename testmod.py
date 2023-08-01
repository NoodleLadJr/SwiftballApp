def print_info():
    print("Hello From Python!")
    with open("dummy.txt", "w", encoding="UTF-8") as f:
        f.write("Hello World!")
        f.close()
